use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanningFactorArray {
    pub header: crate::std_msgs::msg::Header,
    pub factors: Vec<crate::autoware_internal_planning_msgs::msg::PlanningFactor>,
}

impl Default for PlanningFactorArray {
    fn default() -> Self {
        PlanningFactorArray {
            header: crate::std_msgs::msg::Header::default(),
            factors: Vec::new(),
        }
    }
}

impl ros2_client::Message for PlanningFactorArray {}
