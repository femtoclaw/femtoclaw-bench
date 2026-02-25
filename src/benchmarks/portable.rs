use crate::{protocol, util};
use crate::report::{Metric, SuiteResult};
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
            metrics.extend(memory_benches(iters));
        }
        "e2e" => {
            metrics.extend(e2e_benches(iters)?);
        }
        "all" => {
            metrics.extend(protocol_benches(iters)?);
            metrics.extend(dispatch_benches(iters));
            metrics.extend(memory_benches(iters));
            metrics.extend(e2e_benches(iters)?);
            notes.push("Startup benchmark is separate (use `startup` command).".into());
        }
        _ => { // core
            metrics.extend(protocol_benches(iters)?);
            metrics.extend(dispatch_benches(iters));
            metrics.extend(memory_benches(iters));
        }
    }

    Ok(SuiteResult { suite: format!("FemtoClaw Bench ({preset})"), metrics, notes })
}

fn protocol_benches(iters: u64) -> anyhow::Result<Vec<Metric>> {
    let msg = r#"{"message":{"content":"ok"}}"#;
    let tc = r#"{"tool_call":{"tool":"shell","args":{"bin":"ls","argv":["-la"]}}}"#;

    let d1 = util::bench_iters(|| {
        let _ = protocol::parse_strict(msg).unwrap();
    }, iters);
    let d2 = util::bench_iters(|| {
        let _ = protocol::parse_strict(tc).unwrap();
    }, iters);

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

    let d = util::bench_iters(|| {
        let tool = tools.get("shell").unwrap();
        // simple validation
        let _bin = args.get("bin").and_then(|v| v.as_str()).unwrap();
        let _ = tool;
    }, iters);

    vec![metric("capability.dispatch.lookup+validate", "ns/op", iters, d)]
}

fn memory_benches(iters: u64) -> Vec<Metric> {
    // Bounded STM push + eviction
    const MAX: usize = 24;
    let d = util::bench_iters(|| {
        let mut buf: Vec<String> = Vec::with_capacity(MAX+1);
        for i in 0..MAX+1 {
            buf.push(i.to_string());
            if buf.len() > MAX {
                buf.drain(0..(buf.len()-MAX));
            }
        }
    }, (iters / 10).max(1)); // heavier loop
    vec![metric("memory.stm.push+evict(25 msgs)", "ns/op", (iters/10).max(1), d)]
}

fn e2e_benches(iters: u64) -> anyhow::Result<Vec<Metric>> {
    // End-to-end simulated step: parse tool call -> dispatch -> produce message.
    let tc = r#"{"tool_call":{"tool":"shell","args":{"bin":"ls","argv":["-la"]}}}"#;
    let d = util::bench_iters(|| {
        let out = protocol::parse_strict(tc).unwrap();
        match out {
            protocol::Output::ToolCall { tool, .. } => {
                // dispatch
                if tool != "shell" { panic!("unexpected tool"); }
                // result mediation (simulated)
                let _final = "done";
            }
            _ => panic!("expected tool call"),
        }
    }, iters);

    Ok(vec![metric("e2e.simulated.step", "ns/op", iters, d)])
}
