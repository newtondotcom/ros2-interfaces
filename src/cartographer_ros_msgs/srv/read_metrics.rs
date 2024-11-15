use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadMetricsReq {

}

impl Default for ReadMetricsReq {
    fn default() -> Self {
        ReadMetricsReq {

        }
    }
}

impl ros2_client::Message for ReadMetricsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadMetricsRes {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
    pub metric_families: Vec<crate::cartographer_ros_msgs::msg::MetricFamily>,
    pub timestamp: crate::builtin_interfaces::msg::Time,
}

impl Default for ReadMetricsRes {
    fn default() -> Self {
        ReadMetricsRes {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
            metric_families: Vec::new(),
            timestamp: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for ReadMetricsRes {}


pub struct ReadMetrics;
impl ros2_client::Service for ReadMetrics {
    type Request = ReadMetricsReq;
    type Response = ReadMetricsRes;

    fn request_type_name(&self) -> &str { "ReadMetricsReq" }
    fn response_type_name(&self) -> &str { "ReadMetricsRes" }
}
