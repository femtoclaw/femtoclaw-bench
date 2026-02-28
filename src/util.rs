use std::time::{Duration, Instant};

#[cfg(not(windows))]
use sysinfo::{ProcessExt, System, SystemExt};

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
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = ((sorted.len() - 1) as f64 * p).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

#[cfg(windows)]
pub fn current_rss_kib() -> Option<u64> {
    use std::mem::{size_of, zeroed};
    use windows_sys::Win32::System::ProcessStatus::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
    use windows_sys::Win32::System::Threading::GetCurrentProcess;

    unsafe {
        let handle = GetCurrentProcess();
        let mut counters: PROCESS_MEMORY_COUNTERS = zeroed();
        counters.cb = size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
        let ok = GetProcessMemoryInfo(
            handle,
            &mut counters as *mut PROCESS_MEMORY_COUNTERS as *mut _,
            counters.cb,
        );
        if ok == 0 {
            None
        } else {
            Some((counters.WorkingSetSize / 1024) as u64)
        }
    }
}

#[cfg(not(windows))]
pub fn current_rss_kib() -> Option<u64> {
    let pid = sysinfo::get_current_pid().ok()?;
    let mut sys = System::new_all();
    sys.refresh_process(pid);
    sys.process(pid).map(ProcessExt::memory)
}
