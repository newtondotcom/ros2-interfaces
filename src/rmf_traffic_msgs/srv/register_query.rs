use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterQueryReq {
    pub query: crate::rmf_traffic_msgs::msg::ScheduleQuery,
}

impl Default for RegisterQueryReq {
    fn default() -> Self {
        RegisterQueryReq {
            query: crate::rmf_traffic_msgs::msg::ScheduleQuery::default(),
        }
    }
}

impl ros2_client::Message for RegisterQueryReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterQueryRes {
    pub node_id: crate::rmf_traffic_msgs::msg::ScheduleIdentity,
    pub query_id: u64,
    pub error: ::std::string::String,
}

impl Default for RegisterQueryRes {
    fn default() -> Self {
        RegisterQueryRes {
            node_id: crate::rmf_traffic_msgs::msg::ScheduleIdentity::default(),
            query_id: 0,
            error: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for RegisterQueryRes {}


pub struct RegisterQuery;
impl ros2_client::Service for RegisterQuery {
    type Request = RegisterQueryReq;
    type Response = RegisterQueryRes;

    fn request_type_name(&self) -> &str { "RegisterQueryReq" }
    fn response_type_name(&self) -> &str { "RegisterQueryRes" }
}
