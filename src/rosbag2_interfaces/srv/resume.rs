use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResumeReq {

}

impl Default for ResumeReq {
    fn default() -> Self {
        ResumeReq {

        }
    }
}

impl ros2_client::Message for ResumeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResumeRes {

}

impl Default for ResumeRes {
    fn default() -> Self {
        ResumeRes {

        }
    }
}

impl ros2_client::Message for ResumeRes {}


pub struct Resume;
impl ros2_client::Service for Resume {
    type Request = ResumeReq;
    type Response = ResumeRes;

    fn request_type_name(&self) -> &str { "ResumeReq" }
    fn response_type_name(&self) -> &str { "ResumeRes" }
}
