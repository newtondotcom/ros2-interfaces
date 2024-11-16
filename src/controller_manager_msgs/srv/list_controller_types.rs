use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListControllerTypesReq {

}

impl Default for ListControllerTypesReq {
    fn default() -> Self {
        ListControllerTypesReq {

        }
    }
}

impl ros2_client::Message for ListControllerTypesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListControllerTypesRes {
    pub types: Vec<::std::string::String>,
    pub base_classes: Vec<::std::string::String>,
}

impl Default for ListControllerTypesRes {
    fn default() -> Self {
        ListControllerTypesRes {
            types: Vec::new(),
            base_classes: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListControllerTypesRes {}


pub struct ListControllerTypes;
impl ros2_client::Service for ListControllerTypes {
    type Request = ListControllerTypesReq;
    type Response = ListControllerTypesRes;

    fn request_type_name(&self) -> &str { "ListControllerTypesReq" }
    fn response_type_name(&self) -> &str { "ListControllerTypesRes" }
}
