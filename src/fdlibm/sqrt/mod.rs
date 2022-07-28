#[cfg(test)]
mod tests;

use super::halves::{FromHalves, Halves};
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

    let (mut sign, mut exponent, mut significand) = x.parts();
    println!("------------------------BEFORE----------------");
    println!("significand =\t{:064b}", significand);
    println!("exponent =\t{:011b}", exponent);

    if x.is_subnormal() {
        // while last 21 bits of the significand are zero (bits 33..53)
        // shift significand left by 21
        while significand & 0x0010000000000000 == 0 {
            significand <<= 1;
            exponent -= 1;
        }
        // while significand has 0 bits shift it left
        // substract i - 1 from exponent
    }
    exponent -= 1023;
    // set exponent of high word to 1
    significand &= 0x000fffffffffffff;
    significand |= 0x0010000000000000;

    if exponent & 1 == 1 {
        // Odd exponent, double x to make it even
        significand <<= 1;
    }
    exponent >>= 1; // exponent = exponent / 2
    println!("------------------------AFTER----------------");
    println!("significand =\t{:064b}", significand);
    println!("exponent =\t{:011b}", exponent);

    // Generate sqrt(x) bit by bit
    let mut r = 0x00200000;
    while r != 0 {
        r >>= 1;
    }

    /*
       m -= 1023;
       h = h & 0x000fffff | 0x00100000;
       if m & 1 == 1 {
           h += h + (l & 0x80000000) >> 31;
           l += l;
       }
       m >>= 1;

       h += h + (l & 0x80000000) >> 31;
       l += l;
       let (mut q, mut q1, mut s0, mut s1, mut t) = (0, 0_u32, 0, 0_u32, 0);
       let mut r = 0x00200000;

       while r != 0 {
           t = s0 + r;
           if t <= h {
               s0 = t + r;
               h -= t;
               q += r;
           }
           h += h + (l & 0x80000000) >> 31;
           l += l;
           r >>= 1;
       }

       let (mut t, mut t1) = (0, 0_u32);
       r = 0x80000000;
       while r != 0 {
           t1 = s1 + r;
           t = s0;
           if t < h || (t == h && t1 <= l) {
               s1 = t1 + r;
               if t1 & 0x80000000 == 0x80000000 && s1 & 0x80000000 == 0 {
                   s0 += 1;
               }
               h -= t;
               if l < t1 {
                   h -= 1;
               }
               l -= t1;
               q1 += r;
           }
           h += h + (l & 0x80000000) >> 31;
           l += l;
           r >>= 1;
       }

       let mut z: f64 = 0.0;

       if h | l != 0 {
           z = ONE - TINY;
           if z >= ONE {
               z = ONE + TINY;
               if q1 == 0xffffffff {
                   q1 = 0;
                   q += 1;
               }
           } else if z > ONE {
               if q1 == 0xfffffffe {
                   q += 1;
               }
               q1 += 2;
           } else {
               q1 += q1 & 1;
           }
       }

       h = (q >> 1) + 0x3fe00000;
       l = q1 >> 1;
       if q & 1 == 1 {
           l |= 0x80000000;
       }
       h += m << 20;

       f64::from_halves(h, l)
    */
    f64::from_parts(sign, exponent, significand)
}
