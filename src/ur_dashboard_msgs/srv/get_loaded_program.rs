use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLoadedProgramReq {

}

impl Default for GetLoadedProgramReq {
    fn default() -> Self {
        GetLoadedProgramReq {

        }
    }
}

impl ros2_client::Message for GetLoadedProgramReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLoadedProgramRes {
    pub answer: ::std::string::String,
    pub program_name: ::std::string::String,
    pub success: bool,
}

impl Default for GetLoadedProgramRes {
    fn default() -> Self {
        GetLoadedProgramRes {
            answer: ::std::string::String::new(),
            program_name: ::std::string::String::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for GetLoadedProgramRes {}


pub struct GetLoadedProgram;
impl ros2_client::Service for GetLoadedProgram {
    type Request = GetLoadedProgramReq;
    type Response = GetLoadedProgramRes;

    fn request_type_name(&self) -> &str { "GetLoadedProgramReq" }
    fn response_type_name(&self) -> &str { "GetLoadedProgramRes" }
}
