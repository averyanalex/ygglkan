#![feature(slice_flatten)]

use std::sync::atomic::{AtomicU32, Ordering};

use clap::Parser;
use hex::ToHex;
use rand::RngCore;
use rayon::prelude::*;
use std::io::Write;

const WORKGROUP_SIZE: usize = 64;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Block size. Each block has 64 keys.
    #[arg(short, long, default_value_t = 1024)]
    batch_size: usize,

    /// Print hashrate stats
    #[arg(short, long, default_value_t = false)]
    print_stats: bool,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    let shader_binary = wgpu::include_spirv_raw!(env!("kernel.spv"));

    println!("Starting miner...");
    println!(
        "Batch size: {}, keys in batch: {}",
        args.batch_size,
        args.batch_size * 64
    );
    println!("This may take a while due to shader compilation.");

    futures::executor::block_on(start_internal(
        shader_binary,
        args.batch_size * 64,
        args.print_stats,
    ));
}

pub async fn start_internal(
    shader_binary: wgpu::ShaderModuleDescriptorSpirV<'static>,
    batch_size: usize,
    print_stats: bool,
) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        ..Default::default()
    });
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("Failed to find an appropriate adapter");

    let mut limits = wgpu::Limits::default();
    limits.max_storage_buffer_binding_size = (batch_size * 32) as u32;

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::SPIRV_SHADER_PASSTHROUGH,
                limits: limits,
            },
            None,
        )
        .await
        .expect("Failed to create device");

    drop(instance);
    drop(adapter);

    let shader_module = unsafe { device.create_shader_module_spirv(&shader_binary) };

    let readback_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: (batch_size * 32) as wgpu::BufferAddress,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: (batch_size * 32) as wgpu::BufferAddress,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            count: None,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                has_dynamic_offset: false,
                min_binding_size: None,
                ty: wgpu::BufferBindingType::Storage { read_only: false },
            },
        }],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage_buffer.as_entire_binding(),
        }],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        module: &shader_module,
        entry_point: "main",
    });

    let mut pubkeys = vec![[0u8; 32]; batch_size];
    let mut new_seeds = vec![[0u8; 32]; batch_size];
    let mut seeds = vec![[0u8; 32]; batch_size];

    let max_leading_zeros = AtomicU32::new(20);

    rand::thread_rng().fill_bytes(&mut seeds.flatten_mut());
    rand::thread_rng().fill_bytes(&mut new_seeds.flatten_mut());
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.set_pipeline(&compute_pipeline);
        cpass.dispatch_workgroups((batch_size / WORKGROUP_SIZE) as u32, 1, 1);
    }
    encoder.copy_buffer_to_buffer(
        &storage_buffer,
        0,
        &readback_buffer,
        0,
        (batch_size * 32) as wgpu::BufferAddress,
    );
    queue.write_buffer(&storage_buffer, 0, seeds.flatten());
    queue.submit(Some(encoder.finish()));
    let pubkeys_slice = readback_buffer.slice(..);
    pubkeys_slice.map_async(wgpu::MapMode::Read, |r| r.unwrap());
    device.poll(wgpu::Maintain::Wait);
    let pubkeys_range = pubkeys_slice.get_mapped_range();
    pubkeys.flatten_mut().copy_from_slice(&pubkeys_range);
    drop(pubkeys_range);
    readback_buffer.unmap();

    loop {
        let start_now = std::time::Instant::now();

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_bind_group(0, &bind_group, &[]);
            cpass.set_pipeline(&compute_pipeline);
            cpass.dispatch_workgroups((batch_size / WORKGROUP_SIZE) as u32, 1, 1);
        }
        encoder.copy_buffer_to_buffer(
            &storage_buffer,
            0,
            &readback_buffer,
            0,
            (batch_size * 32) as wgpu::BufferAddress,
        );
        queue.write_buffer(&storage_buffer, 0, new_seeds.flatten());
        queue.submit(Some(encoder.finish()));

        pubkeys
            .par_iter()
            .zip(seeds.par_iter())
            .for_each(|(pk, seed)| {
                let mut first_bytes = [0u8; 8];
                first_bytes.copy_from_slice(&pk[..8]);
                let first_number = u64::from_be_bytes(first_bytes);

                let leading_zeros = first_number.leading_zeros();
                if max_leading_zeros.fetch_max(leading_zeros, Ordering::AcqRel) <= leading_zeros {
                    let mut sk = [0u8; 64];
                    sk[..32].copy_from_slice(seed);
                    sk[32..].copy_from_slice(pk);
                    let mut lock = std::io::stdout().lock();
                    writeln!(lock, "=======================================").unwrap();
                    writeln!(lock, "PrivateKey: {:02X?}", sk.encode_hex::<String>()).unwrap();
                    writeln!(lock, "PublicKey: {:02X?}", pk.encode_hex::<String>()).unwrap();
                    writeln!(lock, "Height: {}", leading_zeros).unwrap();
                    writeln!(lock, "=======================================").unwrap();
                };
            });

        seeds.flatten_mut().copy_from_slice(new_seeds.flatten());
        rand::thread_rng().fill_bytes(&mut new_seeds.flatten_mut());

        let pubkeys_slice = readback_buffer.slice(..);
        pubkeys_slice.map_async(wgpu::MapMode::Read, |r| r.unwrap());
        device.poll(wgpu::Maintain::Wait);
        let pubkeys_range = pubkeys_slice.get_mapped_range();
        pubkeys.flatten_mut().copy_from_slice(&pubkeys_range);
        drop(pubkeys_range);
        readback_buffer.unmap();

        if print_stats {
            let time_elapsed = (std::time::Instant::now() - start_now).as_secs_f64();
            let hashrate = batch_size as f64 / time_elapsed / 1_000_000.0;
            println!("Hashrate: {:.2} MH/s", hashrate);
        }
    }
}
