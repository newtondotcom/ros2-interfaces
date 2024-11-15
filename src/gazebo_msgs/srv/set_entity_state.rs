use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEntityStateReq {
    pub state: crate::gazebo_msgs::msg::EntityState,
}

impl Default for SetEntityStateReq {
    fn default() -> Self {
        SetEntityStateReq {
            state: crate::gazebo_msgs::msg::EntityState::default(),
        }
    }
}

impl ros2_client::Message for SetEntityStateReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEntityStateRes {
    pub success: bool,
}

impl Default for SetEntityStateRes {
    fn default() -> Self {
        SetEntityStateRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SetEntityStateRes {}


pub struct SetEntityState;
impl ros2_client::Service for SetEntityState {
    type Request = SetEntityStateReq;
    type Response = SetEntityStateRes;

    fn request_type_name(&self) -> &str { "SetEntityStateReq" }
    fn response_type_name(&self) -> &str { "SetEntityStateRes" }
}
