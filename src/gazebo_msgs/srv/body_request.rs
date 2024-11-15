use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BodyRequestReq {
    pub body_name: ::std::string::String,
}

impl Default for BodyRequestReq {
    fn default() -> Self {
        BodyRequestReq {
            body_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for BodyRequestReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BodyRequestRes {

}

impl Default for BodyRequestRes {
    fn default() -> Self {
        BodyRequestRes {

        }
    }
}

impl ros2_client::Message for BodyRequestRes {}


pub struct BodyRequest;
impl ros2_client::Service for BodyRequest {
    type Request = BodyRequestReq;
    type Response = BodyRequestRes;

    fn request_type_name(&self) -> &str { "BodyRequestReq" }
    fn response_type_name(&self) -> &str { "BodyRequestRes" }
}
