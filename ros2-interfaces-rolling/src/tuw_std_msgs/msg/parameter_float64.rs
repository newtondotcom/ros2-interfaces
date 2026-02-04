use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterFloat64 {
    pub name: crate::std_msgs::msg::String,
    pub data: f64,
}

impl Default for ParameterFloat64 {
    fn default() -> Self {
        ParameterFloat64 {
            name: crate::std_msgs::msg::String::default(),
            data: 0.0,
        }
    }
}

impl ros2_client::Message for ParameterFloat64 {}
