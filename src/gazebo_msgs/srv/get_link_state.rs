use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLinkStateReq {
    pub link_name: ::std::string::String,
    pub reference_frame: ::std::string::String,
}

impl Default for GetLinkStateReq {
    fn default() -> Self {
        GetLinkStateReq {
            link_name: ::std::string::String::new(),
            reference_frame: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetLinkStateReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLinkStateRes {
    pub link_state: crate::gazebo_msgs::msg::LinkState,
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for GetLinkStateRes {
    fn default() -> Self {
        GetLinkStateRes {
            link_state: crate::gazebo_msgs::msg::LinkState::default(),
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetLinkStateRes {}


pub struct GetLinkState;
impl ros2_client::Service for GetLinkState {
    type Request = GetLinkStateReq;
    type Response = GetLinkStateRes;

    fn request_type_name(&self) -> &str { "GetLinkStateReq" }
    fn response_type_name(&self) -> &str { "GetLinkStateRes" }
}
