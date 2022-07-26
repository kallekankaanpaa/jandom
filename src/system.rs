use once_cell::sync::Lazy;

#[link(name = "user32")]
extern "stdcall" {
    fn QueryPerformanceFrequency(frequency: *mut i64) -> i32;
    fn QueryPerformanceCounter(count: *mut i64) -> i32;
}

const NANOSECS_PER_SEC: f64 = 1_000_000_000_f64;

#[cfg(windows)]
static PERFORMANCE_FREQUENCY: Lazy<i64> = Lazy::new(|| {
    let mut f: i64 = 0;
    unsafe {
        QueryPerformanceFrequency(&mut f);
    }
    f
});

#[cfg(windows)]
pub fn nano_time() -> i64 {
    let mut count: i64 = 0;
    unsafe {
        QueryPerformanceCounter(&mut count);
    }
    ((count as f64) / (*PERFORMANCE_FREQUENCY as f64) * NANOSECS_PER_SEC) as i64
}
