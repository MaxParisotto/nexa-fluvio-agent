use anyhow::Result;
use fluvio::{Fluvio, FluvioConfig, RecordKey};
use serde_json::json;
use crate::metrics::SystemMetrics;

pub async fn publish_metrics(addr: &str, topic: &str, metrics: &SystemMetrics) -> Result<()> {
    let config = FluvioConfig::new(addr);
    let fluvio = Fluvio::connect_with_config(&config).await?;
    let producer = fluvio.topic_producer(topic).await?;
    
    let metrics_json = json!({
        "cpu_usage": metrics.cpu_usage,
        "memory_used": metrics.memory_used,
        "memory_total": metrics.memory_total,
        "swap_used": metrics.swap_used,
        "swap_total": metrics.swap_total,
        "load_avg_1min": metrics.load_avg_1min,
        "load_avg_5min": metrics.load_avg_5min,
        "load_avg_15min": metrics.load_avg_15min,
        "uptime": metrics.uptime,
        "processes_count": metrics.processes_count,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    producer.send(RecordKey::NULL, metrics_json.to_string()).await?;
    Ok(())
}
