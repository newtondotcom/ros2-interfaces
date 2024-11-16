use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsProgramRunningReq {

}

impl Default for IsProgramRunningReq {
    fn default() -> Self {
        IsProgramRunningReq {

        }
    }
}

impl ros2_client::Message for IsProgramRunningReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsProgramRunningRes {
    pub answer: ::std::string::String,
    pub program_running: bool,
    pub success: bool,
}

impl Default for IsProgramRunningRes {
    fn default() -> Self {
        IsProgramRunningRes {
            answer: ::std::string::String::new(),
            program_running: false,
            success: false,
        }
    }
}

impl ros2_client::Message for IsProgramRunningRes {}


pub struct IsProgramRunning;
impl ros2_client::Service for IsProgramRunning {
    type Request = IsProgramRunningReq;
    type Response = IsProgramRunningRes;

    fn request_type_name(&self) -> &str { "IsProgramRunningReq" }
    fn response_type_name(&self) -> &str { "IsProgramRunningRes" }
}
