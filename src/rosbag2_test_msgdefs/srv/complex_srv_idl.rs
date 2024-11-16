use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexSrvIdlReq {
    pub req: crate::rosbag2_test_msgdefs::msg::BasicIdl,
}

impl Default for ComplexSrvIdlReq {
    fn default() -> Self {
        ComplexSrvIdlReq {
            req: crate::rosbag2_test_msgdefs::msg::BasicIdl::default(),
        }
    }
}

impl ros2_client::Message for ComplexSrvIdlReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexSrvIdlRes {
    pub resp: crate::rosbag2_test_msgdefs::msg::BasicIdl,
}

impl Default for ComplexSrvIdlRes {
    fn default() -> Self {
        ComplexSrvIdlRes {
            resp: crate::rosbag2_test_msgdefs::msg::BasicIdl::default(),
        }
    }
}

impl ros2_client::Message for ComplexSrvIdlRes {}


pub struct ComplexSrvIdl;
impl ros2_client::Service for ComplexSrvIdl {
    type Request = ComplexSrvIdlReq;
    type Response = ComplexSrvIdlRes;

    fn request_type_name(&self) -> &str { "ComplexSrvIdlReq" }
    fn response_type_name(&self) -> &str { "ComplexSrvIdlRes" }
}
