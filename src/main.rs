mod metrics;
mod fluvio_pub;

use std::time::Duration;
use clap::Parser;
use tokio::time::sleep;
use metrics::collect_metrics;
use fluvio_pub::publish_metrics;

#[derive(Parser)]
#[command(name = "nexa-metrics")]
#[command(about = "System metrics collector for Fluvio", version = "0.1.0")]
struct Cli {
    #[arg(short, long, default_value = "system-metrics")]
    topic: String,
    
    #[arg(short, long, default_value_t = 5)]
    interval: u64,
    
    #[arg(long = "fluvio_addr", default_value = "localhost:9003")]
    fluvio_addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    
    loop {
        let metrics = collect_metrics();
        match publish_metrics(&args.fluvio_addr, &args.topic, &metrics).await {
            Ok(_) => (),
            Err(e) => eprintln!("Error publishing metrics: {}", e),
        }
        sleep(Duration::from_secs(args.interval)).await;
    }
}
