use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PoseWithCovarianceStampedRequest {
    pub pose_with_covariance: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for PoseWithCovarianceStampedRequest {
    fn default() -> Self {
        PoseWithCovarianceStampedRequest {
            pose_with_covariance: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

impl ros2_client::Message for PoseWithCovarianceStampedRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PoseWithCovarianceStampedResponse {
    pub success: bool,
    pub reliable: bool,
    pub pose_with_covariance: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for PoseWithCovarianceStampedResponse {
    fn default() -> Self {
        PoseWithCovarianceStampedResponse {
            success: false,
            reliable: false,
            pose_with_covariance: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

impl ros2_client::Message for PoseWithCovarianceStampedResponse {}


pub struct PoseWithCovarianceStamped;
impl ros2_client::Service for PoseWithCovarianceStamped {
    type Request = PoseWithCovarianceStampedRequest;
    type Response = PoseWithCovarianceStampedResponse;

    fn request_type_name(&self) -> &str { "PoseWithCovarianceStampedRequest" }
    fn response_type_name(&self) -> &str { "PoseWithCovarianceStampedResponse" }
}
