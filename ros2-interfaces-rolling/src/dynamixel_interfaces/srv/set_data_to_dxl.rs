use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDataToDxlRequest {
    pub header: crate::std_msgs::msg::Header,
    pub id: u8,
    pub item_name: ::std::string::String,
    pub item_data: u32,
}

impl Default for SetDataToDxlRequest {
    fn default() -> Self {
        SetDataToDxlRequest {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            item_name: ::std::string::String::new(),
            item_data: 0,
        }
    }
}

impl ros2_client::Message for SetDataToDxlRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDataToDxlResponse {
    pub result: bool,
}

impl Default for SetDataToDxlResponse {
    fn default() -> Self {
        SetDataToDxlResponse {
            result: false,
        }
    }
}

impl ros2_client::Message for SetDataToDxlResponse {}


pub struct SetDataToDxl;
impl ros2_client::Service for SetDataToDxl {
    type Request = SetDataToDxlRequest;
    type Response = SetDataToDxlResponse;

    fn request_type_name(&self) -> &str { "SetDataToDxlRequest" }
    fn response_type_name(&self) -> &str { "SetDataToDxlResponse" }
}
