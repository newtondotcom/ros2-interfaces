use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReconfigureSnapshotStreamReq {
    pub topic_name: ::std::string::String,
    pub parameters: crate::py_trees_ros_interfaces::msg::SnapshotStreamParameters,
}

impl Default for ReconfigureSnapshotStreamReq {
    fn default() -> Self {
        ReconfigureSnapshotStreamReq {
            topic_name: ::std::string::String::new(),
            parameters: crate::py_trees_ros_interfaces::msg::SnapshotStreamParameters::default(),
        }
    }
}

impl ros2_client::Message for ReconfigureSnapshotStreamReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReconfigureSnapshotStreamRes {
    pub result: bool,
}

impl Default for ReconfigureSnapshotStreamRes {
    fn default() -> Self {
        ReconfigureSnapshotStreamRes {
            result: false,
        }
    }
}

impl ros2_client::Message for ReconfigureSnapshotStreamRes {}


pub struct ReconfigureSnapshotStream;
impl ros2_client::Service for ReconfigureSnapshotStream {
    type Request = ReconfigureSnapshotStreamReq;
    type Response = ReconfigureSnapshotStreamRes;

    fn request_type_name(&self) -> &str { "ReconfigureSnapshotStreamReq" }
    fn response_type_name(&self) -> &str { "ReconfigureSnapshotStreamRes" }
}
