use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Information {
    pub header: crate::std_msgs::msg::Header,
    pub yaw_rate_stat: u8,
    pub roll_rate_stat: u8,
    pub pitch_rate_stat: u8,
    pub ax_stat: u8,
    pub ay_stat: u8,
    pub az_stat: u8,
    pub temp_rate_z: u8,
    pub hw_index: u8,
}

impl Default for Information {
    fn default() -> Self {
        Information {
            header: crate::std_msgs::msg::Header::default(),
            yaw_rate_stat: 0,
            roll_rate_stat: 0,
            pitch_rate_stat: 0,
            ax_stat: 0,
            ay_stat: 0,
            az_stat: 0,
            temp_rate_z: 0,
            hw_index: 0,
        }
    }
}

impl ros2_client::Message for Information {}
