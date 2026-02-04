use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoalRequest {

}

impl Default for GoalRequest {
    fn default() -> Self {
        GoalRequest {

        }
    }
}

impl ros2_client::Message for GoalRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoalResponse {
    pub pose_x: f32,
    pub pose_y: f32,
    pub success: bool,
}

impl Default for GoalResponse {
    fn default() -> Self {
        GoalResponse {
            pose_x: 0.0,
            pose_y: 0.0,
            success: false,
        }
    }
}

impl ros2_client::Message for GoalResponse {}


pub struct Goal;
impl ros2_client::Service for Goal {
    type Request = GoalRequest;
    type Response = GoalResponse;

    fn request_type_name(&self) -> &str { "GoalRequest" }
    fn response_type_name(&self) -> &str { "GoalResponse" }
}
