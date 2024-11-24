#![doc = include_str!("../README.md")]
#![doc(issue_tracker_base_url = "https://github.com/anton-rs/hilo/issues/")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use clap::Parser;
use hilo_node::Node;

mod cli;
mod telemetry;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Parse arguments.
    let args = cli::NodeArgs::parse();

    // Initialize the telemetry stack.
    telemetry::init(args.metrics_port)?;
    tracing::info!(
        "Running a standalone Hilo Node. Attributes validation: {}",
        args.validation_mode
    );

    // Construct the node from the config.
    let cfg = args.into();
    let node = Node::from_config(cfg);

    // Run the node.
    if let Err(e) = node.run().await {
        eyre::bail!("[CRIT] Node failed: {:?}", e)
    }

    Ok(())
}