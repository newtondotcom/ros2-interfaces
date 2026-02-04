use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMultiStateValidityRequest {
    pub robot_state: crate::moveit_msgs::msg::RobotState,
    pub joint_states: Vec<crate::sensor_msgs::msg::JointState>,
    pub group_name: ::std::string::String,
    pub constraints: crate::moveit_msgs::msg::Constraints,
}

impl Default for GetMultiStateValidityRequest {
    fn default() -> Self {
        GetMultiStateValidityRequest {
            robot_state: crate::moveit_msgs::msg::RobotState::default(),
            joint_states: Vec::new(),
            group_name: ::std::string::String::new(),
            constraints: crate::moveit_msgs::msg::Constraints::default(),
        }
    }
}

impl ros2_client::Message for GetMultiStateValidityRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMultiStateValidityResponse {
    pub valid: bool,
    pub contacts: Vec<crate::moveit_msgs::msg::ContactInformation>,
    pub cost_sources: Vec<crate::moveit_msgs::msg::CostSource>,
    pub constraint_result: Vec<crate::moveit_msgs::msg::ConstraintEvalResult>,
}

impl Default for GetMultiStateValidityResponse {
    fn default() -> Self {
        GetMultiStateValidityResponse {
            valid: false,
            contacts: Vec::new(),
            cost_sources: Vec::new(),
            constraint_result: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetMultiStateValidityResponse {}


pub struct GetMultiStateValidity;
impl ros2_client::Service for GetMultiStateValidity {
    type Request = GetMultiStateValidityRequest;
    type Response = GetMultiStateValidityResponse;

    fn request_type_name(&self) -> &str { "GetMultiStateValidityRequest" }
    fn response_type_name(&self) -> &str { "GetMultiStateValidityResponse" }
}
