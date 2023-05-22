pub struct U128 {
    pub higher: u64,
    pub lower: u64,
}

impl From<u64> for U128 {
    fn from(n: u64) -> Self {
        Self {
            lower: n,
            higher: 0,
        }
    }
}

impl core::ops::BitAnd for U128 {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        Self {
            lower: self.lower & other.lower,
            higher: self.higher & other.higher,
        }
    }
}

fn u64_long_mul(left: u64, right: u64) -> U128 {
    let a = left >> 32;
    let b = left & 0xffffffff;
    let c = right >> 32;
    let d = right & 0xffffffff;

    let lo = b.wrapping_mul(d);

    let bc = b.wrapping_mul(c);
    let ad = a.wrapping_mul(d);
    let mid = ad.wrapping_add(bc);

    let carry = u64::MAX - ad < bc;
    let hi = a
        .wrapping_mul(c)
        .wrapping_add(if carry { 1 << 32 } else { 0 });

    U128::from_parts(hi, lo).wrapping_add(U128::from_parts(mid >> 32, mid << 32))
}

impl U128 {
    pub fn from_parts(hi: u64, lo: u64) -> Self {
        Self {
            lower: lo,
            higher: hi,
        }
    }

    #[inline(always)]
    pub fn from_mul_u64(a: u64, b: u64) -> Self {
        let mut low = u64_long_mul(a, b);
        low.higher = low.higher;
        low
    }

    pub fn wrapping_add(&self, other: U128) -> Self {
        let lo = self.lower.wrapping_add(other.lower);
        let carry = u64::MAX - self.lower < other.lower;
        let hi = self.higher.wrapping_add(other.higher);
        let hi = hi.wrapping_add(if carry { 1 } else { 0 });
        Self {
            higher: hi,
            lower: lo,
        }
    }

    pub fn to_u64(&self) -> u64 {
        self.lower
    }

    pub fn shr(&self, shift: u32) -> Self {
        let lo = self.lower;
        let hi = self.higher;

        let (hi, lo) = if (shift & 64) != 0 {
            (0, hi.wrapping_shr(shift & 63))
        } else {
            let new_hi = hi.wrapping_shr(shift);
            let mut new_lo = lo.wrapping_shr(shift);
            if (shift & 127) != 0 {
                new_lo |= hi.wrapping_shl(64u32.wrapping_sub(shift));
            }
            (new_hi, new_lo)
        };

        Self {
            higher: hi,
            lower: lo,
        }
    }

    pub fn add_u64(&self, n: u64) -> Self {
        let lo = self.lower.wrapping_add(n);
        let carry = u64::MAX - self.lower < n;
        let hi = self.higher;
        let hi = hi.wrapping_add(if carry { 1 } else { 0 });
        Self {
            higher: hi,
            lower: lo,
        }
    }

    pub const fn _to_u128(&self) -> u128 {
        ((self.higher as u128) << 64) + self.lower as u128
    }
}

pub fn rotr(n: u64, shift: u32) -> u64 {
    (n.wrapping_shr(shift)) | (n.wrapping_shl(64 - shift))
}
