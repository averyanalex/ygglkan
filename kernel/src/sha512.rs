use crunchy::unroll;

use crate::bigint::rotr;

const ROUND_CONSTANTS: [u64; 80] = [
    0x428a2f98d728ae22,
    0x7137449123ef65cd,
    0xb5c0fbcfec4d3b2f,
    0xe9b5dba58189dbbc,
    0x3956c25bf348b538,
    0x59f111f1b605d019,
    0x923f82a4af194f9b,
    0xab1c5ed5da6d8118,
    0xd807aa98a3030242,
    0x12835b0145706fbe,
    0x243185be4ee4b28c,
    0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f,
    0x80deb1fe3b1696b1,
    0x9bdc06a725c71235,
    0xc19bf174cf692694,
    0xe49b69c19ef14ad2,
    0xefbe4786384f25e3,
    0x0fc19dc68b8cd5b5,
    0x240ca1cc77ac9c65,
    0x2de92c6f592b0275,
    0x4a7484aa6ea6e483,
    0x5cb0a9dcbd41fbd4,
    0x76f988da831153b5,
    0x983e5152ee66dfab,
    0xa831c66d2db43210,
    0xb00327c898fb213f,
    0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2,
    0xd5a79147930aa725,
    0x06ca6351e003826f,
    0x142929670a0e6e70,
    0x27b70a8546d22ffc,
    0x2e1b21385c26c926,
    0x4d2c6dfc5ac42aed,
    0x53380d139d95b3df,
    0x650a73548baf63de,
    0x766a0abb3c77b2a8,
    0x81c2c92e47edaee6,
    0x92722c851482353b,
    0xa2bfe8a14cf10364,
    0xa81a664bbc423001,
    0xc24b8b70d0f89791,
    0xc76c51a30654be30,
    0xd192e819d6ef5218,
    0xd69906245565a910,
    0xf40e35855771202a,
    0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8,
    0x1e376c085141ab53,
    0x2748774cdf8eeb99,
    0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63,
    0x4ed8aa4ae3418acb,
    0x5b9cca4f7763e373,
    0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc,
    0x78a5636f43172f60,
    0x84c87814a1f0ab72,
    0x8cc702081a6439ec,
    0x90befffa23631e28,
    0xa4506cebde82bde9,
    0xbef9a3f7b2c67915,
    0xc67178f2e372532b,
    0xca273eceea26619c,
    0xd186b8c721c0c207,
    0xeada7dd6cde0eb1e,
    0xf57d4f7fee6ed178,
    0x06f067aa72176fba,
    0x0a637dc5a2c898a6,
    0x113f9804bef90dae,
    0x1b710b35131c471b,
    0x28db77f523047d84,
    0x32caab7b40c72493,
    0x3c9ebe0a15c9bebc,
    0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6,
    0x597f299cfc657e2a,
    0x5fcb6fab3ad6faec,
    0x6c44198c4a475817,
];

// #[inline(always)]
fn load_be(base: &[u8; 64], offset: usize) -> u64 {
    (base[offset + 7] as u64)
        | (base[offset + 6] as u64) << 8
        | (base[offset + 5] as u64) << 16
        | (base[offset + 4] as u64) << 24
        | (base[offset + 3] as u64) << 32
        | (base[offset + 2] as u64) << 40
        | (base[offset + 1] as u64) << 48
        | (base[offset + 0] as u64) << 56
}

// #[inline(always)]
fn load_be_128(base: &[u8; 128], offset: usize) -> u64 {
    (base[offset + 7] as u64)
        | (base[offset + 6] as u64) << 8
        | (base[offset + 5] as u64) << 16
        | (base[offset + 4] as u64) << 24
        | (base[offset + 3] as u64) << 32
        | (base[offset + 2] as u64) << 40
        | (base[offset + 1] as u64) << 48
        | (base[offset + 0] as u64) << 56
}

// #[inline(always)]
fn store_be(base: &mut [u8; 64], offset: usize, x: u64) {
    base[offset + 7] = x as u8;
    base[offset + 6] = (x >> 8) as u8;
    base[offset + 5] = (x >> 16) as u8;
    base[offset + 4] = (x >> 24) as u8;
    base[offset + 3] = (x >> 32) as u8;
    base[offset + 2] = (x >> 40) as u8;
    base[offset + 1] = (x >> 48) as u8;
    base[offset + 0] = (x >> 56) as u8;
}

#[derive(Clone, Copy)]
struct State {
    st: [u64; 8],
}

impl State {
    #[inline(always)]
    fn new() -> Self {
        const IV: [u8; 64] = [
            0x6a, 0x09, 0xe6, 0x67, 0xf3, 0xbc, 0xc9, 0x08, 0xbb, 0x67, 0xae, 0x85, 0x84, 0xca,
            0xa7, 0x3b, 0x3c, 0x6e, 0xf3, 0x72, 0xfe, 0x94, 0xf8, 0x2b, 0xa5, 0x4f, 0xf5, 0x3a,
            0x5f, 0x1d, 0x36, 0xf1, 0x51, 0x0e, 0x52, 0x7f, 0xad, 0xe6, 0x82, 0xd1, 0x9b, 0x05,
            0x68, 0x8c, 0x2b, 0x3e, 0x6c, 0x1f, 0x1f, 0x83, 0xd9, 0xab, 0xfb, 0x41, 0xbd, 0x6b,
            0x5b, 0xe0, 0xcd, 0x19, 0x13, 0x7e, 0x21, 0x79,
        ];
        let mut t = [0u64; 8];

        unroll! {
            for i in 0..8 {
                t[i] = load_be(&IV, i * 8);
            }
        }
        State { st: t }
    }

    // #[inline(always)]
    fn store(&self, out: &mut [u8; 64]) {
        unroll! {
            for i in 0..8 {
                store_be(out, i * 8, self.st[i]);
            }
        }
    }

    // #[inline(always)]
    fn add(&mut self, x: &State) {
        let sx = &mut self.st;
        let ex = &x.st;

        unroll! {
            for i in 0..8 {
                sx[i] = sx[i].wrapping_add(ex[i]);
            }
        }
    }

    // #[inline(always)]
    fn blocks(&mut self, input: &[u8; 128]) {
        let mut t = *self;
        let mut w = W::new(input);

        unroll! {
            for i in 0..4 {
                w.big_g(&mut t, i);
                w.expand();
            }
        }
        w.big_g(&mut t, 4);
        t.add(self);
        self.st = t.st;
    }
}

struct W([u64; 16]);
impl W {
    // #[inline(always)]
    fn new(input: &[u8; 128]) -> Self {
        let mut w = [0u64; 16];

        unroll! {
            for i in 0..16 {
                w[i] = load_be_128(input, i * 8);
            }
        }
        W(w)
    }

    // #[inline(always)]
    fn big_ch(x: u64, y: u64, z: u64) -> u64 {
        (x & y) ^ (!x & z)
    }

    // #[inline(always)]
    fn big_maj(x: u64, y: u64, z: u64) -> u64 {
        (x & y) ^ (x & z) ^ (y & z)
    }

    // #[inline(always)]
    fn big_sigma0(x: u64) -> u64 {
        rotr(x, 28) ^ rotr(x, 34) ^ rotr(x, 39)
    }

    // #[inline(always)]
    fn big_sigma1(x: u64) -> u64 {
        rotr(x, 14) ^ rotr(x, 18) ^ rotr(x, 41)
    }

    // #[inline(always)]
    fn sigma0(x: u64) -> u64 {
        rotr(x, 1) ^ rotr(x, 8) ^ (x >> 7)
    }

    // #[inline(always)]
    fn sigma1(x: u64) -> u64 {
        rotr(x, 19) ^ rotr(x, 61) ^ (x >> 6)
    }

    // #[inline(always)]
    fn big_m(&mut self, a: usize, b: usize, c: usize, d: usize) {
        let w = &mut self.0;
        w[a] = w[a]
            .wrapping_add(Self::sigma1(w[b]))
            .wrapping_add(w[c])
            .wrapping_add(Self::sigma0(w[d]));
    }

    // #[inline(always)]
    fn expand(&mut self) {
        unroll! {
            for i in 0..16 {
                self.big_m(i, (i + 14) & 15, (i + 9) & 15, (i + 1) & 15);
            }
        }
    }

    // #[inline(always)]
    fn big_f(&mut self, state: &mut State, i: usize, k: u64) {
        let t = &mut state.st;
        t[(16 - i + 7) & 7] = t[(16 - i + 7) & 7]
            .wrapping_add(Self::big_sigma1(t[(16 - i + 4) & 7]))
            .wrapping_add(Self::big_ch(
                t[(16 - i + 4) & 7],
                t[(16 - i + 5) & 7],
                t[(16 - i + 6) & 7],
            ))
            .wrapping_add(k)
            .wrapping_add(self.0[i]);
        t[(16 - i + 3) & 7] = t[(16 - i + 3) & 7].wrapping_add(t[(16 - i + 7) & 7]);
        t[(16 - i + 7) & 7] = t[(16 - i + 7) & 7]
            .wrapping_add(Self::big_sigma0(t[(16 - i + 0) & 7]))
            .wrapping_add(Self::big_maj(
                t[(16 - i + 0) & 7],
                t[(16 - i + 1) & 7],
                t[(16 - i + 2) & 7],
            ));
    }

    // #[inline(always)]
    fn big_g(&mut self, state: &mut State, s: usize) {
        // let rc = &ROUND_CONSTANTS[s * 16..];
        unroll! {
            for i in 0..16 {
                self.big_f(state, i, ROUND_CONSTANTS[s * 16 + i]);
            }
        }
    }
}

pub fn hash(input: &[u8; 32]) -> [u8; 64] {
    let mut state = State::new();

    let mut padded = [0u8; 128];
    for i in 00..32 {
        padded[i] = input[i];
    }
    padded[32] = 0x80;
    let bits = 32 * 8;
    for i in 0..8 {
        padded[128 - 8 + i] = (bits as u64 >> (56 - i * 8)) as u8;
    }
    state.blocks(&padded);
    let mut out = [0u8; 64];
    state.store(&mut out);
    out
}
