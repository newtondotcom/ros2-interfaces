use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricArray {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub metric_array: Vec<crate::autoware_internal_metric_msgs::msg::Metric>,
}

impl Default for MetricArray {
    fn default() -> Self {
        MetricArray {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            metric_array: Vec::new(),
        }
    }
}

impl ros2_client::Message for MetricArray {}
