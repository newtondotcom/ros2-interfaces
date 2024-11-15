use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldSetReadSrvReq {

}

impl Default for FieldSetReadSrvReq {
    fn default() -> Self {
        FieldSetReadSrvReq {

        }
    }
}

impl ros2_client::Message for FieldSetReadSrvReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldSetReadSrvRes {
    pub field_set_selection_method: i32,
    pub active_field_set: i32,
    pub success: bool,
}

impl Default for FieldSetReadSrvRes {
    fn default() -> Self {
        FieldSetReadSrvRes {
            field_set_selection_method: 0,
            active_field_set: 0,
            success: false,
        }
    }
}

impl ros2_client::Message for FieldSetReadSrvRes {}


pub struct FieldSetReadSrv;
impl ros2_client::Service for FieldSetReadSrv {
    type Request = FieldSetReadSrvReq;
    type Response = FieldSetReadSrvRes;

    fn request_type_name(&self) -> &str { "FieldSetReadSrvReq" }
    fn response_type_name(&self) -> &str { "FieldSetReadSrvRes" }
}
