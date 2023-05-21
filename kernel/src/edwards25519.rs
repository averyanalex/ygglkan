use core::ops::{Add, Sub};

use super::field25519::*;

#[derive(Clone, Copy)]
struct GeP2 {
    x: Fe,
    y: Fe,
    z: Fe,
}

#[derive(Clone, Copy)]
pub struct GeP3 {
    x: Fe,
    y: Fe,
    z: Fe,
    t: Fe,
}

#[derive(Clone, Copy, Default)]
struct GeP1P1 {
    x: Fe,
    y: Fe,
    z: Fe,
    t: Fe,
}

#[derive(Clone, Copy)]
struct GePrecomp {
    y_plus_x: Fe,
    y_minus_x: Fe,
    xy2d: Fe,
}

#[derive(Clone, Copy, Default)]
struct GeCached {
    y_plus_x: Fe,
    y_minus_x: Fe,
    z: Fe,
    t2d: Fe,
}

impl GeCached {
    fn maybe_set(&mut self, other: &GeCached, do_swap: u8) {
        self.y_plus_x.maybe_set(&other.y_plus_x, do_swap);
        self.y_minus_x.maybe_set(&other.y_minus_x, do_swap);
        self.z.maybe_set(&other.z, do_swap);
        self.t2d.maybe_set(&other.t2d, do_swap);
    }
}

impl GeP1P1 {
    fn to_p3(&self) -> GeP3 {
        GeP3 {
            x: self.x * self.t,
            y: self.y * self.z,
            z: self.z * self.t,
            t: self.x * self.y,
        }
    }
}

impl From<GeP2> for GeP3 {
    fn from(p: GeP2) -> GeP3 {
        GeP3 {
            x: p.x,
            y: p.y,
            z: p.z,
            t: p.x * p.y,
        }
    }
}

impl GeP2 {
    fn dbl(&self) -> GeP1P1 {
        let xx = self.x.square();
        let yy = self.y.square();
        let b = self.z.square_and_double();
        let a = self.x + self.y;
        let aa = a.square();
        let y3 = yy + xx;
        let z3 = yy - xx;
        let x3 = aa - y3;
        let t3 = b - z3;

        GeP1P1 {
            x: x3,
            y: y3,
            z: z3,
            t: t3,
        }
    }
}

impl GeP3 {
    fn to_p2(&self) -> GeP2 {
        GeP2 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    fn to_cached(&self) -> GeCached {
        GeCached {
            y_plus_x: self.y + self.x,
            y_minus_x: self.y - self.x,
            z: self.z,
            t2d: self.t * fe_d2(),
        }
    }

    fn zero() -> GeP3 {
        GeP3 {
            x: fe_zero(),
            y: fe_one(),
            z: fe_one(),
            t: fe_zero(),
        }
    }

    fn dbl(&self) -> GeP1P1 {
        self.to_p2().dbl()
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        let recip = self.z.invert();
        let x = self.x * recip;
        let y = self.y * recip;
        let mut bs = y.to_bytes();
        bs[31] ^= (if x.is_negative() { 1 } else { 0 }) << 7;
        bs
    }
}

impl Add<GeP3> for GeP3 {
    type Output = GeP3;

    fn add(self, other: GeP3) -> GeP3 {
        (self + other.to_cached()).to_p3()
    }
}

impl Sub<GeP3> for GeP3 {
    type Output = GeP3;

    fn sub(self, other: GeP3) -> GeP3 {
        (self - other.to_cached()).to_p3()
    }
}

impl Add<GeCached> for GeP3 {
    type Output = GeP1P1;

    fn add(self, _rhs: GeCached) -> GeP1P1 {
        let y1_plus_x1 = self.y + self.x;
        let y1_minus_x1 = self.y - self.x;
        let a = y1_plus_x1 * _rhs.y_plus_x;
        let b = y1_minus_x1 * _rhs.y_minus_x;
        let c = _rhs.t2d * self.t;
        let zz = self.z * _rhs.z;
        let d = zz + zz;
        let x3 = a - b;
        let y3 = a + b;
        let z3 = d + c;
        let t3 = d - c;

        GeP1P1 {
            x: x3,
            y: y3,
            z: z3,
            t: t3,
        }
    }
}

impl Add<GePrecomp> for GeP3 {
    type Output = GeP1P1;

    fn add(self, _rhs: GePrecomp) -> GeP1P1 {
        let y1_plus_x1 = self.y + self.x;
        let y1_minus_x1 = self.y - self.x;
        let a = y1_plus_x1 * _rhs.y_plus_x;
        let b = y1_minus_x1 * _rhs.y_minus_x;
        let c = _rhs.xy2d * self.t;
        let d = self.z + self.z;
        let x3 = a - b;
        let y3 = a + b;
        let z3 = d + c;
        let t3 = d - c;

        GeP1P1 {
            x: x3,
            y: y3,
            z: z3,
            t: t3,
        }
    }
}

impl Sub<GeCached> for GeP3 {
    type Output = GeP1P1;

    fn sub(self, _rhs: GeCached) -> GeP1P1 {
        let y1_plus_x1 = self.y + self.x;
        let y1_minus_x1 = self.y - self.x;
        let a = y1_plus_x1 * _rhs.y_minus_x;
        let b = y1_minus_x1 * _rhs.y_plus_x;
        let c = _rhs.t2d * self.t;
        let zz = self.z * _rhs.z;
        let d = zz + zz;
        let x3 = a - b;
        let y3 = a + b;
        let z3 = d - c;
        let t3 = d + c;

        GeP1P1 {
            x: x3,
            y: y3,
            z: z3,
            t: t3,
        }
    }
}

impl Sub<GePrecomp> for GeP3 {
    type Output = GeP1P1;

    fn sub(self, _rhs: GePrecomp) -> GeP1P1 {
        let y1_plus_x1 = self.y + self.x;
        let y1_minus_x1 = self.y - self.x;
        let a = y1_plus_x1 * _rhs.y_minus_x;
        let b = y1_minus_x1 * _rhs.y_plus_x;
        let c = _rhs.xy2d * self.t;
        let d = self.z + self.z;
        let x3 = a - b;
        let y3 = a + b;
        let z3 = d - c;
        let t3 = d + c;

        GeP1P1 {
            x: x3,
            y: y3,
            z: z3,
            t: t3,
        }
    }
}

fn ge_precompute(base: &GeP3) -> [GeCached; 16] {
    let base_cached = base.to_cached();
    let mut pc = [GeP3::zero(); 16];
    pc[1] = *base;
    for i in 2..16 {
        pc[i] = if i % 2 == 0 {
            pc[i / 2].dbl().to_p3()
        } else {
            pc[i - 1].add(base_cached).to_p3()
        }
    }
    let mut pc_cached: [GeCached; 16] = Default::default();
    for i in 0..16 {
        pc_cached[i] = pc[i].to_cached();
    }
    pc_cached
}

fn ge_scalarmult(scalar: &[u8; 32], base: &GeP3) -> GeP3 {
    let pc = ge_precompute(base);
    let mut q = GeP3::zero();
    let mut pos = 252;
    loop {
        let slot = ((scalar[pos >> 3] >> (pos & 7)) & 15) as usize;
        let mut t = pc[0];
        for i in 1..16 {
            t.maybe_set(&pc[i], (((slot ^ i).wrapping_sub(1)) >> 8) as u8 & 1);
        }
        q = q.add(t).to_p3();
        if pos == 0 {
            break;
        }
        q = q.dbl().to_p3().dbl().to_p3().dbl().to_p3().dbl().to_p3();
        pos -= 4;
    }
    q
}

pub fn ge_scalarmult_base(scalar: &[u8; 32]) -> GeP3 {
    const BXP: [u8; 32] = [
        0x1a, 0xd5, 0x25, 0x8f, 0x60, 0x2d, 0x56, 0xc9, 0xb2, 0xa7, 0x25, 0x95, 0x60, 0xc7, 0x2c,
        0x69, 0x5c, 0xdc, 0xd6, 0xfd, 0x31, 0xe2, 0xa4, 0xc0, 0xfe, 0x53, 0x6e, 0xcd, 0xd3, 0x36,
        0x69, 0x21,
    ];
    const BYP: [u8; 32] = [
        0x58, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
        0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
        0x66, 0x66,
    ];
    let bx = Fe::from_bytes(&BXP);
    let by = Fe::from_bytes(&BYP);
    let base = GeP3 {
        x: bx,
        y: by,
        z: fe_one(),
        t: bx * by,
    };
    ge_scalarmult(scalar, &base)
}
