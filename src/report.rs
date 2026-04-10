use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub unit: String,
    pub iters: u64,
    pub total_ns: f64,
    pub ns_per_op: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteResult {
    pub suite: String,
    pub metrics: Vec<Metric>,
    pub notes: Vec<String>,
}

pub fn print(res: &SuiteResult) {
    println!("== {} ==", res.suite);
    for m in &res.metrics {
        println!(
            "- {:32} {:12.2} {}  (iters: {})",
            m.name, m.ns_per_op, m.unit, m.iters
        );
    }
    for n in &res.notes {
        println!("  note: {}", n);
    }
}

pub fn write_json(res: &SuiteResult, path: &str) -> anyhow::Result<()> {
    let s = serde_json::to_string_pretty(res)?;
    std::fs::write(path, s)?;
    Ok(())
}
