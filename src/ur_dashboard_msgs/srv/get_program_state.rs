use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProgramStateReq {

}

impl Default for GetProgramStateReq {
    fn default() -> Self {
        GetProgramStateReq {

        }
    }
}

impl ros2_client::Message for GetProgramStateReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProgramStateRes {
    pub state: crate::ur_dashboard_msgs::msg::ProgramState,
    pub program_name: ::std::string::String,
    pub answer: ::std::string::String,
    pub success: bool,
}

impl Default for GetProgramStateRes {
    fn default() -> Self {
        GetProgramStateRes {
            state: crate::ur_dashboard_msgs::msg::ProgramState::default(),
            program_name: ::std::string::String::new(),
            answer: ::std::string::String::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for GetProgramStateRes {}


pub struct GetProgramState;
impl ros2_client::Service for GetProgramState {
    type Request = GetProgramStateReq;
    type Response = GetProgramStateRes;

    fn request_type_name(&self) -> &str { "GetProgramStateReq" }
    fn response_type_name(&self) -> &str { "GetProgramStateRes" }
}
