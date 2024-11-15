use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListHardwareComponentsReq {

}

impl Default for ListHardwareComponentsReq {
    fn default() -> Self {
        ListHardwareComponentsReq {

        }
    }
}

impl ros2_client::Message for ListHardwareComponentsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListHardwareComponentsRes {
    pub component: Vec<crate::controller_manager_msgs::msg::HardwareComponentState>,
}

impl Default for ListHardwareComponentsRes {
    fn default() -> Self {
        ListHardwareComponentsRes {
            component: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListHardwareComponentsRes {}


pub struct ListHardwareComponents;
impl ros2_client::Service for ListHardwareComponents {
    type Request = ListHardwareComponentsReq;
    type Response = ListHardwareComponentsRes;

    fn request_type_name(&self) -> &str { "ListHardwareComponentsReq" }
    fn response_type_name(&self) -> &str { "ListHardwareComponentsRes" }
}
