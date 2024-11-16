use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SplitBagfileReq {

}

impl Default for SplitBagfileReq {
    fn default() -> Self {
        SplitBagfileReq {

        }
    }
}

impl ros2_client::Message for SplitBagfileReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SplitBagfileRes {

}

impl Default for SplitBagfileRes {
    fn default() -> Self {
        SplitBagfileRes {

        }
    }
}

impl ros2_client::Message for SplitBagfileRes {}


pub struct SplitBagfile;
impl ros2_client::Service for SplitBagfile {
    type Request = SplitBagfileReq;
    type Response = SplitBagfileRes;

    fn request_type_name(&self) -> &str { "SplitBagfileReq" }
    fn response_type_name(&self) -> &str { "SplitBagfileRes" }
}
