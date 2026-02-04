use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Grasp {
    pub id: ::std::string::String,
    pub uuid: ::std::string::String,
    pub match_uuid: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::PoseStamped,
    pub priority: i8,
    pub gripper_id: ::std::string::String,
    pub stroke_per_finger_approach_mm: f64,
    pub stroke_per_finger_grasp_mm: f64,
    pub collision_checked: bool,
}

impl Default for Grasp {
    fn default() -> Self {
        Grasp {
            id: ::std::string::String::new(),
            uuid: ::std::string::String::new(),
            match_uuid: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::PoseStamped::default(),
            priority: 0,
            gripper_id: ::std::string::String::new(),
            stroke_per_finger_approach_mm: 0.0,
            stroke_per_finger_grasp_mm: 0.0,
            collision_checked: false,
        }
    }
}

impl ros2_client::Message for Grasp {}
