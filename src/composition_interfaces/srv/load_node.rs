use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadNodeReq {
    pub package_name: ::std::string::String,
    pub plugin_name: ::std::string::String,
    pub node_name: ::std::string::String,
    pub node_namespace: ::std::string::String,
    pub log_level: u8,
    pub remap_rules: Vec<::std::string::String>,
    pub parameters: Vec<crate::rcl_interfaces::msg::Parameter>,
    pub extra_arguments: Vec<crate::rcl_interfaces::msg::Parameter>,
}

impl Default for LoadNodeReq {
    fn default() -> Self {
        LoadNodeReq {
            package_name: ::std::string::String::new(),
            plugin_name: ::std::string::String::new(),
            node_name: ::std::string::String::new(),
            node_namespace: ::std::string::String::new(),
            log_level: 0,
            remap_rules: Vec::new(),
            parameters: Vec::new(),
            extra_arguments: Vec::new(),
        }
    }
}

impl ros2_client::Message for LoadNodeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadNodeRes {
    pub success: bool,
    pub error_message: ::std::string::String,
    pub full_node_name: ::std::string::String,
    pub unique_id: u64,
}

impl Default for LoadNodeRes {
    fn default() -> Self {
        LoadNodeRes {
            success: false,
            error_message: ::std::string::String::new(),
            full_node_name: ::std::string::String::new(),
            unique_id: 0,
        }
    }
}

impl ros2_client::Message for LoadNodeRes {}


pub struct LoadNode;
impl ros2_client::Service for LoadNode {
    type Request = LoadNodeReq;
    type Response = LoadNodeRes;

    fn request_type_name(&self) -> &str { "LoadNodeReq" }
    fn response_type_name(&self) -> &str { "LoadNodeRes" }
}
