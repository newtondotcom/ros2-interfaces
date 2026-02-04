use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrajectoryPoint {
    pub header: crate::std_msgs::msg::Header,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub velocity: crate::geometry_msgs::msg::Twist,
    pub acceleration: crate::geometry_msgs::msg::Accel,
    pub effort: crate::geometry_msgs::msg::Wrench,
}

impl Default for TrajectoryPoint {
    fn default() -> Self {
        TrajectoryPoint {
            header: crate::std_msgs::msg::Header::default(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            velocity: crate::geometry_msgs::msg::Twist::default(),
            acceleration: crate::geometry_msgs::msg::Accel::default(),
            effort: crate::geometry_msgs::msg::Wrench::default(),
        }
    }
}

impl ros2_client::Message for TrajectoryPoint {}
