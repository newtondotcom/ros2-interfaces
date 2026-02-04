use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoredCandidateTrajectories {
    pub scored_candidate_trajectories: Vec<crate::autoware_internal_planning_msgs::msg::ScoredCandidateTrajectory>,
    pub generator_info: Vec<crate::autoware_internal_planning_msgs::msg::GeneratorInfo>,
}

impl Default for ScoredCandidateTrajectories {
    fn default() -> Self {
        ScoredCandidateTrajectories {
            scored_candidate_trajectories: Vec::new(),
            generator_info: Vec::new(),
        }
    }
}

impl ros2_client::Message for ScoredCandidateTrajectories {}
