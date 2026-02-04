use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXNavSvin {
    pub header: crate::std_msgs::msg::Header,
    pub version: u8,
    pub itow: u32,
    pub dur: u32,
    pub mean_x: i32,
    pub mean_y: i32,
    pub mean_z: i32,
    pub mean_x_hp: i8,
    pub mean_y_hp: i8,
    pub mean_z_hp: i8,
    pub mean_acc: u32,
    pub obs: u32,
    pub valid: bool,
    pub active: bool,
}

impl Default for UBXNavSvin {
    fn default() -> Self {
        UBXNavSvin {
            header: crate::std_msgs::msg::Header::default(),
            version: 0,
            itow: 0,
            dur: 0,
            mean_x: 0,
            mean_y: 0,
            mean_z: 0,
            mean_x_hp: 0,
            mean_y_hp: 0,
            mean_z_hp: 0,
            mean_acc: 0,
            obs: 0,
            valid: false,
            active: false,
        }
    }
}

impl ros2_client::Message for UBXNavSvin {}
