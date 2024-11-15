use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenBlackboardStreamReq {
    pub variables: Vec<::std::string::String>,
    pub filter_on_visited_path: bool,
    pub with_activity_stream: bool,
}

impl Default for OpenBlackboardStreamReq {
    fn default() -> Self {
        OpenBlackboardStreamReq {
            variables: Vec::new(),
            filter_on_visited_path: false,
            with_activity_stream: false,
        }
    }
}

impl ros2_client::Message for OpenBlackboardStreamReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenBlackboardStreamRes {
    pub topic: ::std::string::String,
}

impl Default for OpenBlackboardStreamRes {
    fn default() -> Self {
        OpenBlackboardStreamRes {
            topic: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for OpenBlackboardStreamRes {}


pub struct OpenBlackboardStream;
impl ros2_client::Service for OpenBlackboardStream {
    type Request = OpenBlackboardStreamReq;
    type Response = OpenBlackboardStreamRes;

    fn request_type_name(&self) -> &str { "OpenBlackboardStreamReq" }
    fn response_type_name(&self) -> &str { "OpenBlackboardStreamRes" }
}
