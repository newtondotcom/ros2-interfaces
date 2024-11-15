use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddTwoIntsReq {
    pub a: i64,
    pub b: i64,
}

impl Default for AddTwoIntsReq {
    fn default() -> Self {
        AddTwoIntsReq {
            a: 0,
            b: 0,
        }
    }
}

impl ros2_client::Message for AddTwoIntsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddTwoIntsRes {
    pub sum: i64,
}

impl Default for AddTwoIntsRes {
    fn default() -> Self {
        AddTwoIntsRes {
            sum: 0,
        }
    }
}

impl ros2_client::Message for AddTwoIntsRes {}


pub struct AddTwoInts;
impl ros2_client::Service for AddTwoInts {
    type Request = AddTwoIntsReq;
    type Response = AddTwoIntsRes;

    fn request_type_name(&self) -> &str { "AddTwoIntsReq" }
    fn response_type_name(&self) -> &str { "AddTwoIntsRes" }
}
