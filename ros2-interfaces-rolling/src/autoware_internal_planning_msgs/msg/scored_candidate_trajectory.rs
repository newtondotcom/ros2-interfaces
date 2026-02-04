use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoredCandidateTrajectory {
    pub candidate_trajectory: crate::autoware_internal_planning_msgs::msg::CandidateTrajectory,
    pub score: f32,
}

impl Default for ScoredCandidateTrajectory {
    fn default() -> Self {
        ScoredCandidateTrajectory {
            candidate_trajectory: crate::autoware_internal_planning_msgs::msg::CandidateTrajectory::default(),
            score: 0.0,
        }
    }
}

impl ros2_client::Message for ScoredCandidateTrajectory {}
