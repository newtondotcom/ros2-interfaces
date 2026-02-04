use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCmdListRequest {
    pub number_of_cmds: i32,
    pub time_step: f64,
    pub follow_path: crate::kompass_interfaces::msg::FollowPathRequest,
}

impl Default for GetCmdListRequest {
    fn default() -> Self {
        GetCmdListRequest {
            number_of_cmds: 0,
            time_step: 0.0,
            follow_path: crate::kompass_interfaces::msg::FollowPathRequest::default(),
        }
    }
}

impl ros2_client::Message for GetCmdListRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCmdListResponse {
    pub cmd_list: crate::kompass_interfaces::msg::TwistArray,
}

impl Default for GetCmdListResponse {
    fn default() -> Self {
        GetCmdListResponse {
            cmd_list: crate::kompass_interfaces::msg::TwistArray::default(),
        }
    }
}

impl ros2_client::Message for GetCmdListResponse {}


pub struct GetCmdList;
impl ros2_client::Service for GetCmdList {
    type Request = GetCmdListRequest;
    type Response = GetCmdListResponse;

    fn request_type_name(&self) -> &str { "GetCmdListRequest" }
    fn response_type_name(&self) -> &str { "GetCmdListResponse" }
}
