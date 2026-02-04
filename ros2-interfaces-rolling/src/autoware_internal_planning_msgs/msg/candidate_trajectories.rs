use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CandidateTrajectories {
    pub candidate_trajectories: Vec<crate::autoware_internal_planning_msgs::msg::CandidateTrajectory>,
    pub generator_info: Vec<crate::autoware_internal_planning_msgs::msg::GeneratorInfo>,
}

impl Default for CandidateTrajectories {
    fn default() -> Self {
        CandidateTrajectories {
            candidate_trajectories: Vec::new(),
            generator_info: Vec::new(),
        }
    }
}

impl ros2_client::Message for CandidateTrajectories {}
