use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnloadNodeReq {
    pub unique_id: u64,
}

impl Default for UnloadNodeReq {
    fn default() -> Self {
        UnloadNodeReq {
            unique_id: 0,
        }
    }
}

impl ros2_client::Message for UnloadNodeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnloadNodeRes {
    pub success: bool,
    pub error_message: ::std::string::String,
}

impl Default for UnloadNodeRes {
    fn default() -> Self {
        UnloadNodeRes {
            success: false,
            error_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for UnloadNodeRes {}


pub struct UnloadNode;
impl ros2_client::Service for UnloadNode {
    type Request = UnloadNodeReq;
    type Response = UnloadNodeRes;

    fn request_type_name(&self) -> &str { "UnloadNodeReq" }
    fn response_type_name(&self) -> &str { "UnloadNodeRes" }
}
