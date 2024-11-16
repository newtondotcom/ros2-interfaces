use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRobotStateFromWarehouseReq {
    pub name: ::std::string::String,
    pub robot: ::std::string::String,
}

impl Default for DeleteRobotStateFromWarehouseReq {
    fn default() -> Self {
        DeleteRobotStateFromWarehouseReq {
            name: ::std::string::String::new(),
            robot: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DeleteRobotStateFromWarehouseReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRobotStateFromWarehouseRes {

}

impl Default for DeleteRobotStateFromWarehouseRes {
    fn default() -> Self {
        DeleteRobotStateFromWarehouseRes {

        }
    }
}

impl ros2_client::Message for DeleteRobotStateFromWarehouseRes {}


pub struct DeleteRobotStateFromWarehouse;
impl ros2_client::Service for DeleteRobotStateFromWarehouse {
    type Request = DeleteRobotStateFromWarehouseReq;
    type Response = DeleteRobotStateFromWarehouseRes;

    fn request_type_name(&self) -> &str { "DeleteRobotStateFromWarehouseReq" }
    fn response_type_name(&self) -> &str { "DeleteRobotStateFromWarehouseRes" }
}
