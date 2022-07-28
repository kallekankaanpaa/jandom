pub trait Parts {
    fn parts(&self) -> (u8, u16, u64);
    fn from_parts(sign: u8, exponent: u16, significand: u64) -> f64;
}

impl Parts for f64 {
    fn parts(&self) -> (u8, u16, u64) {
        let significand = self.to_bits() & 0x000fffffffffffff;
        let exponent = self.to_bits() >> 52 & 0x7ff;
        let sign = self.to_bits() >> 63;
        (sign as u8, exponent as u16, significand)
    }
    fn from_parts(sign: u8, exponent: u16, significand: u64) -> f64 {
        f64::from_bits(
            u64::from(sign) << 63
                | u64::from(exponent & 0x7ff) << 52
                | (significand & 0x000fffffffffffff),
        )
    }
}

#[test]
fn parts() {
    let x = 12.345;
    let (sign, exponent, significand) = x.parts();
    assert_eq!(x, f64::from_parts(sign, exponent, significand))
}
