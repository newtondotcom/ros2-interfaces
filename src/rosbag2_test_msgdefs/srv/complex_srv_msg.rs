use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexSrvMsgReq {
    pub req: crate::rosbag2_test_msgdefs::msg::BasicMsg,
}

impl Default for ComplexSrvMsgReq {
    fn default() -> Self {
        ComplexSrvMsgReq {
            req: crate::rosbag2_test_msgdefs::msg::BasicMsg::default(),
        }
    }
}

impl ros2_client::Message for ComplexSrvMsgReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexSrvMsgRes {
    pub resp: crate::rosbag2_test_msgdefs::msg::BasicMsg,
}

impl Default for ComplexSrvMsgRes {
    fn default() -> Self {
        ComplexSrvMsgRes {
            resp: crate::rosbag2_test_msgdefs::msg::BasicMsg::default(),
        }
    }
}

impl ros2_client::Message for ComplexSrvMsgRes {}


pub struct ComplexSrvMsg;
impl ros2_client::Service for ComplexSrvMsg {
    type Request = ComplexSrvMsgReq;
    type Response = ComplexSrvMsgRes;

    fn request_type_name(&self) -> &str { "ComplexSrvMsgReq" }
    fn response_type_name(&self) -> &str { "ComplexSrvMsgRes" }
}
