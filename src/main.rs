mod client;
mod config;
mod data;
mod exporter;
mod processor;
mod receiver;

use clap::{Parser, Subcommand};
use client::AuthType;
use exporter::Exporter;
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
        /// Authentication method to use (none, basic, apikey, etc.)
        #[arg(
            default_value = "none",
            help = "Authentication method",
            long,
            value_enum
        )]
        input_auth: AuthType,
        /// Authentication method to use (none, basic, apikey, etc.)
        #[arg(
            default_value = "none",
            help = "Authentication method",
            long,
            value_enum
        )]
        output_auth: AuthType,
    },
    /// Setup Elasticsearch assets for visualizing output data
    Setup {
        /// Elasticsearch host to setup datastream assets in
        #[arg(help = "Elasticsearch host to setup assets in")]
        host: String,
        /// Authentication method to use (none, basic, apikey, etc.)
        #[arg(
            default_value = "none",
            help = "Authentication method",
            long,
            value_enum
        )]
        auth: AuthType,
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
    config::load();

    // Initialize logger
    let env = env_logger::Env::default().filter_or("LOG_LEVEL", config::LOG_LEVEL);
    env_logger::Builder::from_env(env)
        .format_timestamp_millis()
        .init();

    std::panic::set_hook(Box::new(|panic| {
        // Use the error level to log the panic
        log::error!("{}", panic);
    }));

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
        Commands::Eval {
            input,
            input_auth,
            output,
            output_auth,
        } => {
            let reciever = Receiver::parse(input, input_auth).expect("Failed to parse input");
            let exporter =
                Exporter::parse(output.as_ref(), output_auth).expect("Failed to parse output");
            let docs = processor::evaluate_shard_balance(&reciever)
                .await
                .expect("Failed to evaluate shard balance");

            match exporter.is_connected().await {
                true => log::info!("Connected to {exporter}"),
                false => log::warn!("Failed to connect to {exporter}"),
            };
            let doc_count = exporter.write(docs).await.expect("Error writing docs");
            log::info!("Wrote {doc_count} docs to {exporter}");
        }
        Commands::Setup { host, auth } => {
            log::info!("Setting up eshipster datastreams on {host}");
            let exporter = Exporter::parse(Some(host), auth).expect("Error parsing output");
            client::setup::elasticsearch(&exporter)
                .await
                .expect("Error on Elasticsearch setup");
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
