use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListControllersReq {

}

impl Default for ListControllersReq {
    fn default() -> Self {
        ListControllersReq {

        }
    }
}

impl ros2_client::Message for ListControllersReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListControllersRes {
    pub controller: Vec<crate::controller_manager_msgs::msg::ControllerState>,
}

impl Default for ListControllersRes {
    fn default() -> Self {
        ListControllersRes {
            controller: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListControllersRes {}


pub struct ListControllers;
impl ros2_client::Service for ListControllers {
    type Request = ListControllersReq;
    type Response = ListControllersRes;

    fn request_type_name(&self) -> &str { "ListControllersReq" }
    fn response_type_name(&self) -> &str { "ListControllersRes" }
}
