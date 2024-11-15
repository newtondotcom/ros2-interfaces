use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplyLinkWrenchReq {
    pub link_name: ::std::string::String,
    pub reference_frame: ::std::string::String,
    pub reference_point: crate::geometry_msgs::msg::Point,
    pub wrench: crate::geometry_msgs::msg::Wrench,
    pub start_time: crate::builtin_interfaces::msg::Time,
    pub duration: crate::builtin_interfaces::msg::Duration,
}

impl Default for ApplyLinkWrenchReq {
    fn default() -> Self {
        ApplyLinkWrenchReq {
            link_name: ::std::string::String::new(),
            reference_frame: ::std::string::String::new(),
            reference_point: crate::geometry_msgs::msg::Point::default(),
            wrench: crate::geometry_msgs::msg::Wrench::default(),
            start_time: crate::builtin_interfaces::msg::Time::default(),
            duration: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}

impl ros2_client::Message for ApplyLinkWrenchReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplyLinkWrenchRes {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for ApplyLinkWrenchRes {
    fn default() -> Self {
        ApplyLinkWrenchRes {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ApplyLinkWrenchRes {}


pub struct ApplyLinkWrench;
impl ros2_client::Service for ApplyLinkWrench {
    type Request = ApplyLinkWrenchReq;
    type Response = ApplyLinkWrenchRes;

    fn request_type_name(&self) -> &str { "ApplyLinkWrenchReq" }
    fn response_type_name(&self) -> &str { "ApplyLinkWrenchRes" }
}
