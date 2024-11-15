use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLabelReq {
    pub node_id: i32,
    pub node_label: ::std::string::String,
}

impl Default for SetLabelReq {
    fn default() -> Self {
        SetLabelReq {
            node_id: 0,
            node_label: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetLabelReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLabelRes {

}

impl Default for SetLabelRes {
    fn default() -> Self {
        SetLabelRes {

        }
    }
}

impl ros2_client::Message for SetLabelRes {}


pub struct SetLabel;
impl ros2_client::Service for SetLabel {
    type Request = SetLabelReq;
    type Response = SetLabelRes;

    fn request_type_name(&self) -> &str { "SetLabelReq" }
    fn response_type_name(&self) -> &str { "SetLabelRes" }
}
