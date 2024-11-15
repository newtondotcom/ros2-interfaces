use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRobotStateFromWarehouseReq {
    pub name: ::std::string::String,
    pub robot: ::std::string::String,
}

impl Default for GetRobotStateFromWarehouseReq {
    fn default() -> Self {
        GetRobotStateFromWarehouseReq {
            name: ::std::string::String::new(),
            robot: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetRobotStateFromWarehouseReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRobotStateFromWarehouseRes {
    pub state: crate::moveit_msgs::msg::RobotState,
}

impl Default for GetRobotStateFromWarehouseRes {
    fn default() -> Self {
        GetRobotStateFromWarehouseRes {
            state: crate::moveit_msgs::msg::RobotState::default(),
        }
    }
}

impl ros2_client::Message for GetRobotStateFromWarehouseRes {}


pub struct GetRobotStateFromWarehouse;
impl ros2_client::Service for GetRobotStateFromWarehouse {
    type Request = GetRobotStateFromWarehouseReq;
    type Response = GetRobotStateFromWarehouseRes;

    fn request_type_name(&self) -> &str { "GetRobotStateFromWarehouseReq" }
    fn response_type_name(&self) -> &str { "GetRobotStateFromWarehouseRes" }
}
