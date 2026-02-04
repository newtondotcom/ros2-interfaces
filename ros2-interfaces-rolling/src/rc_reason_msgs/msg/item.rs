use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub uuid: ::std::string::String,
    pub grasp_uuids: Vec<::std::string::String>,
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub rectangle: crate::rc_reason_msgs::msg::Rectangle,
    pub bounding_box: crate::rc_reason_msgs::msg::Box,
    pub pose: crate::geometry_msgs::msg::PoseStamped,
    pub template_id: ::std::string::String,
    pub view_uuid: ::std::string::String,
    pub view_name: ::std::string::String,
    pub view_pose_set: bool,
}

impl Item {
    pub const RECTANGLE: &'static str = "RECTANGLE";
    pub const TEXTURED_RECTANGLE: &'static str = "TEXTURED_RECTANGLE";
    pub const BAG: &'static str = "BAG";
    pub const CONSUMER_GOODS: &'static str = "CONSUMER_GOODS";
    pub const SHEET_METAL: &'static str = "SHEET_METAL";
}

impl Default for Item {
    fn default() -> Self {
        Item {
            uuid: ::std::string::String::new(),
            grasp_uuids: Vec::new(),
            type_: ::std::string::String::new(),
            rectangle: crate::rc_reason_msgs::msg::Rectangle::default(),
            bounding_box: crate::rc_reason_msgs::msg::Box::default(),
            pose: crate::geometry_msgs::msg::PoseStamped::default(),
            template_id: ::std::string::String::new(),
            view_uuid: ::std::string::String::new(),
            view_name: ::std::string::String::new(),
            view_pose_set: false,
        }
    }
}

impl ros2_client::Message for Item {}
