use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddMapvizDisplayReq {
    pub name: ::std::string::String,
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub draw_order: i32,
    pub visible: bool,
    pub properties: Vec<crate::marti_common_msgs::msg::KeyValue>,
}

impl Default for AddMapvizDisplayReq {
    fn default() -> Self {
        AddMapvizDisplayReq {
            name: ::std::string::String::new(),
            type_: ::std::string::String::new(),
            draw_order: 0,
            visible: false,
            properties: Vec::new(),
        }
    }
}

impl ros2_client::Message for AddMapvizDisplayReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddMapvizDisplayRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for AddMapvizDisplayRes {
    fn default() -> Self {
        AddMapvizDisplayRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for AddMapvizDisplayRes {}


pub struct AddMapvizDisplay;
impl ros2_client::Service for AddMapvizDisplay {
    type Request = AddMapvizDisplayReq;
    type Response = AddMapvizDisplayRes;

    fn request_type_name(&self) -> &str { "AddMapvizDisplayReq" }
    fn response_type_name(&self) -> &str { "AddMapvizDisplayRes" }
}
