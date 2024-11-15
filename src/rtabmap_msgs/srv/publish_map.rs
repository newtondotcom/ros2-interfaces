use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishMapReq {
    pub global_map: bool,
    pub optimized: bool,
    pub graph_only: bool,
}

impl Default for PublishMapReq {
    fn default() -> Self {
        PublishMapReq {
            global_map: false,
            optimized: false,
            graph_only: false,
        }
    }
}

impl ros2_client::Message for PublishMapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishMapRes {

}

impl Default for PublishMapRes {
    fn default() -> Self {
        PublishMapRes {

        }
    }
}

impl ros2_client::Message for PublishMapRes {}


pub struct PublishMap;
impl ros2_client::Service for PublishMap {
    type Request = PublishMapReq;
    type Response = PublishMapRes;

    fn request_type_name(&self) -> &str { "PublishMapReq" }
    fn response_type_name(&self) -> &str { "PublishMapRes" }
}
