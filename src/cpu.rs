use std::{
    sync::atomic::{AtomicU64, AtomicU8, Ordering},
    time::Instant,
};

use rand::RngCore;
use rayon::prelude::*;
use regex::Regex;

pub fn start_cpu(stats: bool, regexes: Vec<Regex>) {
    let generated = AtomicU64::new(0);
    let start_now = Instant::now();

    let max_leading_zeros: Vec<AtomicU8> = (0..(regexes.len())).map(|_| AtomicU8::new(0)).collect();

    std::iter::repeat(()).par_bridge().for_each(|_| {
        let (seed, pk) = gen_kp();
        crate::handler::handle_keypair(&seed, &pk, &regexes, &max_leading_zeros);
        if generated.fetch_add(1, Ordering::Relaxed) % 1000000 == 0 {
            if stats {
                let time_elapsed = (std::time::Instant::now() - start_now).as_secs_f64();
                let hashrate =
                    generated.load(Ordering::Relaxed) as f64 / time_elapsed / 1_000_000.0;
                println!("Hashrate: {:.3} MH/s", hashrate);
            }
        }
    });
}

fn gen_kp() -> (crate::Seed, crate::PublicKey) {
    let mut seed = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut seed);

    let kp = ed25519_dalek::SigningKey::from_bytes(&seed);
    let binding = kp.verifying_key();
    let public = binding.as_bytes();

    (seed, *public)
}
