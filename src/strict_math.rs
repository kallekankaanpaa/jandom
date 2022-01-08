extern "C" {
    fn __ieee754_sqrt(x: f64) -> f64;
    fn __ieee754_log(x: f64) -> f64;
}

pub fn sqrt(x: f64) -> f64 {
    unsafe { __ieee754_sqrt(x) }
}

pub fn log(x: f64) -> f64 {
    unsafe { __ieee754_log(x) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt() {
        assert_eq!(sqrt(100_f64), 10_f64);
        assert_eq!(sqrt(10_f64), 3.1622776601683795);
    }

    #[test]
    fn test_log() {
        assert_eq!(log(100_f64), 4.605170185988092);
        assert_eq!(log(10_f64), 2.302585092994046);
    }
}
