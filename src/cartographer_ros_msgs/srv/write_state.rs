use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WriteStateReq {
    pub filename: ::std::string::String,
    pub include_unfinished_submaps: bool,
}

impl Default for WriteStateReq {
    fn default() -> Self {
        WriteStateReq {
            filename: ::std::string::String::new(),
            include_unfinished_submaps: false,
        }
    }
}

impl ros2_client::Message for WriteStateReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WriteStateRes {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
}

impl Default for WriteStateRes {
    fn default() -> Self {
        WriteStateRes {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
        }
    }
}

impl ros2_client::Message for WriteStateRes {}


pub struct WriteState;
impl ros2_client::Service for WriteState {
    type Request = WriteStateReq;
    type Response = WriteStateRes;

    fn request_type_name(&self) -> &str { "WriteStateReq" }
    fn response_type_name(&self) -> &str { "WriteStateRes" }
}
