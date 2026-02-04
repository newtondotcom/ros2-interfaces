use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterArray {
    pub data: Vec<crate::tuw_std_msgs::msg::Parameter>,
}

impl Default for ParameterArray {
    fn default() -> Self {
        ParameterArray {
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for ParameterArray {}
