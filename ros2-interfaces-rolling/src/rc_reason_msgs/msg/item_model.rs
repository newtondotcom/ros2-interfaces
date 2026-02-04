use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemModel {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub unknown: crate::rc_reason_msgs::msg::RangeBox,
    pub rectangle: crate::rc_reason_msgs::msg::RangeRectangle,
    pub textured_box: crate::rc_reason_msgs::msg::TexturedBox,
}

impl ItemModel {
    pub const UNKNOWN: &'static str = "UNKNOWN";
    pub const RECTANGLE: &'static str = "RECTANGLE";
    pub const TEXTURED_BOX: &'static str = "TEXTURED_BOX";
    pub const BAG: &'static str = "BAG";
    pub const CONSUMER_GOODS: &'static str = "CONSUMER_GOODS";
    pub const SHEET_METAL: &'static str = "SHEET_METAL";
}

impl Default for ItemModel {
    fn default() -> Self {
        ItemModel {
            type_: ::std::string::String::new(),
            unknown: crate::rc_reason_msgs::msg::RangeBox::default(),
            rectangle: crate::rc_reason_msgs::msg::RangeRectangle::default(),
            textured_box: crate::rc_reason_msgs::msg::TexturedBox::default(),
        }
    }
}

impl ros2_client::Message for ItemModel {}
