//! Pseudorandom number generator implemented with the same algorithm and parameters as
//! `java.util.Random`.
//!
//! This crate has feature parity with the Java 17 implementation of `Random`. The crate
//! includes the sources for fdlibm (freely distributable libm) which is used by `StrictMath`
//! in Java.

use std::fmt;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex};

mod system;
#[cfg(test)]
mod tests;

extern "C" {
    fn __ieee754_sqrt(x: f64) -> f64;
    fn __ieee754_log(x: f64) -> f64;
}

const MULTIPLIER: i64 = 0x0005_deec_e66d;
const INCREMENT: i64 = 0xb;
const MASK: i64 = (1 << 48) - 1;

/// A pseudorandom number generator
pub struct Random {
    state: AtomicI64,
    next_next_gaussian: Arc<Mutex<Option<f64>>>,
}

impl Random {
    /// Creates a new random number generator using a single [i64] seed.
    ///
    /// This has the same effect as calling the constructor with seed param in Java.
    #[must_use]
    pub fn new(seed: i64) -> Self {
        Self {
            state: AtomicI64::new(Self::initalize_state(seed)),
            next_next_gaussian: Arc::new(Mutex::new(None)),
        }
    }

    /// Calculates the the initial state from a seed
    const fn initalize_state(seed: i64) -> i64 {
        seed ^ MULTIPLIER & MASK
    }

    /// Advances the RNG by one and returns random bits.
    ///
    /// # Panics
    ///
    /// Panics if `bits` is bigger than 32
    fn next(&mut self, bits: u8) -> i32 {
        assert!(bits <= 32, "can't return more than 32 random bits");

        let mut previous_state = self.state.load(Ordering::Acquire);
        loop {
            let new_state = previous_state
                .wrapping_mul(MULTIPLIER)
                .wrapping_add(INCREMENT)
                & MASK;
            // Using weak since it allows optimizations on certain (e.g. ARM) architectures
            match self.state.compare_exchange_weak(
                previous_state,
                new_state,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ) {
                Ok(_) => return (new_state >> (48 - bits)) as i32,
                Err(state) => previous_state = state,
            }
        }
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
    ///
    /// # Panics
    ///
    /// Panics if bound is zero or negative
    pub fn next_i32_bounded(&mut self, bound: i32) -> i32 {
        assert!(bound > 0, "bound must be positive");

        // bound is power of 2
        if (bound & -bound) == bound {
            let bound_i64 = i64::from(bound);
            let next_i64 = i64::from(self.next(31));
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
        (i64::from(self.next(32)) << 32) + i64::from(self.next(32))
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
        (((i64::from(self.next(26)) << 27) + i64::from(self.next(27))) as f64)
            / ((1_i64 << 53) as f64)
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
            for byte in bytes.iter_mut().skip(max) {
                *byte = random as i8;
                random >>= 8;
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
        let mutex = self.next_next_gaussian.clone();
        let mut next_gaussian = mutex.lock().unwrap();
        if let Some(gaussian) = *next_gaussian {
            *next_gaussian = None;
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
                multiplier = __ieee754_sqrt(-2_f64 * __ieee754_log(s) / s);
            }
            *next_gaussian = Some(v2 * multiplier);
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

static SEED_UNIQUFIER: AtomicI64 = AtomicI64::new(8_682_522_807_148_012);

/// The default implementation represents the Java Random constructor with no params.
impl Default for Random {
    #[inline]
    fn default() -> Self {
        const MULTIPLIER: i64 = 1_181_783_497_276_652_981;

        let mut current = SEED_UNIQUFIER.load(Ordering::Acquire);
        loop {
            let new = current.wrapping_mul(MULTIPLIER);
            match SEED_UNIQUFIER.compare_exchange_weak(
                current,
                new,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ) {
                Ok(_) => return Self::new(new ^ system::nano_time()),
                Err(uniquifier) => current = uniquifier,
            }
        }
    }
}
