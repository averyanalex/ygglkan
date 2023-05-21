#![allow(unused_parens)]
#![allow(non_camel_case_types)]

use crunchy::unroll;

use core::cmp::{Eq, PartialEq};
use core::ops::{Add, Mul, Sub};

type fiat_25519_u1 = u8;
type fiat_25519_i1 = i8;
type fiat_25519_i2 = i8;

use crate::bigint::U128;

fn fiat_25519_addcarryx_u51(
    out1: &mut u64,
    out2: &mut fiat_25519_u1,
    arg1: fiat_25519_u1,
    arg2: u64,
    arg3: u64,
) {
    let x1: u64 = (((arg1 as u64).wrapping_add(arg2)).wrapping_add(arg3));
    let x2: u64 = (x1 & 0x7ffffffffffff);
    let x3: fiat_25519_u1 = ((x1 >> 51) as fiat_25519_u1);
    *out1 = x2;
    *out2 = x3;
}

fn fiat_25519_subborrowx_u51(
    out1: &mut u64,
    out2: &mut fiat_25519_u1,
    arg1: fiat_25519_u1,
    arg2: u64,
    arg3: u64,
) {
    let x1: i64 = (((arg2 as i64).wrapping_sub((arg1 as i64))).wrapping_sub((arg3 as i64)));
    let x2: fiat_25519_i1 = ((x1 >> 51) as fiat_25519_i1);
    let x3: u64 = (((x1 as i64) & 0x7ffffffffffff_i64) as u64);
    *out1 = x3;
    *out2 = ((0x0_i8.wrapping_sub((x2 as fiat_25519_i2))) as fiat_25519_u1);
}

fn fiat_25519_cmovznz_u64(out1: &mut u64, arg1: fiat_25519_u1, arg2: u64, arg3: u64) {
    let x1: fiat_25519_u1 = (!(!arg1));
    let x2: u64 = ((((0x0_i8.wrapping_sub((x1 as fiat_25519_i2))) as fiat_25519_i1) as i64) as u64);
    let x3: u64 = ((x2 & arg3) | ((!x2) & arg2));
    *out1 = x3;
}

fn fiat_25519_carry_mul(out1: &mut [u64; 5], arg1: &[u64; 5], arg2: &[u64; 5]) {
    let x1: U128 = U128::from_mul_u64(arg1[4], arg2[4].wrapping_mul(0x13));
    let x2: U128 = U128::from_mul_u64(arg1[4], arg2[3].wrapping_mul(0x13));
    let x3: U128 = U128::from_mul_u64(arg1[4], arg2[2].wrapping_mul(0x13));
    let x4: U128 = U128::from_mul_u64(arg1[4], arg2[1].wrapping_mul(0x13));
    let x5: U128 = U128::from_mul_u64(arg1[3], arg2[4].wrapping_mul(0x13));
    let x6: U128 = U128::from_mul_u64(arg1[3], arg2[3].wrapping_mul(0x13));
    let x7: U128 = U128::from_mul_u64(arg1[3], arg2[2].wrapping_mul(0x13));
    let x8: U128 = U128::from_mul_u64(arg1[2], arg2[4].wrapping_mul(0x13));
    let x9: U128 = U128::from_mul_u64(arg1[2], arg2[3].wrapping_mul(0x13));
    let x10: U128 = U128::from_mul_u64(arg1[1], arg2[4].wrapping_mul(0x13));
    let x11: U128 = U128::from_mul_u64(arg1[4], arg2[0]);
    let x12: U128 = U128::from_mul_u64(arg1[3], arg2[1]);
    let x13: U128 = U128::from_mul_u64(arg1[3], arg2[0]);
    let x14: U128 = U128::from_mul_u64(arg1[2], arg2[2]);
    let x15: U128 = U128::from_mul_u64(arg1[2], arg2[1]);
    let x16: U128 = U128::from_mul_u64(arg1[2], arg2[0]);
    let x17: U128 = U128::from_mul_u64(arg1[1], arg2[3]);
    let x18: U128 = U128::from_mul_u64(arg1[1], arg2[2]);
    let x19: U128 = U128::from_mul_u64(arg1[1], arg2[1]);
    let x20: U128 = U128::from_mul_u64(arg1[1], arg2[0]);
    let x21: U128 = U128::from_mul_u64(arg1[0], arg2[4]);
    let x22: U128 = U128::from_mul_u64(arg1[0], arg2[3]);
    let x23: U128 = U128::from_mul_u64(arg1[0], arg2[2]);
    let x24: U128 = U128::from_mul_u64(arg1[0], arg2[1]);
    let x25: U128 = U128::from_mul_u64(arg1[0], arg2[0]);
    let x26: U128 =
        (x25.wrapping_add((x10.wrapping_add((x9.wrapping_add((x7.wrapping_add(x4))))))));
    let x27: u64 = ((x26.shr(51)).to_u64());
    let x28: u64 = ((x26.to_u64() & 0x7ffffffffffff_u64));
    let x29: U128 =
        (x21.wrapping_add((x17.wrapping_add((x14.wrapping_add((x12.wrapping_add(x11))))))));
    let x30: U128 =
        (x22.wrapping_add((x18.wrapping_add((x15.wrapping_add((x13.wrapping_add(x1))))))));
    let x31: U128 =
        (x23.wrapping_add((x19.wrapping_add((x16.wrapping_add((x5.wrapping_add(x2))))))));
    let x32: U128 =
        (x24.wrapping_add((x20.wrapping_add((x8.wrapping_add((x6.wrapping_add(x3))))))));
    let x33: U128 = x32.add_u64(x27);
    let x34: u64 = ((x33.shr(51)).to_u64());
    let x35: u64 = ((x33.to_u64() & 0x7ffffffffffff_u64));
    let x36: U128 = x31.add_u64(x34);
    let x37: u64 = (x36.shr(51)).to_u64();
    let x38: u64 = ((x36.to_u64() & 0x7ffffffffffff_u64));
    let x39: U128 = x30.add_u64(x37);
    let x40: u64 = x39.shr(51).to_u64();
    let x41: u64 = (x39.to_u64() & 0x7ffffffffffff_u64);
    let x42: U128 = x29.add_u64(x40);
    let x43: u64 = ((x42.shr(51)).to_u64() as u64);
    let x44: u64 = ((x42.to_u64() & 0x7ffffffffffff_u64) as u64);
    let x45: u64 = (x43.wrapping_mul(0x13));
    let x46: u64 = (x28.wrapping_add(x45));
    let x47: u64 = (x46 >> 51);
    let x48: u64 = (x46 & 0x7ffffffffffff);
    let x49: u64 = (x47.wrapping_add(x35));
    let x50: fiat_25519_u1 = ((x49 >> 51) as fiat_25519_u1);
    let x51: u64 = (x49 & 0x7ffffffffffff);
    let x52: u64 = ((x50 as u64).wrapping_add(x38));
    out1[0] = x48;
    out1[1] = x51;
    out1[2] = x52;
    out1[3] = x41;
    out1[4] = x44;
}

fn fiat_25519_carry_square(out1: &mut [u64; 5], arg1: &[u64; 5]) {
    let x1: u64 = ((arg1[4]).wrapping_mul(0x13));
    let x2: u64 = (x1.wrapping_mul(0x2));
    let x3: u64 = ((arg1[4]).wrapping_mul(0x2));
    let x4: u64 = ((arg1[3]).wrapping_mul(0x13));
    let x5: u64 = (x4.wrapping_mul(0x2));
    let x6: u64 = ((arg1[3]).wrapping_mul(0x2));
    let x7: u64 = ((arg1[2]).wrapping_mul(0x2));
    let x8: u64 = ((arg1[1]).wrapping_mul(0x2));
    let x9: U128 =  U128::from_mul_u64(arg1[4], x1);
    let x10: U128 = U128::from_mul_u64(arg1[3], x2);
    let x11: U128 = U128::from_mul_u64(arg1[3], x4);
    let x12: U128 = U128::from_mul_u64(arg1[2], x2);
    let x13: U128 = U128::from_mul_u64(arg1[2], x5);
    let x14: U128 = U128::from_mul_u64(arg1[2], arg1[2]);
    let x15: U128 = U128::from_mul_u64(arg1[1], x2);
    let x16: U128 = U128::from_mul_u64(arg1[1], x6);
    let x17: U128 = U128::from_mul_u64(arg1[1], x7);
    let x18: U128 = U128::from_mul_u64(arg1[1], arg1[1]);
    let x19: U128 = U128::from_mul_u64(arg1[0], x3);
    let x20: U128 = U128::from_mul_u64(arg1[0], x6);
    let x21: U128 = U128::from_mul_u64(arg1[0], x7);
    let x22: U128 = U128::from_mul_u64(arg1[0], x8);
    let x23: U128 = U128::from_mul_u64(arg1[0], arg1[0]);
    let x24: U128 = (x23.wrapping_add((x15.wrapping_add(x13))));
    let x25: u64 = ((x24.shr(51)).to_u64());
    let x26: u64 = ((x24.to_u64() & 0x7ffffffffffff_u64));
    let x27: U128 = (x19.wrapping_add((x16.wrapping_add(x14))));
    let x28: U128 = (x20.wrapping_add((x17.wrapping_add(x9))));
    let x29: U128 = (x21.wrapping_add((x18.wrapping_add(x10))));
    let x30: U128 = (x22.wrapping_add((x12.wrapping_add(x11))));
    let x31: U128 = x30.add_u64(x25);
    let x32: u64 = ((x31.shr(51)).to_u64());
    let x33: u64 = ((x31.to_u64() & 0x7ffffffffffff_u64));
    let x34: U128 = x29.add_u64(x32);
    let x35: u64 = ((x34.shr(51).to_u64()));
    let x36: u64 = ((x34.to_u64() & 0x7ffffffffffff_u64));
    let x37: U128 = x28.add_u64(x35);
    let x38: u64 = ((x37.shr(51)).to_u64());
    let x39: u64 = ((x37.to_u64() & 0x7ffffffffffff_u64));
    let x40: U128 = x27.add_u64(x38);
    let x41: u64 = ((x40.shr(51)).to_u64());
    let x42: u64 = ((x40.to_u64() & 0x7ffffffffffff_u64));
    let x43: u64 = (x41.wrapping_mul(0x13));
    let x44: u64 = (x26.wrapping_add(x43));
    let x45: u64 = (x44 >> 51);
    let x46: u64 = (x44 & 0x7ffffffffffff);
    let x47: u64 = (x45.wrapping_add(x33));
    let x48: fiat_25519_u1 = ((x47 >> 51) as fiat_25519_u1);
    let x49: u64 = (x47 & 0x7ffffffffffff);
    let x50: u64 = ((x48 as u64).wrapping_add(x36));
    out1[0] = x46;
    out1[1] = x49;
    out1[2] = x50;
    out1[3] = x39;
    out1[4] = x42;
}

fn fiat_25519_carry(out1: &mut [u64; 5], arg1: &[u64; 5]) {
    let x1: u64 = (arg1[0]);
    let x2: u64 = ((x1 >> 51).wrapping_add((arg1[1])));
    let x3: u64 = ((x2 >> 51).wrapping_add((arg1[2])));
    let x4: u64 = ((x3 >> 51).wrapping_add((arg1[3])));
    let x5: u64 = ((x4 >> 51).wrapping_add((arg1[4])));
    let x6: u64 = ((x1 & 0x7ffffffffffff).wrapping_add(((x5 >> 51).wrapping_mul(0x13))));
    let x7: u64 = ((((x6 >> 51) as fiat_25519_u1) as u64).wrapping_add((x2 & 0x7ffffffffffff)));
    let x8: u64 = (x6 & 0x7ffffffffffff);
    let x9: u64 = (x7 & 0x7ffffffffffff);
    let x10: u64 = ((((x7 >> 51) as fiat_25519_u1) as u64).wrapping_add((x3 & 0x7ffffffffffff)));
    let x11: u64 = (x4 & 0x7ffffffffffff);
    let x12: u64 = (x5 & 0x7ffffffffffff);
    out1[0] = x8;
    out1[1] = x9;
    out1[2] = x10;
    out1[3] = x11;
    out1[4] = x12;
}

fn fiat_25519_add(out1: &mut [u64; 5], arg1: &[u64; 5], arg2: &[u64; 5]) {
    let x1: u64 = ((arg1[0]).wrapping_add((arg2[0])));
    let x2: u64 = ((arg1[1]).wrapping_add((arg2[1])));
    let x3: u64 = ((arg1[2]).wrapping_add((arg2[2])));
    let x4: u64 = ((arg1[3]).wrapping_add((arg2[3])));
    let x5: u64 = ((arg1[4]).wrapping_add((arg2[4])));
    out1[0] = x1;
    out1[1] = x2;
    out1[2] = x3;
    out1[3] = x4;
    out1[4] = x5;
}

fn fiat_25519_sub(out1: &mut [u64; 5], arg1: &[u64; 5], arg2: &[u64; 5]) {
    let x1: u64 = ((0xfffffffffffdau64.wrapping_add((arg1[0]))).wrapping_sub((arg2[0])));
    let x2: u64 = ((0xffffffffffffeu64.wrapping_add((arg1[1]))).wrapping_sub((arg2[1])));
    let x3: u64 = ((0xffffffffffffeu64.wrapping_add((arg1[2]))).wrapping_sub((arg2[2])));
    let x4: u64 = ((0xffffffffffffeu64.wrapping_add((arg1[3]))).wrapping_sub((arg2[3])));
    let x5: u64 = ((0xffffffffffffeu64.wrapping_add((arg1[4]))).wrapping_sub((arg2[4])));
    out1[0] = x1;
    out1[1] = x2;
    out1[2] = x3;
    out1[3] = x4;
    out1[4] = x5;
}

fn fiat_25519_selectznz(
    out1: &mut [u64; 5],
    arg1: fiat_25519_u1,
    arg2: &[u64; 5],
    arg3: &[u64; 5],
) {
    let mut x1: u64 = 0;
    fiat_25519_cmovznz_u64(&mut x1, arg1, (arg2[0]), (arg3[0]));
    let mut x2: u64 = 0;
    fiat_25519_cmovznz_u64(&mut x2, arg1, (arg2[1]), (arg3[1]));
    let mut x3: u64 = 0;
    fiat_25519_cmovznz_u64(&mut x3, arg1, (arg2[2]), (arg3[2]));
    let mut x4: u64 = 0;
    fiat_25519_cmovznz_u64(&mut x4, arg1, (arg2[3]), (arg3[3]));
    let mut x5: u64 = 0;
    fiat_25519_cmovznz_u64(&mut x5, arg1, (arg2[4]), (arg3[4]));
    out1[0] = x1;
    out1[1] = x2;
    out1[2] = x3;
    out1[3] = x4;
    out1[4] = x5;
}

fn fiat_25519_to_bytes(out1: &mut [u8; 32], arg1: &[u64; 5]) {
    let mut x1: u64 = 0;
    let mut x2: fiat_25519_u1 = 0;
    fiat_25519_subborrowx_u51(&mut x1, &mut x2, 0x0, (arg1[0]), 0x7ffffffffffed);
    let mut x3: u64 = 0;
    let mut x4: fiat_25519_u1 = 0;
    fiat_25519_subborrowx_u51(&mut x3, &mut x4, x2, (arg1[1]), 0x7ffffffffffff);
    let mut x5: u64 = 0;
    let mut x6: fiat_25519_u1 = 0;
    fiat_25519_subborrowx_u51(&mut x5, &mut x6, x4, (arg1[2]), 0x7ffffffffffff);
    let mut x7: u64 = 0;
    let mut x8: fiat_25519_u1 = 0;
    fiat_25519_subborrowx_u51(&mut x7, &mut x8, x6, (arg1[3]), 0x7ffffffffffff);
    let mut x9: u64 = 0;
    let mut x10: fiat_25519_u1 = 0;
    fiat_25519_subborrowx_u51(&mut x9, &mut x10, x8, (arg1[4]), 0x7ffffffffffff);
    let mut x11: u64 = 0;
    fiat_25519_cmovznz_u64(&mut x11, x10, 0x0_u64, 0xffffffffffffffff);
    let mut x12: u64 = 0;
    let mut x13: fiat_25519_u1 = 0;
    fiat_25519_addcarryx_u51(&mut x12, &mut x13, 0x0, x1, (x11 & 0x7ffffffffffed));
    let mut x14: u64 = 0;
    let mut x15: fiat_25519_u1 = 0;
    fiat_25519_addcarryx_u51(&mut x14, &mut x15, x13, x3, (x11 & 0x7ffffffffffff));
    let mut x16: u64 = 0;
    let mut x17: fiat_25519_u1 = 0;
    fiat_25519_addcarryx_u51(&mut x16, &mut x17, x15, x5, (x11 & 0x7ffffffffffff));
    let mut x18: u64 = 0;
    let mut x19: fiat_25519_u1 = 0;
    fiat_25519_addcarryx_u51(&mut x18, &mut x19, x17, x7, (x11 & 0x7ffffffffffff));
    let mut x20: u64 = 0;
    let mut x21: fiat_25519_u1 = 0;
    fiat_25519_addcarryx_u51(&mut x20, &mut x21, x19, x9, (x11 & 0x7ffffffffffff));
    let x22: u64 = (x20 << 4);
    let x23: u64 = (x18.wrapping_mul(0x2_u64));
    let x24: u64 = (x16 << 6);
    let x25: u64 = (x14 << 3);
    let x26: u8 = ((x12 & 0xff_u64) as u8);
    let x27: u64 = (x12 >> 8);
    let x28: u8 = ((x27 & 0xff_u64) as u8);
    let x29: u64 = (x27 >> 8);
    let x30: u8 = ((x29 & 0xff_u64) as u8);
    let x31: u64 = (x29 >> 8);
    let x32: u8 = ((x31 & 0xff_u64) as u8);
    let x33: u64 = (x31 >> 8);
    let x34: u8 = ((x33 & 0xff_u64) as u8);
    let x35: u64 = (x33 >> 8);
    let x36: u8 = ((x35 & 0xff_u64) as u8);
    let x37: u8 = ((x35 >> 8) as u8);
    let x38: u64 = (x25.wrapping_add((x37 as u64)));
    let x39: u8 = ((x38 & 0xff_u64) as u8);
    let x40: u64 = (x38 >> 8);
    let x41: u8 = ((x40 & 0xff_u64) as u8);
    let x42: u64 = (x40 >> 8);
    let x43: u8 = ((x42 & 0xff_u64) as u8);
    let x44: u64 = (x42 >> 8);
    let x45: u8 = ((x44 & 0xff_u64) as u8);
    let x46: u64 = (x44 >> 8);
    let x47: u8 = ((x46 & 0xff_u64) as u8);
    let x48: u64 = (x46 >> 8);
    let x49: u8 = ((x48 & 0xff_u64) as u8);
    let x50: u8 = ((x48 >> 8) as u8);
    let x51: u64 = (x24.wrapping_add((x50 as u64)));
    let x52: u8 = ((x51 & 0xff_u64) as u8);
    let x53: u64 = (x51 >> 8);
    let x54: u8 = ((x53 & 0xff_u64) as u8);
    let x55: u64 = (x53 >> 8);
    let x56: u8 = ((x55 & 0xff_u64) as u8);
    let x57: u64 = (x55 >> 8);
    let x58: u8 = ((x57 & 0xff_u64) as u8);
    let x59: u64 = (x57 >> 8);
    let x60: u8 = ((x59 & 0xff_u64) as u8);
    let x61: u64 = (x59 >> 8);
    let x62: u8 = ((x61 & 0xff_u64) as u8);
    let x63: u64 = (x61 >> 8);
    let x64: u8 = ((x63 & 0xff_u64) as u8);
    let x65: fiat_25519_u1 = ((x63 >> 8) as fiat_25519_u1);
    let x66: u64 = (x23.wrapping_add((x65 as u64)));
    let x67: u8 = ((x66 & 0xff_u64) as u8);
    let x68: u64 = (x66 >> 8);
    let x69: u8 = ((x68 & 0xff_u64) as u8);
    let x70: u64 = (x68 >> 8);
    let x71: u8 = ((x70 & 0xff_u64) as u8);
    let x72: u64 = (x70 >> 8);
    let x73: u8 = ((x72 & 0xff_u64) as u8);
    let x74: u64 = (x72 >> 8);
    let x75: u8 = ((x74 & 0xff_u64) as u8);
    let x76: u64 = (x74 >> 8);
    let x77: u8 = ((x76 & 0xff_u64) as u8);
    let x78: u8 = ((x76 >> 8) as u8);
    let x79: u64 = (x22.wrapping_add((x78 as u64)));
    let x80: u8 = ((x79 & 0xff_u64) as u8);
    let x81: u64 = (x79 >> 8);
    let x82: u8 = ((x81 & 0xff_u64) as u8);
    let x83: u64 = (x81 >> 8);
    let x84: u8 = ((x83 & 0xff_u64) as u8);
    let x85: u64 = (x83 >> 8);
    let x86: u8 = ((x85 & 0xff_u64) as u8);
    let x87: u64 = (x85 >> 8);
    let x88: u8 = ((x87 & 0xff_u64) as u8);
    let x89: u64 = (x87 >> 8);
    let x90: u8 = ((x89 & 0xff_u64) as u8);
    let x91: u8 = ((x89 >> 8) as u8);
    out1[0] = x26;
    out1[1] = x28;
    out1[2] = x30;
    out1[3] = x32;
    out1[4] = x34;
    out1[5] = x36;
    out1[6] = x39;
    out1[7] = x41;
    out1[8] = x43;
    out1[9] = x45;
    out1[10] = x47;
    out1[11] = x49;
    out1[12] = x52;
    out1[13] = x54;
    out1[14] = x56;
    out1[15] = x58;
    out1[16] = x60;
    out1[17] = x62;
    out1[18] = x64;
    out1[19] = x67;
    out1[20] = x69;
    out1[21] = x71;
    out1[22] = x73;
    out1[23] = x75;
    out1[24] = x77;
    out1[25] = x80;
    out1[26] = x82;
    out1[27] = x84;
    out1[28] = x86;
    out1[29] = x88;
    out1[30] = x90;
    out1[31] = x91;
}

#[derive(Clone, Default, Copy)]
pub struct Fe {
    pub d: [u64; 5],
}

impl PartialEq for Fe {
    fn eq(&self, other: &Fe) -> bool {
        let self_elems = self.d;
        let other_elems = other.d;

        unroll! {
            for i in 0..5 {
                if self_elems[i] != other_elems[i] {
                    return false;
                }
            }
        }

        true
    }
}
impl Eq for Fe {}

// pub static FE_ZERO: Fe = Fe([0, 0, 0, 0, 0]);
pub const fn fe_zero() -> Fe {
    Fe { d: [0, 0, 0, 0, 0] }
}
// pub static FE_ONE: Fe = Fe([1, 0, 0, 0, 0]);
pub const fn fe_one() -> Fe {
    Fe { d: [1, 0, 0, 0, 0] }
}
// pub static FE_D2: Fe = Fe([
//     1859910466990425,
//     932731440258426,
//     1072319116312658,
//     1815898335770999,
//     633789495995903,
// ]);
pub const fn fe_d2() -> Fe {
    Fe {
        d: [
            1859910466990425,
            932731440258426,
            1072319116312658,
            1815898335770999,
            633789495995903,
        ],
    }
}

fn load_8u(s: &[u8; 8]) -> u64 {
    (s[0] as u64)
        | ((s[1] as u64) << 8)
        | ((s[2] as u64) << 16)
        | ((s[3] as u64) << 24)
        | ((s[4] as u64) << 32)
        | ((s[5] as u64) << 40)
        | ((s[6] as u64) << 48)
        | ((s[7] as u64) << 56)
}

impl Add for Fe {
    type Output = Fe;

    fn add(self, _rhs: Fe) -> Fe {
        let Fe { d: f } = self;
        let Fe { d: g } = _rhs;
        let mut h = Fe::default();
        fiat_25519_add(&mut h.d, &f, &g);
        h
    }
}

impl Sub for Fe {
    type Output = Fe;

    fn sub(self, _rhs: Fe) -> Fe {
        let Fe { d: f } = self;
        let Fe { d: g } = _rhs;
        let mut h = Fe::default();
        fiat_25519_sub(&mut h.d, &f, &g);
        h.carry()
    }
}

impl Mul for Fe {
    type Output = Fe;

    fn mul(self, _rhs: Fe) -> Fe {
        let Fe { d: f } = self;
        let Fe { d: g } = _rhs;
        let mut h = Fe::default();
        fiat_25519_carry_mul(&mut h.d, &f, &g);
        h
    }
}

impl Fe {
    pub fn from_bytes(s: &[u8; 32]) -> Fe {
        let mut h = Fe::default();
        let mask = 0x7ffffffffffff;

        let mut ns = [0u8; 8];
        unroll! {
            for i in 0..8 {
                ns[i] = s[i + 0]
            }
        }
        h.d[0] = load_8u(&ns) & mask;
        unroll! {
            for i in 0..8 {
                ns[i] = s[i + 6]
            }
        }
        h.d[1] = (load_8u(&ns) >> 3) & mask;
        unroll! {
            for i in 0..8 {
                ns[i] = s[i + 12]
            }
        }
        h.d[2] = (load_8u(&ns) >> 6) & mask;
        unroll! {
            for i in 0..8 {
                ns[i] = s[i + 19]
            }
        }
        h.d[3] = (load_8u(&ns) >> 1) & mask;
        unroll! {
            for i in 0..8 {
                ns[i] = s[i + 24]
            }
        }
        h.d[4] = (load_8u(&ns) >> 12) & mask;
        h
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        let &Fe { d: es } = &self.carry();
        let mut s_ = [0u8; 32];
        fiat_25519_to_bytes(&mut s_, &es);
        s_
    }

    fn carry(&self) -> Fe {
        let mut h = Fe::default();
        fiat_25519_carry(&mut h.d, &self.d);
        h
    }

    pub fn maybe_set(&mut self, other: &Fe, do_swap: u8) {
        let &mut Fe { d: f } = self;
        let &Fe { d: g } = other;
        let mut t = [0u64; 5];
        fiat_25519_selectznz(&mut t, do_swap, &f, &g);
        self.d = t
    }

    pub fn square(&self) -> Fe {
        let &Fe { d: f } = &self;
        let mut h = Fe::default();
        fiat_25519_carry_square(&mut h.d, f);
        h
    }

    pub fn square_and_double(&self) -> Fe {
        let h = self.square();
        (h + h)
    }

    pub fn invert(&self) -> Fe {
        let z1 = *self;
        let z2 = z1.square();
        let z8 = z2.square().square();
        let z9 = z1 * z8;
        let z11 = z2 * z9;
        let z22 = z11.square();
        let z_5_0 = z9 * z22;
        let z_10_5 = (0..5).fold(z_5_0, |z_5_n, _| z_5_n.square());
        let z_10_0 = z_10_5 * z_5_0;
        let z_20_10 = (0..10).fold(z_10_0, |x, _| x.square());
        let z_20_0 = z_20_10 * z_10_0;
        let z_40_20 = (0..20).fold(z_20_0, |x, _| x.square());
        let z_40_0 = z_40_20 * z_20_0;
        let z_50_10 = (0..10).fold(z_40_0, |x, _| x.square());
        let z_50_0 = z_50_10 * z_10_0;
        let z_100_50 = (0..50).fold(z_50_0, |x, _| x.square());
        let z_100_0 = z_100_50 * z_50_0;
        let z_200_100 = (0..100).fold(z_100_0, |x, _| x.square());
        let z_200_0 = z_200_100 * z_100_0;
        let z_250_50 = (0..50).fold(z_200_0, |x, _| x.square());
        let z_250_0 = z_250_50 * z_50_0;
        let z_255_5 = (0..5).fold(z_250_0, |x, _| x.square());
        let z_255_21 = z_255_5 * z11;
        z_255_21
    }

    pub fn is_negative(&self) -> bool {
        (self.to_bytes()[0] & 1) != 0
    }
}
