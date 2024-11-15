use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelocalizeFromGNSSReq {

}

impl Default for RelocalizeFromGNSSReq {
    fn default() -> Self {
        RelocalizeFromGNSSReq {

        }
    }
}

impl ros2_client::Message for RelocalizeFromGNSSReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelocalizeFromGNSSRes {
    pub accepted: bool,
}

impl Default for RelocalizeFromGNSSRes {
    fn default() -> Self {
        RelocalizeFromGNSSRes {
            accepted: false,
        }
    }
}

impl ros2_client::Message for RelocalizeFromGNSSRes {}


pub struct RelocalizeFromGNSS;
impl ros2_client::Service for RelocalizeFromGNSS {
    type Request = RelocalizeFromGNSSReq;
    type Response = RelocalizeFromGNSSRes;

    fn request_type_name(&self) -> &str { "RelocalizeFromGNSSReq" }
    fn response_type_name(&self) -> &str { "RelocalizeFromGNSSRes" }
}
