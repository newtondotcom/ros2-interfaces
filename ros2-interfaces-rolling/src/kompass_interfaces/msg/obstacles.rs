use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Obstacles {
    pub header: crate::std_msgs::msg::Header,
    pub info: crate::nav_msgs::msg::MapMetaData,
    pub obstacle_type: Vec<i32>,
    pub x_global: Vec<f32>,
    pub y_global: Vec<f32>,
    pub x_local: Vec<f32>,
    pub y_local: Vec<f32>,
    pub i_grid: Vec<i32>,
    pub j_grid: Vec<i32>,
    pub occupied_zone: Vec<f32>,
    pub class_id: Vec<i32>,
    pub object_id: Vec<i32>,
    pub vx: Vec<f32>,
    pub vy: Vec<f32>,
}

impl Default for Obstacles {
    fn default() -> Self {
        Obstacles {
            header: crate::std_msgs::msg::Header::default(),
            info: crate::nav_msgs::msg::MapMetaData::default(),
            obstacle_type: Vec::new(),
            x_global: Vec::new(),
            y_global: Vec::new(),
            x_local: Vec::new(),
            y_local: Vec::new(),
            i_grid: Vec::new(),
            j_grid: Vec::new(),
            occupied_zone: Vec::new(),
            class_id: Vec::new(),
            object_id: Vec::new(),
            vx: Vec::new(),
            vy: Vec::new(),
        }
    }
}

impl ros2_client::Message for Obstacles {}
