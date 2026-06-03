mod engine;
mod db;
mod plugins;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "smart-shell-complete")]
#[command(about = "Shell autocomplete that learns from your history")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Learn,
    Predict,
    Complete { prefix: String },
    Stats,
    Install { shell: String },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Learn => engine::learner::learn(),
        Commands::Predict => engine::predictor::predict(),
        Commands::Complete { prefix } => engine::predictor::complete(&prefix),
        Commands::Stats => db::query::show_stats(),
        Commands::Install { shell } => plugins::install(&shell),
    }
}
