use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_protocol(c: &mut Criterion) {
    let msg = r#"{"message":{"content":"ok"}}"#;
    let tc = r#"{"tool_call":{"tool":"shell","args":{"bin":"ls","argv":["-la"]}}}"#;

    c.bench_function("protocol.parse.message", |b| {
        b.iter(|| {
            let _ = femtoclaw_bench::protocol::parse_strict(black_box(msg)).unwrap();
        })
    });

    c.bench_function("protocol.parse.tool_call", |b| {
        b.iter(|| {
            let _ = femtoclaw_bench::protocol::parse_strict(black_box(tc)).unwrap();
        })
    });
}

fn bench_dispatch(c: &mut Criterion) {
    use std::collections::HashMap;
    let mut tools: HashMap<&'static str, u8> = HashMap::new();
    tools.insert("shell", 1);
    tools.insert("web_get", 2);

    let args = serde_json::json!({"bin":"ls","argv":["-la"]});

    c.bench_function("capability.dispatch.lookup+validate", |b| {
        b.iter(|| {
            let tool = tools.get("shell").unwrap();
            let _bin = args.get("bin").and_then(|v| v.as_str()).unwrap();
            black_box(tool);
            black_box(_bin);
        })
    });
}

criterion_group!(benches, bench_protocol, bench_dispatch);
criterion_main!(benches);
