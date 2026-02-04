use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RebootDxlRequest {
    pub header: crate::std_msgs::msg::Header,
}

impl Default for RebootDxlRequest {
    fn default() -> Self {
        RebootDxlRequest {
            header: crate::std_msgs::msg::Header::default(),
        }
    }
}

impl ros2_client::Message for RebootDxlRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RebootDxlResponse {
    pub result: bool,
}

impl Default for RebootDxlResponse {
    fn default() -> Self {
        RebootDxlResponse {
            result: false,
        }
    }
}

impl ros2_client::Message for RebootDxlResponse {}


pub struct RebootDxl;
impl ros2_client::Service for RebootDxl {
    type Request = RebootDxlRequest;
    type Response = RebootDxlResponse;

    fn request_type_name(&self) -> &str { "RebootDxlRequest" }
    fn response_type_name(&self) -> &str { "RebootDxlResponse" }
}
