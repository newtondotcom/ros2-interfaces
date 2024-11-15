use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveLabelReq {
    pub label: ::std::string::String,
}

impl Default for RemoveLabelReq {
    fn default() -> Self {
        RemoveLabelReq {
            label: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for RemoveLabelReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveLabelRes {

}

impl Default for RemoveLabelRes {
    fn default() -> Self {
        RemoveLabelRes {

        }
    }
}

impl ros2_client::Message for RemoveLabelRes {}


pub struct RemoveLabel;
impl ros2_client::Service for RemoveLabel {
    type Request = RemoveLabelReq;
    type Response = RemoveLabelRes;

    fn request_type_name(&self) -> &str { "RemoveLabelReq" }
    fn response_type_name(&self) -> &str { "RemoveLabelRes" }
}
