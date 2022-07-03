use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricsConfig {
    pub enable_metric: bool,
    pub metric_reporter_list: Vec<String>,
    pub monitor_type: String,
    pub metric_level: String,
    pub predefined_metrics: Vec<String>,
    pub push_period_in_second: i32,
    pub prometheus_exporter_port: i16,
}
