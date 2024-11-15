use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManageLifecycleNodesReq {
    pub command: u8,
}

impl ManageLifecycleNodesReq {
    pub const STARTUP: u8 = 0;
    pub const PAUSE: u8 = 1;
    pub const RESUME: u8 = 2;
    pub const RESET: u8 = 3;
    pub const SHUTDOWN: u8 = 4;
    pub const CONFIGURE: u8 = 5;
    pub const CLEANUP: u8 = 6;
}

impl Default for ManageLifecycleNodesReq {
    fn default() -> Self {
        ManageLifecycleNodesReq {
            command: 0,
        }
    }
}

impl ros2_client::Message for ManageLifecycleNodesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManageLifecycleNodesRes {
    pub success: bool,
}

impl Default for ManageLifecycleNodesRes {
    fn default() -> Self {
        ManageLifecycleNodesRes {
            success: false,
        }
    }
}

impl ros2_client::Message for ManageLifecycleNodesRes {}


pub struct ManageLifecycleNodes;
impl ros2_client::Service for ManageLifecycleNodes {
    type Request = ManageLifecycleNodesReq;
    type Response = ManageLifecycleNodesRes;

    fn request_type_name(&self) -> &str { "ManageLifecycleNodesReq" }
    fn response_type_name(&self) -> &str { "ManageLifecycleNodesRes" }
}
