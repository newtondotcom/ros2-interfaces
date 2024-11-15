use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReloadControllerLibrariesReq {
    pub force_kill: bool,
}

impl Default for ReloadControllerLibrariesReq {
    fn default() -> Self {
        ReloadControllerLibrariesReq {
            force_kill: false,
        }
    }
}

impl ros2_client::Message for ReloadControllerLibrariesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReloadControllerLibrariesRes {
    pub ok: bool,
}

impl Default for ReloadControllerLibrariesRes {
    fn default() -> Self {
        ReloadControllerLibrariesRes {
            ok: false,
        }
    }
}

impl ros2_client::Message for ReloadControllerLibrariesRes {}


pub struct ReloadControllerLibraries;
impl ros2_client::Service for ReloadControllerLibraries {
    type Request = ReloadControllerLibrariesReq;
    type Response = ReloadControllerLibrariesRes;

    fn request_type_name(&self) -> &str { "ReloadControllerLibrariesReq" }
    fn response_type_name(&self) -> &str { "ReloadControllerLibrariesRes" }
}
