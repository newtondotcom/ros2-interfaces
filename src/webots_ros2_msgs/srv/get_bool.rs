use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBoolReq {
    pub ask: bool,
}

impl Default for GetBoolReq {
    fn default() -> Self {
        GetBoolReq {
            ask: false,
        }
    }
}

impl ros2_client::Message for GetBoolReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBoolRes {
    pub value: bool,
}

impl Default for GetBoolRes {
    fn default() -> Self {
        GetBoolRes {
            value: false,
        }
    }
}

impl ros2_client::Message for GetBoolRes {}


pub struct GetBool;
impl ros2_client::Service for GetBool {
    type Request = GetBoolReq;
    type Response = GetBoolRes;

    fn request_type_name(&self) -> &str { "GetBoolReq" }
    fn response_type_name(&self) -> &str { "GetBoolRes" }
}
