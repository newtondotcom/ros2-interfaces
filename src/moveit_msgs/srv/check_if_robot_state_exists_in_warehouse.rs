use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckIfRobotStateExistsInWarehouseReq {
    pub name: ::std::string::String,
    pub robot: ::std::string::String,
}

impl Default for CheckIfRobotStateExistsInWarehouseReq {
    fn default() -> Self {
        CheckIfRobotStateExistsInWarehouseReq {
            name: ::std::string::String::new(),
            robot: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CheckIfRobotStateExistsInWarehouseReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckIfRobotStateExistsInWarehouseRes {
    pub exists: bool,
}

impl Default for CheckIfRobotStateExistsInWarehouseRes {
    fn default() -> Self {
        CheckIfRobotStateExistsInWarehouseRes {
            exists: false,
        }
    }
}

impl ros2_client::Message for CheckIfRobotStateExistsInWarehouseRes {}


pub struct CheckIfRobotStateExistsInWarehouse;
impl ros2_client::Service for CheckIfRobotStateExistsInWarehouse {
    type Request = CheckIfRobotStateExistsInWarehouseReq;
    type Response = CheckIfRobotStateExistsInWarehouseRes;

    fn request_type_name(&self) -> &str { "CheckIfRobotStateExistsInWarehouseReq" }
    fn response_type_name(&self) -> &str { "CheckIfRobotStateExistsInWarehouseRes" }
}
