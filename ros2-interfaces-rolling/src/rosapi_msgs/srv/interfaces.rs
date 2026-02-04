use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfacesRequest {

}

impl Default for InterfacesRequest {
    fn default() -> Self {
        InterfacesRequest {

        }
    }
}

impl ros2_client::Message for InterfacesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfacesResponse {
    pub interfaces: Vec<::std::string::String>,
}

impl Default for InterfacesResponse {
    fn default() -> Self {
        InterfacesResponse {
            interfaces: Vec::new(),
        }
    }
}

impl ros2_client::Message for InterfacesResponse {}


pub struct Interfaces;
impl ros2_client::Service for Interfaces {
    type Request = InterfacesRequest;
    type Response = InterfacesResponse;

    fn request_type_name(&self) -> &str { "InterfacesRequest" }
    fn response_type_name(&self) -> &str { "InterfacesResponse" }
}
