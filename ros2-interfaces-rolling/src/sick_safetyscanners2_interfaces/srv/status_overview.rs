use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusOverviewRequest {

}

impl Default for StatusOverviewRequest {
    fn default() -> Self {
        StatusOverviewRequest {

        }
    }
}

impl ros2_client::Message for StatusOverviewRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusOverviewResponse {
    pub version_c_version: ::std::string::String,
    pub version_major_version_number: u8,
    pub version_minor_version_number: u8,
    pub version_release_number: u8,
    pub device_state: u8,
    pub config_state: u8,
    pub application_state: u8,
    pub current_time_power_on_count: u32,
    pub current_time: ::std::string::String,
    pub current_time_time: u32,
    pub current_time_date: u16,
    pub error_info_code: u32,
    pub error_info_time: ::std::string::String,
    pub error_info_time_time: u32,
    pub error_info_time_date: u16,
}

impl Default for StatusOverviewResponse {
    fn default() -> Self {
        StatusOverviewResponse {
            version_c_version: ::std::string::String::new(),
            version_major_version_number: 0,
            version_minor_version_number: 0,
            version_release_number: 0,
            device_state: 0,
            config_state: 0,
            application_state: 0,
            current_time_power_on_count: 0,
            current_time: ::std::string::String::new(),
            current_time_time: 0,
            current_time_date: 0,
            error_info_code: 0,
            error_info_time: ::std::string::String::new(),
            error_info_time_time: 0,
            error_info_time_date: 0,
        }
    }
}

impl ros2_client::Message for StatusOverviewResponse {}


pub struct StatusOverview;
impl ros2_client::Service for StatusOverview {
    type Request = StatusOverviewRequest;
    type Response = StatusOverviewResponse;

    fn request_type_name(&self) -> &str { "StatusOverviewRequest" }
    fn response_type_name(&self) -> &str { "StatusOverviewResponse" }
}
