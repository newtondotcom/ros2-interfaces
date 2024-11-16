use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListRobotStatesInWarehouseReq {
    pub regex: ::std::string::String,
    pub robot: ::std::string::String,
}

impl Default for ListRobotStatesInWarehouseReq {
    fn default() -> Self {
        ListRobotStatesInWarehouseReq {
            regex: ::std::string::String::new(),
            robot: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ListRobotStatesInWarehouseReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListRobotStatesInWarehouseRes {
    pub states: Vec<::std::string::String>,
}

impl Default for ListRobotStatesInWarehouseRes {
    fn default() -> Self {
        ListRobotStatesInWarehouseRes {
            states: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListRobotStatesInWarehouseRes {}


pub struct ListRobotStatesInWarehouse;
impl ros2_client::Service for ListRobotStatesInWarehouse {
    type Request = ListRobotStatesInWarehouseReq;
    type Response = ListRobotStatesInWarehouseRes;

    fn request_type_name(&self) -> &str { "ListRobotStatesInWarehouseReq" }
    fn response_type_name(&self) -> &str { "ListRobotStatesInWarehouseRes" }
}
