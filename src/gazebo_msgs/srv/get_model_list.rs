use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetModelListReq {

}

impl Default for GetModelListReq {
    fn default() -> Self {
        GetModelListReq {

        }
    }
}

impl ros2_client::Message for GetModelListReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetModelListRes {
    pub header: crate::std_msgs::msg::Header,
    pub model_names: Vec<::std::string::String>,
    pub success: bool,
}

impl Default for GetModelListRes {
    fn default() -> Self {
        GetModelListRes {
            header: crate::std_msgs::msg::Header::default(),
            model_names: Vec::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for GetModelListRes {}


pub struct GetModelList;
impl ros2_client::Service for GetModelList {
    type Request = GetModelListReq;
    type Response = GetModelListRes;

    fn request_type_name(&self) -> &str { "GetModelListReq" }
    fn response_type_name(&self) -> &str { "GetModelListRes" }
}
