use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReloadDockDatabaseReq {
    pub filepath: ::std::string::String,
}

impl Default for ReloadDockDatabaseReq {
    fn default() -> Self {
        ReloadDockDatabaseReq {
            filepath: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ReloadDockDatabaseReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReloadDockDatabaseRes {
    pub success: bool,
}

impl Default for ReloadDockDatabaseRes {
    fn default() -> Self {
        ReloadDockDatabaseRes {
            success: false,
        }
    }
}

impl ros2_client::Message for ReloadDockDatabaseRes {}


pub struct ReloadDockDatabase;
impl ros2_client::Service for ReloadDockDatabase {
    type Request = ReloadDockDatabaseReq;
    type Response = ReloadDockDatabaseRes;

    fn request_type_name(&self) -> &str { "ReloadDockDatabaseReq" }
    fn response_type_name(&self) -> &str { "ReloadDockDatabaseRes" }
}
