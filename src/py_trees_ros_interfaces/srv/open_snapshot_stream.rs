use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenSnapshotStreamReq {
    pub topic_name: ::std::string::String,
    pub parameters: crate::py_trees_ros_interfaces::msg::SnapshotStreamParameters,
}

impl Default for OpenSnapshotStreamReq {
    fn default() -> Self {
        OpenSnapshotStreamReq {
            topic_name: ::std::string::String::new(),
            parameters: crate::py_trees_ros_interfaces::msg::SnapshotStreamParameters::default(),
        }
    }
}

impl ros2_client::Message for OpenSnapshotStreamReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenSnapshotStreamRes {
    pub topic_name: ::std::string::String,
}

impl Default for OpenSnapshotStreamRes {
    fn default() -> Self {
        OpenSnapshotStreamRes {
            topic_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for OpenSnapshotStreamRes {}


pub struct OpenSnapshotStream;
impl ros2_client::Service for OpenSnapshotStream {
    type Request = OpenSnapshotStreamReq;
    type Response = OpenSnapshotStreamRes;

    fn request_type_name(&self) -> &str { "OpenSnapshotStreamReq" }
    fn response_type_name(&self) -> &str { "OpenSnapshotStreamRes" }
}
