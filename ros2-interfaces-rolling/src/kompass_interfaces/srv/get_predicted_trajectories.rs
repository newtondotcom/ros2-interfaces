use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPredictedTrajectoriesRequest {
    pub start_time: f32,
    pub horizon: f32,
    pub time_step: f32,
    pub prediction_model_type: ::std::string::String,
}

impl Default for GetPredictedTrajectoriesRequest {
    fn default() -> Self {
        GetPredictedTrajectoriesRequest {
            start_time: 0.0,
            horizon: 0.0,
            time_step: 0.0,
            prediction_model_type: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetPredictedTrajectoriesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPredictedTrajectoriesResponse {
    pub prediction_completed: bool,
    pub predicted_trajectories_topic_name: ::std::string::String,
}

impl Default for GetPredictedTrajectoriesResponse {
    fn default() -> Self {
        GetPredictedTrajectoriesResponse {
            prediction_completed: false,
            predicted_trajectories_topic_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetPredictedTrajectoriesResponse {}


pub struct GetPredictedTrajectories;
impl ros2_client::Service for GetPredictedTrajectories {
    type Request = GetPredictedTrajectoriesRequest;
    type Response = GetPredictedTrajectoriesResponse;

    fn request_type_name(&self) -> &str { "GetPredictedTrajectoriesRequest" }
    fn response_type_name(&self) -> &str { "GetPredictedTrajectoriesResponse" }
}
