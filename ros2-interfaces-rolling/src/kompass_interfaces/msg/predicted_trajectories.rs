use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictedTrajectories {
    pub header: crate::std_msgs::msg::Header,
    pub start_time: f32,
    pub horizon: f32,
    pub time_step: f32,
    pub object_ids: Vec<i64>,
    pub trajectories: Vec<crate::nav_msgs::msg::Path>,
}

impl Default for PredictedTrajectories {
    fn default() -> Self {
        PredictedTrajectories {
            header: crate::std_msgs::msg::Header::default(),
            start_time: 0.0,
            horizon: 0.0,
            time_step: 0.0,
            object_ids: Vec::new(),
            trajectories: Vec::new(),
        }
    }
}

impl ros2_client::Message for PredictedTrajectories {}
