#[cfg(test)]
mod tests;

use super::parts::Parts;
use super::{ONE, TINY};

pub fn sqrt(x: f64) -> f64 {
    if x.is_nan() {
        return f64::NAN;
    }

    if x.is_infinite() {
        if x.is_sign_positive() {
            return f64::INFINITY;
        } else {
            return f64::NAN;
        }
    }

    if x == 0.0 {
        return x;
    }

    if x < 0.0 {
        return f64::NAN;
    }

    let (_, mut exponent, mut significand) = x.parts();

    if x.is_subnormal() {
        // while the leftmost bit of significand is zero
        // double significand and decrease exponent by 1
        while significand & 0x0010000000000000 == 0 {
            significand <<= 1;
            //exponent -= 1;
            exponent = exponent.wrapping_sub(1);
        }
    }
    exponent = exponent.wrapping_add(1);
    //exponent -= 1023;
    exponent = exponent.wrapping_sub(1023);
    significand &= 0x000fffffffffffff;
    significand |= 0x0010000000000000;

    if exponent & 1 == 1 {
        // Odd exponent, double x to make it even
        significand <<= 1;
    }
    exponent >>= 1; // exponent = exponent / 2

    // Generate sqrt(x) bit by bit
    significand <<= 1;
    let mut r: u64 = 0x0020000000000000;
    let mut s: u64 = 0;
    let mut q: u64 = 0;
    while r != 0 {
        let t = s + r;
        if t <= significand {
            s = t + r;
            significand -= t;
            q |= r;
        }
        significand <<= 1;
        r >>= 1;
    }

    // use floating addition to find out rounding direction
    if significand != 0 {
        let mut z = ONE - TINY;
        if z >= ONE {
            z = ONE + TINY;
            if q & 0xffffffff == 0xffffffff {
                q &= 0xffffffff00000000;
                q += 1 << 32;
            } else if z > ONE {
                if q & 0xffffffff == 0xfffffffe {
                    q += 1 << 32;
                }
                q += 2;
            } else {
                q += q & 1;
            }
        }
    }

    // Discard the rounding bit and calculate the exponent
    q >>= 1;
    exponent += 0x3ff;

    // Sign must always be 0 since square root can't be negative
    // -0 case is handeled before
    f64::from_parts(0, exponent, q)
}
