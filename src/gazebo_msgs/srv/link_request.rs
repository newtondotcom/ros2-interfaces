use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkRequestReq {
    pub link_name: ::std::string::String,
}

impl Default for LinkRequestReq {
    fn default() -> Self {
        LinkRequestReq {
            link_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LinkRequestReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkRequestRes {

}

impl Default for LinkRequestRes {
    fn default() -> Self {
        LinkRequestRes {

        }
    }
}

impl ros2_client::Message for LinkRequestRes {}


pub struct LinkRequest;
impl ros2_client::Service for LinkRequest {
    type Request = LinkRequestReq;
    type Response = LinkRequestRes;

    fn request_type_name(&self) -> &str { "LinkRequestReq" }
    fn response_type_name(&self) -> &str { "LinkRequestRes" }
}
