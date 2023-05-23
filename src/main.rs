#![feature(slice_flatten)]

use clap::Parser;
use regex::Regex;
use strum_macros::{Display, EnumString};

const WORKGROUP_SIZE: usize = 64;

mod cpu;
mod gpu;
mod handler;

type PublicKey = [u8; 32];
type Seed = [u8; 32];

#[derive(Debug, Clone, EnumString, Display)]
enum Backend {
    CPU,
    GPU,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// GPU block size. Each block has 64 keys
    #[arg(short, long, default_value_t = 1024)]
    batch_size: usize,

    /// Print hashrate stats
    #[arg(short, long, default_value_t = false)]
    stats: bool,

    /// Regex pattern to search
    #[arg(short, long)]
    regexes: Option<Vec<String>>,

    /// Which backend to use (CPU/GPU)
    #[arg(long, default_value_t = Backend::GPU)]
    backend: Backend,
}
fn main() {
    env_logger::init();

    let args = Args::parse();

    let regexes = match args.regexes {
        Some(r) => r,
        None => vec![String::from("")],
    };
    let regexes: Vec<_> = regexes
        .into_iter()
        .map(|r| Regex::new(&r).unwrap())
        .collect();

    println!("Starting miner...");
    println!(
        "Batch size: {}, keys in batch: {}",
        args.batch_size,
        args.batch_size * 64
    );
    println!("This may take a while due to shader compilation.");

    match args.backend {
        Backend::GPU => gpu::start_gpu(args.batch_size, args.stats, regexes),
        Backend::CPU => cpu::start_cpu(args.stats, regexes),
    }
}
