use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEntityStateReq {
    pub name: ::std::string::String,
    pub reference_frame: ::std::string::String,
}

impl Default for GetEntityStateReq {
    fn default() -> Self {
        GetEntityStateReq {
            name: ::std::string::String::new(),
            reference_frame: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetEntityStateReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEntityStateRes {
    pub header: crate::std_msgs::msg::Header,
    pub state: crate::gazebo_msgs::msg::EntityState,
    pub success: bool,
}

impl Default for GetEntityStateRes {
    fn default() -> Self {
        GetEntityStateRes {
            header: crate::std_msgs::msg::Header::default(),
            state: crate::gazebo_msgs::msg::EntityState::default(),
            success: false,
        }
    }
}

impl ros2_client::Message for GetEntityStateRes {}


pub struct GetEntityState;
impl ros2_client::Service for GetEntityState {
    type Request = GetEntityStateReq;
    type Response = GetEntityStateRes;

    fn request_type_name(&self) -> &str { "GetEntityStateReq" }
    fn response_type_name(&self) -> &str { "GetEntityStateRes" }
}
