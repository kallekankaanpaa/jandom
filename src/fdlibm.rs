pub fn sqrt(x: f64) -> f64 {
    let _ = x;
    unimplemented!()
}

pub fn log(x: f64) -> f64 {
    let _ = x;
    unimplemented!()
}

trait Halves {
    type Half;
    fn high(&self) -> Self::Half;
    fn low(&self) -> Self::Half;
    fn set_high(&mut self, half: Self::Half);
    fn set_low(&mut self, half: Self::Half);
}

impl Halves for f64 {
    type Half = u32;
    fn high(&self) -> Self::Half {
        (self.to_bits() >> 32) as u32
    }

    fn low(&self) -> Self::Half {
        self.to_bits() as u32
    }

    fn set_high(&mut self, half: Self::Half) {
        unsafe {
            let ptr: *mut f64 = self;
            *ptr.cast::<Self::Half>().add(1) = half;
        }
    }

    fn set_low(&mut self, half: Self::Half) {
        unsafe {
            let ptr: *mut f64 = self;
            *ptr.cast::<Self::Half>() = half;
        }
    }
}

trait FromHalves {
    type Half;
    fn from_halves(high: Self::Half, low: Self::Half) -> Self;
}

impl FromHalves for f64 {
    type Half = u32;
    fn from_halves(high: Self::Half, low: Self::Half) -> Self {
        f64::from_bits(u64::from(high) << 32 | u64::from(low))
    }
}

#[test]
fn halves() {
    let x = 12.345;
    let mut y = 123.45;
    y.set_low(x.low());
    y.set_high(x.high());
    let z = f64::from_halves(x.high(), x.low());
    assert_eq!(x, y);
    assert_eq!(x, z);
}
