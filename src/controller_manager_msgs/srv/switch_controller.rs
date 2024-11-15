use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwitchControllerReq {
    pub activate_controllers: Vec<::std::string::String>,
    pub deactivate_controllers: Vec<::std::string::String>,
    pub strictness: i32,
    pub activate_asap: bool,
    pub timeout: crate::builtin_interfaces::msg::Duration,
}

impl SwitchControllerReq {
    pub const BEST_EFFORT: i32 = 1;
    pub const STRICT: i32 = 2;
}

impl Default for SwitchControllerReq {
    fn default() -> Self {
        SwitchControllerReq {
            activate_controllers: Vec::new(),
            deactivate_controllers: Vec::new(),
            strictness: 0,
            activate_asap: false,
            timeout: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}

impl ros2_client::Message for SwitchControllerReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwitchControllerRes {
    pub ok: bool,
}

impl Default for SwitchControllerRes {
    fn default() -> Self {
        SwitchControllerRes {
            ok: false,
        }
    }
}

impl ros2_client::Message for SwitchControllerRes {}


pub struct SwitchController;
impl ros2_client::Service for SwitchController {
    type Request = SwitchControllerReq;
    type Response = SwitchControllerRes;

    fn request_type_name(&self) -> &str { "SwitchControllerReq" }
    fn response_type_name(&self) -> &str { "SwitchControllerRes" }
}
