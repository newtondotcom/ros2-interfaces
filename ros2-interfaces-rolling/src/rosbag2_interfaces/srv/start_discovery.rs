use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartDiscoveryRequest {

}

impl Default for StartDiscoveryRequest {
    fn default() -> Self {
        StartDiscoveryRequest {

        }
    }
}

impl ros2_client::Message for StartDiscoveryRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartDiscoveryResponse {
    pub return_code: i32,
    pub error_string: ::std::string::String,
}

impl Default for StartDiscoveryResponse {
    fn default() -> Self {
        StartDiscoveryResponse {
            return_code: 0,
            error_string: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for StartDiscoveryResponse {}


pub struct StartDiscovery;
impl ros2_client::Service for StartDiscovery {
    type Request = StartDiscoveryRequest;
    type Response = StartDiscoveryResponse;

    fn request_type_name(&self) -> &str { "StartDiscoveryRequest" }
    fn response_type_name(&self) -> &str { "StartDiscoveryResponse" }
}
