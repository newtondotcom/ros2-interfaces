use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPositionIKReq {
    pub ik_request: crate::moveit_msgs::msg::PositionIKRequest,
}

impl Default for GetPositionIKReq {
    fn default() -> Self {
        GetPositionIKReq {
            ik_request: crate::moveit_msgs::msg::PositionIKRequest::default(),
        }
    }
}

impl ros2_client::Message for GetPositionIKReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPositionIKRes {
    pub solution: crate::moveit_msgs::msg::RobotState,
    pub error_code: crate::moveit_msgs::msg::MoveItErrorCodes,
}

impl Default for GetPositionIKRes {
    fn default() -> Self {
        GetPositionIKRes {
            solution: crate::moveit_msgs::msg::RobotState::default(),
            error_code: crate::moveit_msgs::msg::MoveItErrorCodes::default(),
        }
    }
}

impl ros2_client::Message for GetPositionIKRes {}


pub struct GetPositionIK;
impl ros2_client::Service for GetPositionIK {
    type Request = GetPositionIKReq;
    type Response = GetPositionIKRes;

    fn request_type_name(&self) -> &str { "GetPositionIKReq" }
    fn response_type_name(&self) -> &str { "GetPositionIKRes" }
}
