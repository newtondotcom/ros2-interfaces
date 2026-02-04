use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelocalizeFromStateEstimatorRequest {

}

impl Default for RelocalizeFromStateEstimatorRequest {
    fn default() -> Self {
        RelocalizeFromStateEstimatorRequest {

        }
    }
}

impl ros2_client::Message for RelocalizeFromStateEstimatorRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelocalizeFromStateEstimatorResponse {
    pub accepted: bool,
}

impl Default for RelocalizeFromStateEstimatorResponse {
    fn default() -> Self {
        RelocalizeFromStateEstimatorResponse {
            accepted: false,
        }
    }
}

impl ros2_client::Message for RelocalizeFromStateEstimatorResponse {}


pub struct RelocalizeFromStateEstimator;
impl ros2_client::Service for RelocalizeFromStateEstimator {
    type Request = RelocalizeFromStateEstimatorRequest;
    type Response = RelocalizeFromStateEstimatorResponse;

    fn request_type_name(&self) -> &str { "RelocalizeFromStateEstimatorRequest" }
    fn response_type_name(&self) -> &str { "RelocalizeFromStateEstimatorResponse" }
}
