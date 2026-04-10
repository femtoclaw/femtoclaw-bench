use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name="femtoclaw-bench", version, about="FemtoClaw Benchmark Suite")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Run portable benchmarks without criterion.
    Run {
        /// Preset: core|all|protocol|dispatch|memory|e2e
        #[arg(long, default_value="core")]
        preset: String,
        /// Iterations per test (portable runner).
        #[arg(long, default_value_t=1000)]
        iters: u64,
        /// Optional JSON output path.
        #[arg(long)]
        json: Option<String>,
    },
    /// Measure startup latency by spawning a runtime binary.
    Startup {
        /// Path to runtime binary (e.g., ./target/release/femtoclaw)
        #[arg(long)]
        bin: String,
        /// Iterations.
        #[arg(long, default_value_t=50)]
        iterations: u32,
        /// Arguments to pass after `--`.
        #[arg(last=true)]
        args: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Run { preset, iters, json } => {
            let results = femtoclaw_bench::benchmarks::portable::run_preset(&preset, iters).await?;
            femtoclaw_bench::report::print(&results);
            if let Some(path) = json {
                femtoclaw_bench::report::write_json(&results, &path)?;
            }
            Ok(())
        }
        Cmd::Startup { bin, iterations, args } => {
            let results = femtoclaw_bench::benchmarks::startup::run_startup(&bin, &args, iterations).await?;
            femtoclaw_bench::report::print(&results);
            Ok(())
        }
    }
}
