use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandTOLLocalReq {
    pub min_pitch: f32,
    pub offset: f32,
    pub rate: f32,
    pub yaw: f32,
    pub position: crate::geometry_msgs::msg::Vector3,
}

impl Default for CommandTOLLocalReq {
    fn default() -> Self {
        CommandTOLLocalReq {
            min_pitch: 0.0,
            offset: 0.0,
            rate: 0.0,
            yaw: 0.0,
            position: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}

impl ros2_client::Message for CommandTOLLocalReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandTOLLocalRes {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandTOLLocalRes {
    fn default() -> Self {
        CommandTOLLocalRes {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for CommandTOLLocalRes {}


pub struct CommandTOLLocal;
impl ros2_client::Service for CommandTOLLocal {
    type Request = CommandTOLLocalReq;
    type Response = CommandTOLLocalRes;

    fn request_type_name(&self) -> &str { "CommandTOLLocalReq" }
    fn response_type_name(&self) -> &str { "CommandTOLLocalRes" }
}
