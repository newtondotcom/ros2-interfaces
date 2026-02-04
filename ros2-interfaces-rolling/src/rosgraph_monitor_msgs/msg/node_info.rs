use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeInfo {
    pub name: ::std::string::String,
    pub publishers: Vec<crate::rosgraph_monitor_msgs::msg::Topic>,
    pub subscriptions: Vec<crate::rosgraph_monitor_msgs::msg::Topic>,
    pub parameters: Vec<crate::rcl_interfaces::msg::ParameterDescriptor>,
    pub parameter_values: Vec<crate::rcl_interfaces::msg::ParameterValue>,
}

impl Default for NodeInfo {
    fn default() -> Self {
        NodeInfo {
            name: ::std::string::String::new(),
            publishers: Vec::new(),
            subscriptions: Vec::new(),
            parameters: Vec::new(),
            parameter_values: Vec::new(),
        }
    }
}

impl ros2_client::Message for NodeInfo {}
