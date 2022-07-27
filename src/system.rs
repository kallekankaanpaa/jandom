const NANOSECS_PER_SEC: i64 = 1_000_000_000;

#[cfg(windows)]
#[link(name = "user32")]
extern "stdcall" {
    fn QueryPerformanceFrequency(frequency: *mut i64) -> i32;
    fn QueryPerformanceCounter(count: *mut i64) -> i32;
}

#[cfg(windows)]
static PERFORMANCE_FREQUENCY: once_cell::sync::Lazy<i64> = once_cell::sync::Lazy::new(|| {
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
    ((count as f64) / *PERFORMANCE_FREQUENCY as f64 * NANOSECS_PER_SEC as f64) as i64
}

#[cfg(unix)]
const CLOCK_MONOTONIC: i32 = 1;

#[cfg(unix)]
extern "C" {
    fn clock_gettime(clock_id: i32, timespec: *mut Timespec) -> i32;
}

#[cfg(unix)]
#[repr(C)]
struct Timespec {
    seconds: i64,
    nano_seconds: i64,
}

#[cfg(unix)]
pub fn nano_time() -> i64 {
    unsafe {
        let mut timespec = std::mem::zeroed::<Timespec>();
        let status = clock_gettime(CLOCK_MONOTONIC, &mut timespec);
        assert_eq!(status, 0, "clock_gettime failed");
        timespec.seconds * NANOSECS_PER_SEC + timespec.nano_seconds
    }
}
