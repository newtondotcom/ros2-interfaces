use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListClipsEnvsRequest {

}

impl Default for ListClipsEnvsRequest {
    fn default() -> Self {
        ListClipsEnvsRequest {

        }
    }
}

impl ros2_client::Message for ListClipsEnvsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListClipsEnvsResponse {
    pub success: bool,
    pub envs: Vec<::std::string::String>,
    pub error: ::std::string::String,
}

impl Default for ListClipsEnvsResponse {
    fn default() -> Self {
        ListClipsEnvsResponse {
            success: false,
            envs: Vec::new(),
            error: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ListClipsEnvsResponse {}


pub struct ListClipsEnvs;
impl ros2_client::Service for ListClipsEnvs {
    type Request = ListClipsEnvsRequest;
    type Response = ListClipsEnvsResponse;

    fn request_type_name(&self) -> &str { "ListClipsEnvsRequest" }
    fn response_type_name(&self) -> &str { "ListClipsEnvsResponse" }
}
