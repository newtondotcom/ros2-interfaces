use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CleanupLocalGridsReq {
    pub radius: i32,
    pub filter_scans: bool,
}

impl Default for CleanupLocalGridsReq {
    fn default() -> Self {
        CleanupLocalGridsReq {
            radius: 0,
            filter_scans: false,
        }
    }
}

impl ros2_client::Message for CleanupLocalGridsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CleanupLocalGridsRes {
    pub modified: i32,
}

impl Default for CleanupLocalGridsRes {
    fn default() -> Self {
        CleanupLocalGridsRes {
            modified: 0,
        }
    }
}

impl ros2_client::Message for CleanupLocalGridsRes {}


pub struct CleanupLocalGrids;
impl ros2_client::Service for CleanupLocalGrids {
    type Request = CleanupLocalGridsReq;
    type Response = CleanupLocalGridsRes;

    fn request_type_name(&self) -> &str { "CleanupLocalGridsReq" }
    fn response_type_name(&self) -> &str { "CleanupLocalGridsRes" }
}
