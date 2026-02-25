use std::time::{Duration, Instant};

pub fn bench_iters<F: FnMut()>(mut f: F, iters: u64) -> Duration {
    let start = Instant::now();
    for _ in 0..iters {
        f();
    }
    start.elapsed()
}

pub fn ns_per_op(d: Duration, iters: u64) -> f64 {
    let ns = d.as_secs_f64() * 1e9;
    ns / (iters as f64)
}

pub fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() { return 0.0; }
    let idx = ((sorted.len() - 1) as f64 * p).round() as usize;
    sorted[idx.min(sorted.len()-1)]
}
