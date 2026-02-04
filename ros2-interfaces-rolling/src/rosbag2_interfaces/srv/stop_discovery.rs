use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopDiscoveryRequest {

}

impl Default for StopDiscoveryRequest {
    fn default() -> Self {
        StopDiscoveryRequest {

        }
    }
}

impl ros2_client::Message for StopDiscoveryRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopDiscoveryResponse {
    pub return_code: i32,
    pub error_string: ::std::string::String,
}

impl Default for StopDiscoveryResponse {
    fn default() -> Self {
        StopDiscoveryResponse {
            return_code: 0,
            error_string: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for StopDiscoveryResponse {}


pub struct StopDiscovery;
impl ros2_client::Service for StopDiscovery {
    type Request = StopDiscoveryRequest;
    type Response = StopDiscoveryResponse;

    fn request_type_name(&self) -> &str { "StopDiscoveryRequest" }
    fn response_type_name(&self) -> &str { "StopDiscoveryResponse" }
}
