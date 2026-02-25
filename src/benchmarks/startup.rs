use crate::report::{Metric, SuiteResult};
use crate::util;
use std::process::{Command, Stdio};
use std::time::Instant;

pub async fn run_startup(bin: &str, args: &[String], iterations: u32) -> anyhow::Result<SuiteResult> {
    let mut samples_ns: Vec<f64> = Vec::with_capacity(iterations as usize);

    for _ in 0..iterations {
        let start = Instant::now();
        let mut cmd = Command::new(bin);
        // Ask runtime to do a one-shot if supported; otherwise user supplies args.
        for a in args {
            cmd.arg(a);
        }
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let mut child = cmd.spawn()?;
        // Wait for exit; for strict readiness timing you can adapt to watch stdout marker.
        let _ = child.wait()?;
        let elapsed = start.elapsed().as_secs_f64() * 1e9;
        samples_ns.push(elapsed);
    }

    samples_ns.sort_by(|a,b| a.partial_cmp(b).unwrap());

    let p50 = util::percentile(&samples_ns, 0.50);
    let p95 = util::percentile(&samples_ns, 0.95);
    let p99 = util::percentile(&samples_ns, 0.99);

    let metrics = vec![
        Metric { name: "startup.p50".into(), unit: "ns".into(), iters: iterations as u64, total_ns: p50, ns_per_op: p50 },
        Metric { name: "startup.p95".into(), unit: "ns".into(), iters: iterations as u64, total_ns: p95, ns_per_op: p95 },
        Metric { name: "startup.p99".into(), unit: "ns".into(), iters: iterations as u64, total_ns: p99, ns_per_op: p99 },
    ];

    Ok(SuiteResult {
        suite: "FemtoClaw Startup Benchmark".into(),
        metrics,
        notes: vec![
            "This measures process spawn + runtime completion; for 'ready' markers, instrument your runtime to emit a readiness line.".into()
        ],
    })
}
