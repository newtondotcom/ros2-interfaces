use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenameRobotStateInWarehouseReq {
    pub old_name: ::std::string::String,
    pub new_name: ::std::string::String,
    pub robot: ::std::string::String,
}

impl Default for RenameRobotStateInWarehouseReq {
    fn default() -> Self {
        RenameRobotStateInWarehouseReq {
            old_name: ::std::string::String::new(),
            new_name: ::std::string::String::new(),
            robot: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for RenameRobotStateInWarehouseReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenameRobotStateInWarehouseRes {

}

impl Default for RenameRobotStateInWarehouseRes {
    fn default() -> Self {
        RenameRobotStateInWarehouseRes {

        }
    }
}

impl ros2_client::Message for RenameRobotStateInWarehouseRes {}


pub struct RenameRobotStateInWarehouse;
impl ros2_client::Service for RenameRobotStateInWarehouse {
    type Request = RenameRobotStateInWarehouseReq;
    type Response = RenameRobotStateInWarehouseRes;

    fn request_type_name(&self) -> &str { "RenameRobotStateInWarehouseReq" }
    fn response_type_name(&self) -> &str { "RenameRobotStateInWarehouseRes" }
}
