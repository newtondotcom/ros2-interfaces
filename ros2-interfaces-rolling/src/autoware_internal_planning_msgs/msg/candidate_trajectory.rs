use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CandidateTrajectory {
    pub header: crate::std_msgs::msg::Header,
    pub generator_id: crate::unique_identifier_msgs::msg::UUID,
    pub points: Vec<crate::autoware_planning_msgs::msg::TrajectoryPoint>,
}

impl Default for CandidateTrajectory {
    fn default() -> Self {
        CandidateTrajectory {
            header: crate::std_msgs::msg::Header::default(),
            generator_id: crate::unique_identifier_msgs::msg::UUID::default(),
            points: Vec::new(),
        }
    }
}

impl ros2_client::Message for CandidateTrajectory {}
