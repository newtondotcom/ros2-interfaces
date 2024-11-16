use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionServersReq {

}

impl Default for GetActionServersReq {
    fn default() -> Self {
        GetActionServersReq {

        }
    }
}

impl ros2_client::Message for GetActionServersReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionServersRes {
    pub action_servers: Vec<::std::string::String>,
}

impl Default for GetActionServersRes {
    fn default() -> Self {
        GetActionServersRes {
            action_servers: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetActionServersRes {}


pub struct GetActionServers;
impl ros2_client::Service for GetActionServers {
    type Request = GetActionServersReq;
    type Response = GetActionServersRes;

    fn request_type_name(&self) -> &str { "GetActionServersReq" }
    fn response_type_name(&self) -> &str { "GetActionServersRes" }
}
