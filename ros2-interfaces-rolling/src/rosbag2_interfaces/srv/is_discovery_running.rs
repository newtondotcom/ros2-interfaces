use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsDiscoveryRunningRequest {

}

impl Default for IsDiscoveryRunningRequest {
    fn default() -> Self {
        IsDiscoveryRunningRequest {

        }
    }
}

impl ros2_client::Message for IsDiscoveryRunningRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsDiscoveryRunningResponse {
    pub running: bool,
}

impl Default for IsDiscoveryRunningResponse {
    fn default() -> Self {
        IsDiscoveryRunningResponse {
            running: false,
        }
    }
}

impl ros2_client::Message for IsDiscoveryRunningResponse {}


pub struct IsDiscoveryRunning;
impl ros2_client::Service for IsDiscoveryRunning {
    type Request = IsDiscoveryRunningRequest;
    type Response = IsDiscoveryRunningResponse;

    fn request_type_name(&self) -> &str { "IsDiscoveryRunningRequest" }
    fn response_type_name(&self) -> &str { "IsDiscoveryRunningResponse" }
}
