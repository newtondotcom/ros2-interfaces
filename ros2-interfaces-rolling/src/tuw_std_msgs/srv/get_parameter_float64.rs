use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParameterFloat64Request {
    pub name: ::std::string::String,
}

impl Default for GetParameterFloat64Request {
    fn default() -> Self {
        GetParameterFloat64Request {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetParameterFloat64Request {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParameterFloat64Response {
    pub header: crate::std_msgs::msg::Header,
    pub data: f64,
}

impl Default for GetParameterFloat64Response {
    fn default() -> Self {
        GetParameterFloat64Response {
            header: crate::std_msgs::msg::Header::default(),
            data: 0.0,
        }
    }
}

impl ros2_client::Message for GetParameterFloat64Response {}


pub struct GetParameterFloat64;
impl ros2_client::Service for GetParameterFloat64 {
    type Request = GetParameterFloat64Request;
    type Response = GetParameterFloat64Response;

    fn request_type_name(&self) -> &str { "GetParameterFloat64Request" }
    fn response_type_name(&self) -> &str { "GetParameterFloat64Response" }
}
