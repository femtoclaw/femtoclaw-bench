# ⏱️ FemtoClaw Benchmark Suite

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Tier](https://img.shields.io/badge/Tier-Enterprise-green.svg)]()

The **FemtoClaw Benchmark Suite** provides the definitive performance measurement framework for industrial agent runtimes. Unlike LLM benchmarks that evaluate model intelligence or reasoning capability, this suite focuses exclusively on **runtime overhead**, **execution determinism**, and **resource predictability**.

In high-stakes production environments, the latency of the "Authority Layer" (validation, authorization, and dispatch) must be minimal and consistent. This crate ensures that FemtoClaw-class runtimes maintain these industrial guarantees.

---

## 📊 Key Performance Metrics

The suite evaluates runtimes across five critical performance domains:

### 1. Protocol Throughput
Measures the latency and throughput of the JSON protocol validator.
- **Metric**: ops/sec, nanoseconds/parse.
- **Why**: Determines how quickly the runtime can ingest and verify complex multi-tool intent from a Brain.

### 2. Capability Dispatch Latency
Measures the time from protocol validation to the start of actual system execution.
- **Metric**: microseconds/dispatch.
- **Why**: High-frequency automation requires zero-jitter transitions between the "Think" and "Act" phases.

### 3. Memory Subsystem Performance
Evaluates the efficiency of state persistence and history management (STM and WAL).
- **Metric**: ns/push, ns/replay.
- **Why**: Ensuring that recording execution history doesn't become a bottleneck in long-running autonomous loops.

### 4. Startup & Ready Signal
Measures the cold-start time of the runtime binary until it is ready to ingest the first protocol message.
- **Metric**: milliseconds to ready.
- **Why**: Critical for serverless deployments and ephemeral edge containers.

### 5. Memory Footprint (RSS)
Tracks Resident Set Size (RSS) delta during intensive multi-step execution.
- **Metric**: MB peak RSS, MB/iteration leak detection.
- **Why**: Prevents resource exhaustion in long-lived background daemons.

---

## 🛠️ Quickstart

### 1. Run Microbenchmarks
Utilizes `criterion` for high-precision measurement of core library functions.
```bash
cargo bench
```

### 2. Run Comprehensive CLI Suite
The portable benchmark runner provides a high-level summary of runtime characteristics.
```bash
# Build optimized benchmark binary
cargo build --release --bin femtoclaw-bench

# Execute all standard presets
./target/release/femtoclaw-bench run --preset all --json out.json
```

---

## 📖 Command Reference

### `run --preset <PRESET>`
- `core`: Focuses on protocol validation and capability dispatch.
- `memory`: Stress tests the Short-Term Memory (STM) and Write-Ahead Log (WAL).
- `all`: Executes the full suite including end-to-end simulated loops.

### `startup --bin <PATH>`
Benchmarks the binary lifecycle.
```bash
./target/release/femtoclaw-bench startup --bin ../target/release/femtoclaw --iterations 100
```

---

## 🔬 Measurement Methodology

FemtoClaw benchmarks follow strict industrial reproducibility rules:
1.  **Isolation**: Benchmarks should be run on a system with minimal background load.
2.  **Warmup**: Every test includes a non-recorded warmup phase to trigger CPU governor and cache optimization.
3.  **Outlier Rejection**: Statistical analysis is applied to remove anomalous data points caused by OS context switching.
4.  **Deterministic Brains**: To isolate runtime performance, all end-to-end tests use the `echo` or `mock` brain backends to eliminate network and inference variability.

---

## 📄 Related Specifications
- **[FC-PERF-0001: Performance Specification](../femtoclaw-spec/FC-PERF-0001-Performance_Specification.md)**
- **[FC-BENCH-0001: Benchmark Methodology](../femtoclaw-spec/FC-BENCH-0001-Benchmark_Specification.md)**

Copyright © 2026 FemtoClaw Project.
