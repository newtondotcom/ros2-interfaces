use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyFactorArray {
    pub header: crate::std_msgs::msg::Header,
    pub factors: Vec<crate::autoware_internal_planning_msgs::msg::SafetyFactor>,
    pub is_safe: bool,
    pub detail: ::std::string::String,
}

impl Default for SafetyFactorArray {
    fn default() -> Self {
        SafetyFactorArray {
            header: crate::std_msgs::msg::Header::default(),
            factors: Vec::new(),
            is_safe: false,
            detail: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SafetyFactorArray {}
