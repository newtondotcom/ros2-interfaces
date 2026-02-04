use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityState {
    pub header: crate::std_msgs::msg::Header,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub twist: crate::geometry_msgs::msg::Twist,
    pub acceleration: crate::geometry_msgs::msg::Accel,
}

impl Default for EntityState {
    fn default() -> Self {
        EntityState {
            header: crate::std_msgs::msg::Header::default(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            twist: crate::geometry_msgs::msg::Twist::default(),
            acceleration: crate::geometry_msgs::msg::Accel::default(),
        }
    }
}

impl ros2_client::Message for EntityState {}
