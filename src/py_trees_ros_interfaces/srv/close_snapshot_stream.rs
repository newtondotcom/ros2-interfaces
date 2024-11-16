use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseSnapshotStreamReq {
    pub topic_name: ::std::string::String,
}

impl Default for CloseSnapshotStreamReq {
    fn default() -> Self {
        CloseSnapshotStreamReq {
            topic_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CloseSnapshotStreamReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseSnapshotStreamRes {
    pub result: bool,
}

impl Default for CloseSnapshotStreamRes {
    fn default() -> Self {
        CloseSnapshotStreamRes {
            result: false,
        }
    }
}

impl ros2_client::Message for CloseSnapshotStreamRes {}


pub struct CloseSnapshotStream;
impl ros2_client::Service for CloseSnapshotStream {
    type Request = CloseSnapshotStreamReq;
    type Response = CloseSnapshotStreamRes;

    fn request_type_name(&self) -> &str { "CloseSnapshotStreamReq" }
    fn response_type_name(&self) -> &str { "CloseSnapshotStreamRes" }
}
