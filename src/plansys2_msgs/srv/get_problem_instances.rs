use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProblemInstancesReq {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetProblemInstancesReq {
    fn default() -> Self {
        GetProblemInstancesReq {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for GetProblemInstancesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProblemInstancesRes {
    pub success: bool,
    pub instances: Vec<crate::plansys2_msgs::msg::Param>,
    pub error_info: ::std::string::String,
}

impl Default for GetProblemInstancesRes {
    fn default() -> Self {
        GetProblemInstancesRes {
            success: false,
            instances: Vec::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetProblemInstancesRes {}


pub struct GetProblemInstances;
impl ros2_client::Service for GetProblemInstances {
    type Request = GetProblemInstancesReq;
    type Response = GetProblemInstancesRes;

    fn request_type_name(&self) -> &str { "GetProblemInstancesReq" }
    fn response_type_name(&self) -> &str { "GetProblemInstancesRes" }
}
