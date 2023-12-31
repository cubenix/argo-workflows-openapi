use serde::{Deserialize, Serialize};

/// Prometheus is a prometheus metric to be emitted.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Prometheus {
    #[serde(rename = "counter", skip_serializing_if = "Option::is_none")]
    pub counter: Option<Box<super::Counter>>,
    
    #[serde(rename = "gauge", skip_serializing_if = "Option::is_none")]
    pub gauge: Option<Box<super::Gauge>>,
    
    /// Help is a string that describes the metric.
    #[serde(rename = "help")]
    pub help: String,
    
    #[serde(rename = "histogram", skip_serializing_if = "Option::is_none")]
    pub histogram: Option<Box<super::Histogram>>,
    
    /// Labels is a list of metric labels.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<super::MetricLabel>>,
    
    /// Name is the name of the metric.
    #[serde(rename = "name")]
    pub name: String,
    
    /// When is a conditional statement that decides when to emit the metric.
    #[serde(rename = "when", skip_serializing_if = "Option::is_none")]
    pub when: Option<String>,
}

impl Prometheus {
    pub fn new(help: String, name: String) -> Prometheus {
        Prometheus {
            counter: None,
            gauge: None,
            help,
            histogram: None,
            labels: None,
            name,
            when: None,
        }
    }
}
