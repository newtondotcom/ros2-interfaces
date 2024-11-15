use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListHardwareInterfacesReq {

}

impl Default for ListHardwareInterfacesReq {
    fn default() -> Self {
        ListHardwareInterfacesReq {

        }
    }
}

impl ros2_client::Message for ListHardwareInterfacesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListHardwareInterfacesRes {
    pub command_interfaces: Vec<crate::controller_manager_msgs::msg::HardwareInterface>,
    pub state_interfaces: Vec<crate::controller_manager_msgs::msg::HardwareInterface>,
}

impl Default for ListHardwareInterfacesRes {
    fn default() -> Self {
        ListHardwareInterfacesRes {
            command_interfaces: Vec::new(),
            state_interfaces: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListHardwareInterfacesRes {}


pub struct ListHardwareInterfaces;
impl ros2_client::Service for ListHardwareInterfaces {
    type Request = ListHardwareInterfacesReq;
    type Response = ListHardwareInterfacesRes;

    fn request_type_name(&self) -> &str { "ListHardwareInterfacesReq" }
    fn response_type_name(&self) -> &str { "ListHardwareInterfacesRes" }
}
