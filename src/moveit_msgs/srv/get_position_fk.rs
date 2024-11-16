use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPositionFKReq {
    pub header: crate::std_msgs::msg::Header,
    pub fk_link_names: Vec<::std::string::String>,
    pub robot_state: crate::moveit_msgs::msg::RobotState,
}

impl Default for GetPositionFKReq {
    fn default() -> Self {
        GetPositionFKReq {
            header: crate::std_msgs::msg::Header::default(),
            fk_link_names: Vec::new(),
            robot_state: crate::moveit_msgs::msg::RobotState::default(),
        }
    }
}

impl ros2_client::Message for GetPositionFKReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPositionFKRes {
    pub pose_stamped: Vec<crate::geometry_msgs::msg::PoseStamped>,
    pub fk_link_names: Vec<::std::string::String>,
    pub error_code: crate::moveit_msgs::msg::MoveItErrorCodes,
}

impl Default for GetPositionFKRes {
    fn default() -> Self {
        GetPositionFKRes {
            pose_stamped: Vec::new(),
            fk_link_names: Vec::new(),
            error_code: crate::moveit_msgs::msg::MoveItErrorCodes::default(),
        }
    }
}

impl ros2_client::Message for GetPositionFKRes {}


pub struct GetPositionFK;
impl ros2_client::Service for GetPositionFK {
    type Request = GetPositionFKReq;
    type Response = GetPositionFKRes;

    fn request_type_name(&self) -> &str { "GetPositionFKReq" }
    fn response_type_name(&self) -> &str { "GetPositionFKRes" }
}
