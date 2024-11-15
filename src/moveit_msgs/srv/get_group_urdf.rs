use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGroupUrdfReq {
    pub group_name: ::std::string::String,
}

impl Default for GetGroupUrdfReq {
    fn default() -> Self {
        GetGroupUrdfReq {
            group_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetGroupUrdfReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGroupUrdfRes {
    pub error_code: crate::moveit_msgs::msg::MoveItErrorCodes,
    pub urdf_string: ::std::string::String,
}

impl Default for GetGroupUrdfRes {
    fn default() -> Self {
        GetGroupUrdfRes {
            error_code: crate::moveit_msgs::msg::MoveItErrorCodes::default(),
            urdf_string: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetGroupUrdfRes {}


pub struct GetGroupUrdf;
impl ros2_client::Service for GetGroupUrdf {
    type Request = GetGroupUrdfReq;
    type Response = GetGroupUrdfRes;

    fn request_type_name(&self) -> &str { "GetGroupUrdfReq" }
    fn response_type_name(&self) -> &str { "GetGroupUrdfRes" }
}
