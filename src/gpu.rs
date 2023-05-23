use std::sync::atomic::AtomicU8;

use rand::RngCore;
use rayon::prelude::*;
use regex::Regex;
use wgpu::{BindGroup, Buffer, ComputePipeline, Device, Queue};

use crate::{PublicKey, Seed};

pub fn start_gpu(batch_size: usize, stats: bool, regexes: Vec<Regex>) {
    let shader_binary = wgpu::include_spirv_raw!(env!("kernel.spv"));

    futures::executor::block_on(start_internal(
        shader_binary,
        batch_size * 64,
        stats,
        regexes,
    ));
}

async fn start_internal(
    shader_binary: wgpu::ShaderModuleDescriptorSpirV<'static>,
    batch_size: usize,
    print_stats: bool,
    regexes: Vec<Regex>,
) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        ..Default::default()
    });

    let adapter_options = wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        ..Default::default()
    };

    let adapter = instance
        .request_adapter(&adapter_options)
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

    let mut pubkeys: Vec<PublicKey> = vec![[0xFFu8; 32]; batch_size];
    let mut new_seeds: Vec<Seed> = vec![[0u8; 32]; batch_size];
    let mut current_seeds: Vec<Seed> = vec![[0u8; 32]; batch_size];

    let max_leading_zeros: Vec<AtomicU8> = (0..(regexes.len())).map(|_| AtomicU8::new(0)).collect();
    let mut first_run = true;

    rand::thread_rng().fill_bytes(&mut new_seeds.flatten_mut());
    loop {
        let start_now = std::time::Instant::now();

        start_compute_pass(
            &device,
            &bind_group,
            &compute_pipeline,
            batch_size,
            &storage_buffer,
            &readback_buffer,
            &queue,
            new_seeds.flatten(),
        );

        if !first_run {
            handle_keypairs(&current_seeds, &pubkeys, &regexes, &max_leading_zeros);
        } else {
            first_run = false;
        }
        std::mem::swap(&mut current_seeds, &mut new_seeds);
        rand::thread_rng().fill_bytes(&mut new_seeds.flatten_mut());

        read_pubkeys(&device, &readback_buffer, pubkeys.flatten_mut());

        if print_stats {
            let time_elapsed = (std::time::Instant::now() - start_now).as_secs_f64();
            let hashrate = batch_size as f64 / time_elapsed / 1_000_000.0;
            println!("Hashrate: {:.2} MH/s", hashrate);
        }
    }
}

fn start_compute_pass(
    device: &Device,
    bind_group: &BindGroup,
    compute_pipeline: &ComputePipeline,
    batch_size: usize,
    storage_buffer: &Buffer,
    readback_buffer: &Buffer,
    queue: &Queue,
    seeds: &[u8],
) {
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.set_pipeline(&compute_pipeline);
        cpass.dispatch_workgroups((batch_size / crate::WORKGROUP_SIZE) as u32, 1, 1);
    }
    encoder.copy_buffer_to_buffer(
        &storage_buffer,
        0,
        &readback_buffer,
        0,
        (batch_size * 32) as wgpu::BufferAddress,
    );
    queue.write_buffer(&storage_buffer, 0, seeds);
    queue.submit(Some(encoder.finish()));
}

fn read_pubkeys(device: &Device, readback_buffer: &Buffer, pubkeys: &mut [u8]) {
    let pubkeys_slice = readback_buffer.slice(..);
    pubkeys_slice.map_async(wgpu::MapMode::Read, |r| r.unwrap());
    device.poll(wgpu::Maintain::Wait);
    let pubkeys_range = pubkeys_slice.get_mapped_range();
    pubkeys.copy_from_slice(&pubkeys_range);
    drop(pubkeys_range);
    readback_buffer.unmap();
}

fn handle_keypairs(
    seeds: &Vec<Seed>,
    pubkeys: &Vec<PublicKey>,
    regexes: &Vec<Regex>,
    max_leading_zeros: &Vec<AtomicU8>,
) {
    pubkeys
        .par_iter()
        .zip(seeds.par_iter())
        .for_each(|(pk, seed)| {
            crate::handler::handle_keypair(seed, pk, regexes, max_leading_zeros)
        });
}
