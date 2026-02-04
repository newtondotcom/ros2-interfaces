use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPredictedOccupanciesRequest {
    pub start_time: f32,
    pub horizon: f32,
    pub time_step: f32,
    pub prediction_model_type: ::std::string::String,
}

impl Default for GetPredictedOccupanciesRequest {
    fn default() -> Self {
        GetPredictedOccupanciesRequest {
            start_time: 0.0,
            horizon: 0.0,
            time_step: 0.0,
            prediction_model_type: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetPredictedOccupanciesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPredictedOccupanciesResponse {
    pub prediction_completed: bool,
    pub predicted_occupancies_topic_name: ::std::string::String,
}

impl Default for GetPredictedOccupanciesResponse {
    fn default() -> Self {
        GetPredictedOccupanciesResponse {
            prediction_completed: false,
            predicted_occupancies_topic_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetPredictedOccupanciesResponse {}


pub struct GetPredictedOccupancies;
impl ros2_client::Service for GetPredictedOccupancies {
    type Request = GetPredictedOccupanciesRequest;
    type Response = GetPredictedOccupanciesResponse;

    fn request_type_name(&self) -> &str { "GetPredictedOccupanciesRequest" }
    fn response_type_name(&self) -> &str { "GetPredictedOccupanciesResponse" }
}
