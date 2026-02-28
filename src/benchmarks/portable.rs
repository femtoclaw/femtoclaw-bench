use crate::report::{Metric, SuiteResult};
use crate::{protocol, util};
use serde_json::json;

fn metric(name: &str, unit: &str, iters: u64, dur: std::time::Duration) -> Metric {
    let total_ns = dur.as_secs_f64() * 1e9;
    Metric {
        name: name.to_string(),
        unit: unit.to_string(),
        iters,
        total_ns,
        ns_per_op: util::ns_per_op(dur, iters),
    }
}

fn scalar_metric(name: &str, unit: &str, value: f64) -> Metric {
    Metric {
        name: name.to_string(),
        unit: unit.to_string(),
        iters: 1,
        total_ns: value,
        ns_per_op: value,
    }
}

pub async fn run_preset(preset: &str, iters: u64) -> anyhow::Result<SuiteResult> {
    let mut metrics = Vec::new();
    let mut notes = vec![];

    match preset {
        "protocol" => {
            metrics.extend(protocol_benches(iters)?);
        }
        "dispatch" => {
            metrics.extend(dispatch_benches(iters));
        }
        "memory" => {
            metrics.extend(memory_benches(iters, &mut notes));
        }
        "e2e" => {
            metrics.extend(e2e_benches(iters)?);
        }
        "all" => {
            metrics.extend(protocol_benches(iters)?);
            metrics.extend(dispatch_benches(iters));
            metrics.extend(memory_benches(iters, &mut notes));
            metrics.extend(e2e_benches(iters)?);
            notes.push("Startup benchmark is separate (use `startup` command).".into());
        }
        _ => {
            // core
            metrics.extend(protocol_benches(iters)?);
            metrics.extend(dispatch_benches(iters));
            metrics.extend(memory_benches(iters, &mut notes));
        }
    }

    Ok(SuiteResult {
        suite: format!("FemtoClaw Bench ({preset})"),
        metrics,
        notes,
    })
}

fn protocol_benches(iters: u64) -> anyhow::Result<Vec<Metric>> {
    let msg = r#"{"message":{"content":"ok"}}"#;
    let tc = r#"{"tool_call":{"tool":"shell","args":{"bin":"ls","argv":["-la"]}}}"#;

    let d1 = util::bench_iters(
        || {
            let _ = protocol::parse_strict(msg).unwrap();
        },
        iters,
    );
    let d2 = util::bench_iters(
        || {
            let _ = protocol::parse_strict(tc).unwrap();
        },
        iters,
    );

    Ok(vec![
        metric("protocol.parse.message", "ns/op", iters, d1),
        metric("protocol.parse.tool_call", "ns/op", iters, d2),
    ])
}

fn dispatch_benches(iters: u64) -> Vec<Metric> {
    // Simulate tool registry lookup and basic arg validation.
    use std::collections::HashMap;
    let mut tools: HashMap<&'static str, u8> = HashMap::new();
    tools.insert("shell", 1);
    tools.insert("web_get", 2);

    let args = json!({"bin":"ls","argv":["-la"]});

    let d = util::bench_iters(
        || {
            let tool = tools.get("shell").unwrap();
            // simple validation
            let _bin = args.get("bin").and_then(|v| v.as_str()).unwrap();
            let _ = tool;
        },
        iters,
    );

    vec![metric(
        "capability.dispatch.lookup+validate",
        "ns/op",
        iters,
        d,
    )]
}

fn memory_benches(iters: u64, notes: &mut Vec<String>) -> Vec<Metric> {
    let mut metrics = Vec::new();

    // Bounded STM push + eviction latency
    const MAX: usize = 24;
    let mem_iters = (iters / 10).max(1);
    let d = util::bench_iters(
        || {
            let mut buf: Vec<String> = Vec::with_capacity(MAX + 1);
            for i in 0..MAX + 1 {
                buf.push(i.to_string());
                if buf.len() > MAX {
                    buf.drain(0..(buf.len() - MAX));
                }
            }
        },
        mem_iters,
    );
    metrics.push(metric(
        "memory.stm.push+evict(25 msgs)",
        "ns/op",
        mem_iters,
        d,
    ));

    // Working-set memory overhead (RSS) for a fixed allocation burst.
    const RSS_SAMPLES: usize = 4096;
    const PAYLOAD_BYTES: usize = 128;
    match util::current_rss_kib() {
        Some(before_kib) => {
            let mut payloads = Vec::with_capacity(RSS_SAMPLES);
            for i in 0..RSS_SAMPLES {
                payloads.push(format!("msg-{i:04}-{}", "x".repeat(PAYLOAD_BYTES)));
            }

            let after_kib = util::current_rss_kib().unwrap_or(before_kib);
            let delta_kib = after_kib.saturating_sub(before_kib);

            metrics.push(scalar_metric("memory.rss.before", "KiB", before_kib as f64));
            metrics.push(scalar_metric("memory.rss.after", "KiB", after_kib as f64));
            metrics.push(scalar_metric(
                "memory.rss.delta.alloc(4096 msgs)",
                "KiB",
                delta_kib as f64,
            ));

            std::hint::black_box(payloads.len());
        }
        None => notes.push("RSS memory overhead metric unavailable on this platform/runtime.".into()),
    }

    metrics
}

fn e2e_benches(iters: u64) -> anyhow::Result<Vec<Metric>> {
    // End-to-end simulated step: parse tool call -> dispatch -> produce message.
    let tc = r#"{"tool_call":{"tool":"shell","args":{"bin":"ls","argv":["-la"]}}}"#;
    let d = util::bench_iters(
        || {
            let out = protocol::parse_strict(tc).unwrap();
            match out {
                protocol::Output::ToolCall { tool, .. } => {
                    // dispatch
                    if tool != "shell" {
                        panic!("unexpected tool");
                    }
                    // result mediation (simulated)
                    let _final = "done";
                }
                _ => panic!("expected tool call"),
            }
        },
        iters,
    );

    Ok(vec![metric("e2e.simulated.step", "ns/op", iters, d)])
}
