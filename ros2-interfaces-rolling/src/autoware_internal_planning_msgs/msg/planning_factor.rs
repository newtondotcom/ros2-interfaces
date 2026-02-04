use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanningFactor {
    pub module: ::std::string::String,
    pub is_driving_forward: bool,
    pub control_points: Vec<crate::autoware_internal_planning_msgs::msg::ControlPoint>,
    pub behavior: u16,
    pub detail: ::std::string::String,
    pub safety_factors: crate::autoware_internal_planning_msgs::msg::SafetyFactorArray,
}

impl PlanningFactor {
    pub const UNKNOWN: u16 = 0;
    pub const NONE: u16 = 1;
    pub const SLOW_DOWN: u16 = 2;
    pub const STOP: u16 = 3;
    pub const SHIFT_LEFT: u16 = 4;
    pub const SHIFT_RIGHT: u16 = 5;
    pub const TURN_LEFT: u16 = 6;
    pub const TURN_RIGHT: u16 = 7;
}

impl Default for PlanningFactor {
    fn default() -> Self {
        PlanningFactor {
            module: ::std::string::String::new(),
            is_driving_forward: false,
            control_points: Vec::new(),
            behavior: 0,
            detail: ::std::string::String::new(),
            safety_factors: crate::autoware_internal_planning_msgs::msg::SafetyFactorArray::default(),
        }
    }
}

impl ros2_client::Message for PlanningFactor {}
