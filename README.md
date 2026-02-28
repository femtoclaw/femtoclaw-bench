# FemtoClaw Benchmark Suite

This repository provides the **industrial benchmark suite** for FemtoClaw-class runtimes.
It focuses on **measurable, repeatable** performance characteristics:

- Startup latency (process spawn + ready signal) *(optional)*
- Protocol parse/validate throughput
- Tool dispatch latency (capability lookup + arg validation)
- Memory operations (push/history/eviction)
- End-to-end step latency (simulated)
- Memory usage overhead (RSS before/after/delta)

## Quickstart

### Microbenchmarks (Criterion)
```bash
cargo bench
```

### CLI Benchmark Runner (portable)
```bash
cargo run --release -- run --preset core
cargo run --release -- run --preset all --json out.json
```

### Startup benchmark against a runtime binary
```bash
cargo run --release -- startup --bin ./target/release/femtoclaw --iterations 50
```

## Presets

- `core`: protocol + dispatch + memory
- `all`: core + end-to-end + startup (if enabled)
- `protocol`: protocol-only
- `dispatch`: capability dispatch-only
- `memory`: memory-only

## Output

- Console summary (always, including per-metric units)
- JSON report (optional with `--json`)

## Notes

This suite is designed to benchmark **runtime overhead**, not model intelligence.
To compare across runtimes, keep inference fixed (or use deterministic brains).
