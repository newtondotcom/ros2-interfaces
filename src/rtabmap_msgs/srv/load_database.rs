use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadDatabaseReq {
    pub database_path: ::std::string::String,
    pub clear: bool,
}

impl Default for LoadDatabaseReq {
    fn default() -> Self {
        LoadDatabaseReq {
            database_path: ::std::string::String::new(),
            clear: false,
        }
    }
}

impl ros2_client::Message for LoadDatabaseReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadDatabaseRes {

}

impl Default for LoadDatabaseRes {
    fn default() -> Self {
        LoadDatabaseRes {

        }
    }
}

impl ros2_client::Message for LoadDatabaseRes {}


pub struct LoadDatabase;
impl ros2_client::Service for LoadDatabase {
    type Request = LoadDatabaseReq;
    type Response = LoadDatabaseRes;

    fn request_type_name(&self) -> &str { "LoadDatabaseReq" }
    fn response_type_name(&self) -> &str { "LoadDatabaseRes" }
}
