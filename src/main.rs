mod client;
mod data;
mod env;
mod exporter;
mod processor;
mod receiver;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use data::ShardDoc;
use exporter::Exporter;
use processor::extract_shard_docs;
use receiver::Receiver;

// Define command line arguments
#[derive(Parser)]
#[command(name = "eshipster")]
#[command(
    about = "Elasticsearch high-performance strategy enforcer (eshipster)",
    long_about = "A tool to optimize Elasticsearch performance by enforcing ideal shard balance."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Calculate ideal shard balance and make API calls to enforce it
    Balance {
        /// The host to collect shard stats from and execute API commands on
        #[arg(help = "The input to collect shard stats from")]
        host: String,
        /// An alternative output to send the shard documents to
        #[arg(help = "An alternative output to send the shard documents to")]
        output: Option<String>,
    },
    /// Collect shard stats and calculate the ideal shard balance
    Eval {
        /// The input to collect shard stats from
        #[arg(help = "The input to collect shard stats from")]
        input: String,
        /// The output to send the shard documents to
        #[arg(help = "The output to send the shard documents to")]
        output: Option<String>,
    },
    /// Setup Elasticsearch assets for visualizing output data
    Setup {
        /// Elasticsearch host to setup datastream assets in
        #[arg(help = "Elasticsearch host to setup datastream assets in")]
        host: String,
    },
    /// Continuously monitor and enforce shard balance on an Elasticsearch cluster
    Watch {
        /// Elasticsearch endpoint to monitor shard balance
        #[arg(help = "Elasticsearch endpoint to monitor shard balance")]
        host: String,
        /// An alternative output to send the shard documents to
        #[arg(help = "An alternative output to send the shard documents to")]
        output: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    // Initialize logger
    let env = env_logger::Env::default().filter_or("LOG_LEVEL", env::LOG_LEVEL);
    env_logger::Builder::from_env(env)
        .format_timestamp_millis()
        .init();

    // Use clap to parse command line arguments
    let cli = Cli::parse();

    match &cli.command {
        Commands::Balance { host, output } => {
            log::info!("Balancing shards on {host}");
            match output {
                Some(output) => log::info!("Sending docs to {output}"),
                None => log::info!("Sending docs to stdout"),
            }
            let _output = output.as_ref().unwrap_or(host);
            todo!("balance shards on a cluster.");
        }
        Commands::Eval { input, output } => {
            let reciever = Receiver::parse(input).expect("Failed to parse input");
            let exporter = Exporter::parse(output.as_ref()).expect("Failed to parse output");
            let docs = evaluate_shard_balance(&reciever)
                .await
                .expect("Failed to evaluate shard balance");
            log::info!("Writing docs to {exporter}");
            exporter.write(docs).await.expect("Failed to write docs");
        }
        Commands::Setup { host } => {
            log::info!("Setting up eshipster datastreams on {host}");
            todo!("setup eshipster data streams.");
        }
        Commands::Watch { host, output } => {
            log::info!("Watching shard balance on {host}");
            match output {
                Some(output) => log::info!("Sending docs to {output}"),
                None => log::info!("Sending docs back to {host}"),
            }
            todo!("the watch service.");
        }
    }
}

async fn evaluate_shard_balance(reciever: &Receiver) -> Result<Vec<ShardDoc>> {
    log::info!("Evaluating shard balance of {reciever}");
    let indices_stats = reciever.read_indices_stats().await?;
    log::warn!("TODO: perform calculations");
    let shard_docs = extract_shard_docs(indices_stats)?;
    Ok(shard_docs)
}
