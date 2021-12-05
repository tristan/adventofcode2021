use std::time::Instant;

pub struct TimedOutput<T> {
    result: T,
    time: f64
}

impl<T> std::fmt::Display for TimedOutput<T>
where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (took {} seconds)", self.result, self.time)
    }
}

pub fn timeit<F, T>(f: F) -> TimedOutput<T>
where F: FnOnce() -> T {
    let start = Instant::now();
    let res = f();
    let runtime_nanos = start.elapsed().as_nanos();
    let runtime_secs = runtime_nanos as f64 / 1_000_000_000.0;
    TimedOutput {
        result: res,
        time: runtime_secs
    }
}

#[macro_export]
macro_rules! time {
    ($f:expr) => {
        adventofcode2021::timeit(|| $f)
    }
}
