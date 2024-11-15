use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PopupReq {
    pub message: ::std::string::String,
}

impl Default for PopupReq {
    fn default() -> Self {
        PopupReq {
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for PopupReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PopupRes {
    pub answer: ::std::string::String,
    pub success: bool,
}

impl Default for PopupRes {
    fn default() -> Self {
        PopupRes {
            answer: ::std::string::String::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for PopupRes {}


pub struct Popup;
impl ros2_client::Service for Popup {
    type Request = PopupReq;
    type Response = PopupRes;

    fn request_type_name(&self) -> &str { "PopupReq" }
    fn response_type_name(&self) -> &str { "PopupRes" }
}
