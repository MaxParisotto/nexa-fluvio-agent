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
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    producer.send(RecordKey::NULL, metrics_json.to_string()).await?;
    Ok(())
}
