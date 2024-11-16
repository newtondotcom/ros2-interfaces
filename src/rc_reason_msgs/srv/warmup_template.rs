use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarmupTemplateReq {
    pub template_id: ::std::string::String,
}

impl Default for WarmupTemplateReq {
    fn default() -> Self {
        WarmupTemplateReq {
            template_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for WarmupTemplateReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarmupTemplateRes {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for WarmupTemplateRes {
    fn default() -> Self {
        WarmupTemplateRes {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for WarmupTemplateRes {}


pub struct WarmupTemplate;
impl ros2_client::Service for WarmupTemplate {
    type Request = WarmupTemplateReq;
    type Response = WarmupTemplateRes;

    fn request_type_name(&self) -> &str { "WarmupTemplateReq" }
    fn response_type_name(&self) -> &str { "WarmupTemplateRes" }
}
