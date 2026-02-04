use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitializeLocalizationRequest {
    pub pose_with_covariance: Vec<crate::geometry_msgs::msg::PoseWithCovarianceStamped>,
    pub method: u8,
}

impl InitializeLocalizationRequest {
    pub const AUTO: u8 = 0;
    pub const DIRECT: u8 = 1;
}

impl Default for InitializeLocalizationRequest {
    fn default() -> Self {
        InitializeLocalizationRequest {
            pose_with_covariance: Vec::new(),
            method: 0,
        }
    }
}

impl ros2_client::Message for InitializeLocalizationRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitializeLocalizationResponse {
    pub status: crate::autoware_common_msgs::msg::ResponseStatus,
}

impl InitializeLocalizationResponse {
    pub const ERROR_UNSAFE: u16 = 1;
    pub const ERROR_GNSS_SUPPORT: u16 = 2;
    pub const ERROR_GNSS: u16 = 3;
    pub const ERROR_ESTIMATION: u16 = 4;
}

impl Default for InitializeLocalizationResponse {
    fn default() -> Self {
        InitializeLocalizationResponse {
            status: crate::autoware_common_msgs::msg::ResponseStatus::default(),
        }
    }
}

impl ros2_client::Message for InitializeLocalizationResponse {}


pub struct InitializeLocalization;
impl ros2_client::Service for InitializeLocalization {
    type Request = InitializeLocalizationRequest;
    type Response = InitializeLocalizationResponse;

    fn request_type_name(&self) -> &str { "InitializeLocalizationRequest" }
    fn response_type_name(&self) -> &str { "InitializeLocalizationResponse" }
}
