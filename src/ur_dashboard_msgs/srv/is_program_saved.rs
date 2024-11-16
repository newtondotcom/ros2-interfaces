use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsProgramSavedReq {

}

impl Default for IsProgramSavedReq {
    fn default() -> Self {
        IsProgramSavedReq {

        }
    }
}

impl ros2_client::Message for IsProgramSavedReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsProgramSavedRes {
    pub answer: ::std::string::String,
    pub program_name: ::std::string::String,
    pub program_saved: bool,
    pub success: bool,
}

impl Default for IsProgramSavedRes {
    fn default() -> Self {
        IsProgramSavedRes {
            answer: ::std::string::String::new(),
            program_name: ::std::string::String::new(),
            program_saved: false,
            success: false,
        }
    }
}

impl ros2_client::Message for IsProgramSavedRes {}


pub struct IsProgramSaved;
impl ros2_client::Service for IsProgramSaved {
    type Request = IsProgramSavedReq;
    type Response = IsProgramSavedRes;

    fn request_type_name(&self) -> &str { "IsProgramSavedReq" }
    fn response_type_name(&self) -> &str { "IsProgramSavedRes" }
}
