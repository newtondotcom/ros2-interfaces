use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameGraphReq {

}

impl Default for FrameGraphReq {
    fn default() -> Self {
        FrameGraphReq {

        }
    }
}

impl ros2_client::Message for FrameGraphReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameGraphRes {
    pub frame_yaml: ::std::string::String,
}

impl Default for FrameGraphRes {
    fn default() -> Self {
        FrameGraphRes {
            frame_yaml: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for FrameGraphRes {}


pub struct FrameGraph;
impl ros2_client::Service for FrameGraph {
    type Request = FrameGraphReq;
    type Response = FrameGraphRes;

    fn request_type_name(&self) -> &str { "FrameGraphReq" }
    fn response_type_name(&self) -> &str { "FrameGraphRes" }
}
