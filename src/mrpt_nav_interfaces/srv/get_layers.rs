use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLayersReq {

}

impl Default for GetLayersReq {
    fn default() -> Self {
        GetLayersReq {

        }
    }
}

impl ros2_client::Message for GetLayersReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLayersRes {
    pub layers: Vec<::std::string::String>,
}

impl Default for GetLayersRes {
    fn default() -> Self {
        GetLayersRes {
            layers: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetLayersRes {}


pub struct GetLayers;
impl ros2_client::Service for GetLayers {
    type Request = GetLayersReq;
    type Response = GetLayersRes;

    fn request_type_name(&self) -> &str { "GetLayersReq" }
    fn response_type_name(&self) -> &str { "GetLayersRes" }
}
