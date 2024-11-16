use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotReq {

}

impl Default for SnapshotReq {
    fn default() -> Self {
        SnapshotReq {

        }
    }
}

impl ros2_client::Message for SnapshotReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotRes {
    pub success: bool,
}

impl Default for SnapshotRes {
    fn default() -> Self {
        SnapshotRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SnapshotRes {}


pub struct Snapshot;
impl ros2_client::Service for Snapshot {
    type Request = SnapshotReq;
    type Response = SnapshotRes;

    fn request_type_name(&self) -> &str { "SnapshotReq" }
    fn response_type_name(&self) -> &str { "SnapshotRes" }
}
