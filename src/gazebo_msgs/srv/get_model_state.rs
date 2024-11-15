use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetModelStateReq {
    pub model_name: ::std::string::String,
    pub relative_entity_name: ::std::string::String,
}

impl Default for GetModelStateReq {
    fn default() -> Self {
        GetModelStateReq {
            model_name: ::std::string::String::new(),
            relative_entity_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetModelStateReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetModelStateRes {
    pub header: crate::std_msgs::msg::Header,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub twist: crate::geometry_msgs::msg::Twist,
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for GetModelStateRes {
    fn default() -> Self {
        GetModelStateRes {
            header: crate::std_msgs::msg::Header::default(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            twist: crate::geometry_msgs::msg::Twist::default(),
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetModelStateRes {}


pub struct GetModelState;
impl ros2_client::Service for GetModelState {
    type Request = GetModelStateReq;
    type Response = GetModelStateRes;

    fn request_type_name(&self) -> &str { "GetModelStateReq" }
    fn response_type_name(&self) -> &str { "GetModelStateRes" }
}
