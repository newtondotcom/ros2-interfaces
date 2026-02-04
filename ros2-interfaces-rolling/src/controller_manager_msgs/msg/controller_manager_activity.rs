use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControllerManagerActivity {
    pub header: crate::std_msgs::msg::Header,
    pub controllers: Vec<crate::controller_manager_msgs::msg::NamedLifecycleState>,
    pub hardware_components: Vec<crate::controller_manager_msgs::msg::NamedLifecycleState>,
}

impl Default for ControllerManagerActivity {
    fn default() -> Self {
        ControllerManagerActivity {
            header: crate::std_msgs::msg::Header::default(),
            controllers: Vec::new(),
            hardware_components: Vec::new(),
        }
    }
}

impl ros2_client::Message for ControllerManagerActivity {}
