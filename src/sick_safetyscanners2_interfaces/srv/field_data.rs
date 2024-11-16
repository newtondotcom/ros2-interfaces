use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldDataReq {

}

impl Default for FieldDataReq {
    fn default() -> Self {
        FieldDataReq {

        }
    }
}

impl ros2_client::Message for FieldDataReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldDataRes {
    pub fields: Vec<crate::sick_safetyscanners2_interfaces::msg::Field>,
    pub device_name: ::std::string::String,
    pub monitoring_cases: Vec<crate::sick_safetyscanners2_interfaces::msg::MonitoringCase>,
}

impl Default for FieldDataRes {
    fn default() -> Self {
        FieldDataRes {
            fields: Vec::new(),
            device_name: ::std::string::String::new(),
            monitoring_cases: Vec::new(),
        }
    }
}

impl ros2_client::Message for FieldDataRes {}


pub struct FieldData;
impl ros2_client::Service for FieldData {
    type Request = FieldDataReq;
    type Response = FieldDataRes;

    fn request_type_name(&self) -> &str { "FieldDataReq" }
    fn response_type_name(&self) -> &str { "FieldDataRes" }
}
