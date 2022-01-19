#![feature(once_cell)]
//! Pseudorandom number generator implemented with the same algorithm and parameters as
//! `java.util.Random`.
//!
//! This crate has feature parity with the Java 17 implementation of `Random`. The crate
//! includes the sources for fdlibm (freely distributable libm) which is used by StrictMath
//! in Java.

use std::fmt;
use std::lazy::SyncLazy;
use std::sync::Mutex;

#[cfg(test)]
mod tests;

extern "C" {
    fn sqrt(x: f64) -> f64;
    fn log(x: f64) -> f64;
}

const MULTIPLIER: i64 = 0x5deece66d;
const INCREMENT: i64 = 0xb;
const MASK: i64 = (1 << 48) - 1;

/// A pseudorandom number generator
pub struct Random {
    state: i64,
    next_next_gaussian: Option<f64>,
}

impl Random {
    /// Creates a new random number generator using a single [i64] seed.
    ///
    /// This has the same effect as calling the constructor with seed param in Java.
    pub fn new(seed: i64) -> Self {
        Self {
            state: Self::initalize_state(seed),
            next_next_gaussian: None,
        }
    }

    /// Calculates the the initial state from a seed
    fn initalize_state(seed: i64) -> i64 {
        seed ^ MULTIPLIER & MASK
    }

    /// Advances the RNG by one and returns random bits.
    fn next(&mut self, bits: u8) -> i32 {
        if bits > 32 {
            panic!("Can't return more than 32 random bits");
        }
        self.state = self.state.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT) & MASK;
        (self.state >> (48 - bits)) as i32
    }

    /// Returns the next pseudorandom, uniformly distributed [i32] value from this random
    /// number generator's sequence. The general contract of `next_i32` is that one [i32] value
    /// is pseudorandomly generated and returned. All 2^32 possible [i32] values are produced
    /// with (approximately) equal probability.
    pub fn next_i32(&mut self) -> i32 {
        self.next(32)
    }

    /// Returns a pseudorandom, uniformly distributed [i32] value between 0 (inclusive) and
    /// the specified value (exclusive), drawn from this random number generator's sequence.
    /// The general contract of `next_i32_bounded` is that one [i32] value in the specified range is
    /// pseudorandomly generated and returned. All bound possible [i32] values are produced
    /// with (approximately) equal probability.
    pub fn next_i32_bounded(&mut self, bound: i32) -> i32 {
        if bound <= 0 {
            panic!("bound can't be less than 1");
        }

        // bound is power of 2
        if (bound & -bound) == bound {
            let bound_i64 = bound as i64;
            let next_i64 = self.next(31) as i64;
            let result = bound_i64.wrapping_mul(next_i64) >> 31;
            result as i32
        } else {
            loop {
                let bits = self.next(31);
                let val = bits % bound;
                if !bits.wrapping_sub(val).wrapping_add(bound.wrapping_sub(1)) < 0 {
                    return val;
                }
            }
        }
    }

    /// Returns the next pseudorandom, uniformly distributed long value from this random
    /// number generator's sequence. The general contract of `next_i64` is that one long
    /// value is pseudorandomly generated and returned.
    pub fn next_i64(&mut self) -> i64 {
        ((self.next(32) as i64) << 32) + (self.next(32) as i64)
    }

    /// Returns the next pseudorandom, uniformly distributed boolean value from this
    /// random number generator's sequence. The general contract of `next_bool` is that
    /// one boolean value is pseudorandomly generated and returned. The values true and
    /// false are produced with (approximately) equal probability.
    pub fn next_bool(&mut self) -> bool {
        self.next(1) != 0
    }

    /// Returns the next pseudorandom, uniformly distributed [f32] value between 0.0 and
    /// 1.0 from this random number generator's sequence.
    ///
    /// The general contract of `next_f32` is that one [f32] value, chosen (approximately)
    /// uniformly from the range 0.0f (inclusive) to 1.0f (exclusive), is pseudorandomly
    /// generated and returned. All 2^24 possible [f32] values of the form m * 2^-24, where
    /// m is a positive integer less than 2^24, are produced with (approximately) equal
    /// probability.
    pub fn next_f32(&mut self) -> f32 {
        self.next(24) as f32 / ((1 << 24) as f32)
    }

    /// Returns the next pseudorandom, uniformly distributed [f64] value between 0.0 and
    /// 1.0 from this random number generator's sequence.
    ///
    /// The general contract of `next_f64` is that one [f64] value, chosen (approximately)
    /// uniformly from the range 0.0 (inclusive) to 1.0 (exclusive), is pseudorandomly
    /// generated and returned.
    pub fn next_f64(&mut self) -> f64 {
        ((((self.next(26) as i64) << 27) + (self.next(27) as i64)) as f64) / ((1_i64 << 53) as f64)
    }

    /// Generates random bytes and places them into a user-supplied i8 slice. The
    /// number of random bytes produced is equal to the length of the slice.
    pub fn next_bytes(&mut self, bytes: &mut [i8]) {
        let max = bytes.len() & !0x3;

        for i in (0..max).step_by(4) {
            let random = self.next(32);
            bytes[i] = random as i8;
            bytes[i + 1] = (random >> 8) as i8;
            bytes[i + 2] = (random >> 16) as i8;
            bytes[i + 3] = (random >> 24) as i8;
        }
        if max < bytes.len() {
            let mut random = self.next(32);
            for j in max..bytes.len() {
                bytes[j] = random as i8;
                random = random >> 8;
            }
        }
    }

    /// Returns the next pseudorandom, Gaussian ("normally") distributed [f64] value with
    /// mean 0.0 and standard deviation 1.0 from this random number generator's sequence.
    ///
    /// The general contract of `next_gaussian` is that one [f64] value, chosen from
    /// (approximately) the usual normal distribution with mean 0.0 and standard deviation
    /// 1.0, is pseudorandomly generated and returned.
    pub fn next_gaussian(&mut self) -> f64 {
        if let Some(gaussian) = self.next_next_gaussian {
            self.next_next_gaussian = None;
            gaussian
        } else {
            let mut s;
            let mut v1;
            let mut v2;
            loop {
                v1 = 2_f64 * self.next_f64() - 1_f64;
                v2 = 2_f64 * self.next_f64() - 1_f64;
                s = v1 * v1 + v2 * v2;
                if !(s >= 1_f64 || s == 0_f64) {
                    break;
                }
            }
            let multiplier;
            // SAFETY: `sqrt` and `log` are C functions which are safe to call with any
            // arguments.
            unsafe {
                multiplier = sqrt(-2_f64 * log(s) / s);
            }
            self.next_next_gaussian = Some(v2 * multiplier);
            v1 * multiplier
        }
    }
}

/// Implemented custom Debug in order to prevent users from leaking the internal state
impl fmt::Debug for Random {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Random number generator implemented with the same algorithm as java.util.Random"
        )
    }
}

static SEED_UNIQUFIER: SyncLazy<Mutex<i64>> = SyncLazy::new(|| Mutex::new(8682522807148012));

/// The default implementation represents the Java Random constructor with no params.
impl Default for Random {
    fn default() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        const MULTIPLIER: i64 = 1181783497276652981;
        let mut uniquifier = SEED_UNIQUFIER.lock().unwrap();

        *uniquifier = uniquifier.wrapping_mul(MULTIPLIER);

        let elapsed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime returned value earlier than UNIX_EPOCH");
        Self::new(*uniquifier ^ (elapsed.as_nanos() as i64))
    }
}
