use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDataFromDxlRequest {
    pub header: crate::std_msgs::msg::Header,
    pub id: u8,
    pub item_name: ::std::string::String,
    pub timeout_sec: f64,
}

impl Default for GetDataFromDxlRequest {
    fn default() -> Self {
        GetDataFromDxlRequest {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            item_name: ::std::string::String::new(),
            timeout_sec: 0.0,
        }
    }
}

impl ros2_client::Message for GetDataFromDxlRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDataFromDxlResponse {
    pub item_data: u32,
    pub result: bool,
}

impl Default for GetDataFromDxlResponse {
    fn default() -> Self {
        GetDataFromDxlResponse {
            item_data: 0,
            result: false,
        }
    }
}

impl ros2_client::Message for GetDataFromDxlResponse {}


pub struct GetDataFromDxl;
impl ros2_client::Service for GetDataFromDxl {
    type Request = GetDataFromDxlRequest;
    type Response = GetDataFromDxlResponse;

    fn request_type_name(&self) -> &str { "GetDataFromDxlRequest" }
    fn response_type_name(&self) -> &str { "GetDataFromDxlResponse" }
}
