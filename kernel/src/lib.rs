#![cfg_attr(target_arch = "spirv", no_std)]

use glam::UVec3;
use spirv_std::{glam, spirv};

mod bigint;
mod edwards25519;
mod field25519;
mod sha512;

fn split(bytes: &[u8; 64]) -> [u8; 32] {
    let mut scalar = [0u8; 32];
    for i in 00..32 {
        scalar[i] = bytes[i];
    }

    scalar[0] &= 248;
    scalar[31] &= 63;
    scalar[31] |= 64;

    scalar
}

type Seed = [u8; 32];
type PublicKey = [u8; 32];

fn public_from_seed(seed: Seed) -> PublicKey {
    let scalar = {
        let hash_output = sha512::hash(&seed);
        split(&hash_output)
    };
    let pk = edwards25519::ge_scalarmult_base(&scalar).to_bytes();
    pk
}

#[spirv(compute(threads(64)))]
pub fn main(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] keys: &mut [[u8; 32]],
) {
    let index = id.x as usize;
    keys[index] = public_from_seed(keys[index]);
}
