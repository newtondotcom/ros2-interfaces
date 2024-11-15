use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldSetWriteSrvReq {
    pub field_set_selection_method_in: i32,
    pub active_field_set_in: i32,
}

impl Default for FieldSetWriteSrvReq {
    fn default() -> Self {
        FieldSetWriteSrvReq {
            field_set_selection_method_in: 0,
            active_field_set_in: 0,
        }
    }
}

impl ros2_client::Message for FieldSetWriteSrvReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldSetWriteSrvRes {
    pub field_set_selection_method: i32,
    pub active_field_set: i32,
    pub success: bool,
}

impl Default for FieldSetWriteSrvRes {
    fn default() -> Self {
        FieldSetWriteSrvRes {
            field_set_selection_method: 0,
            active_field_set: 0,
            success: false,
        }
    }
}

impl ros2_client::Message for FieldSetWriteSrvRes {}


pub struct FieldSetWriteSrv;
impl ros2_client::Service for FieldSetWriteSrv {
    type Request = FieldSetWriteSrvReq;
    type Response = FieldSetWriteSrvRes;

    fn request_type_name(&self) -> &str { "FieldSetWriteSrvReq" }
    fn response_type_name(&self) -> &str { "FieldSetWriteSrvRes" }
}
