use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamedLifecycleState {
    pub name: ::std::string::String,
    pub state: crate::lifecycle_msgs::msg::State,
}

impl Default for NamedLifecycleState {
    fn default() -> Self {
        NamedLifecycleState {
            name: ::std::string::String::new(),
            state: crate::lifecycle_msgs::msg::State::default(),
        }
    }
}

impl ros2_client::Message for NamedLifecycleState {}
