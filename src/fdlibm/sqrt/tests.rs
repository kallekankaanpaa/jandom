use crate::fdlibm::TINY;

extern "C" {
    fn __ieee754_sqrt(x: f64) -> f64;
    fn __ieee754_new_sqrt(x: f64) -> f64;
}
#[test]
fn asdf() {
    let x = 12.345;
    unsafe {
        __ieee754_new_sqrt(x);
    }
    super::sqrt(x);
}

#[test]
fn new() {
    let x = 12.345;
    unsafe {
        assert_eq!(__ieee754_sqrt(x), __ieee754_new_sqrt(x));
    }
    unsafe {
        assert_eq!(__ieee754_sqrt(TINY), __ieee754_new_sqrt(TINY));
    }
}

#[test]
fn sqrt_eq_zero() {
    let x = 0_f64;
    let r_result = super::sqrt(x);
    let c_result = unsafe { __ieee754_sqrt(x) };
    assert_eq!(
        r_result,
        c_result,
        "binary repr left: {:b} right: {:b}",
        r_result.to_bits(),
        c_result.to_bits()
    );
    let x = -0_f64;
    let r_result = super::sqrt(x);
    let c_result = unsafe { __ieee754_sqrt(x) };
    assert_eq!(
        r_result,
        c_result,
        "binary repr left: {:b} right: {:b}",
        r_result.to_bits(),
        c_result.to_bits()
    );
}

#[test]
fn sqrt_eq_inf() {
    let x = f64::INFINITY;
    let r_result = super::sqrt(x);
    let c_result = unsafe { __ieee754_sqrt(x) };
    assert_eq!(
        r_result,
        c_result,
        "binary repr left: {:b} right: {:b}",
        r_result.to_bits(),
        c_result.to_bits()
    );
    let x = f64::NEG_INFINITY;
    let r_result = super::sqrt(x);
    let c_result = unsafe { __ieee754_sqrt(x) };
    assert_eq!(
        r_result.is_nan(),
        c_result.is_nan(),
        "binary repr left: {:b} right: {:b}",
        r_result.to_bits(),
        c_result.to_bits()
    );
}

#[test]
fn sqrt_eq_neg() {
    let x = -1.0;
    let r_result = super::sqrt(x);
    let c_result = unsafe { __ieee754_sqrt(x) };
    assert_eq!(
        r_result.is_nan(),
        c_result.is_nan(),
        "binary repr left: {:b} right: {:b}",
        r_result.to_bits(),
        c_result.to_bits()
    );
}
