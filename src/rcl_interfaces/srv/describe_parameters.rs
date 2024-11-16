use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescribeParametersReq {
    pub names: Vec<::std::string::String>,
}

impl Default for DescribeParametersReq {
    fn default() -> Self {
        DescribeParametersReq {
            names: Vec::new(),
        }
    }
}

impl ros2_client::Message for DescribeParametersReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescribeParametersRes {
    pub descriptors: Vec<crate::rcl_interfaces::msg::ParameterDescriptor>,
}

impl Default for DescribeParametersRes {
    fn default() -> Self {
        DescribeParametersRes {
            descriptors: Vec::new(),
        }
    }
}

impl ros2_client::Message for DescribeParametersRes {}


pub struct DescribeParameters;
impl ros2_client::Service for DescribeParameters {
    type Request = DescribeParametersReq;
    type Response = DescribeParametersRes;

    fn request_type_name(&self) -> &str { "DescribeParametersReq" }
    fn response_type_name(&self) -> &str { "DescribeParametersRes" }
}
