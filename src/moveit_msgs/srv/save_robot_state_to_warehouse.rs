use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveRobotStateToWarehouseReq {
    pub name: ::std::string::String,
    pub robot: ::std::string::String,
    pub state: crate::moveit_msgs::msg::RobotState,
}

impl Default for SaveRobotStateToWarehouseReq {
    fn default() -> Self {
        SaveRobotStateToWarehouseReq {
            name: ::std::string::String::new(),
            robot: ::std::string::String::new(),
            state: crate::moveit_msgs::msg::RobotState::default(),
        }
    }
}

impl ros2_client::Message for SaveRobotStateToWarehouseReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveRobotStateToWarehouseRes {
    pub success: bool,
}

impl Default for SaveRobotStateToWarehouseRes {
    fn default() -> Self {
        SaveRobotStateToWarehouseRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SaveRobotStateToWarehouseRes {}


pub struct SaveRobotStateToWarehouse;
impl ros2_client::Service for SaveRobotStateToWarehouse {
    type Request = SaveRobotStateToWarehouseReq;
    type Response = SaveRobotStateToWarehouseRes;

    fn request_type_name(&self) -> &str { "SaveRobotStateToWarehouseReq" }
    fn response_type_name(&self) -> &str { "SaveRobotStateToWarehouseRes" }
}
