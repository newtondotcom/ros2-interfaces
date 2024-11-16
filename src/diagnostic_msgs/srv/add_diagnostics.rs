use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddDiagnosticsReq {
    pub load_namespace: ::std::string::String,
}

impl Default for AddDiagnosticsReq {
    fn default() -> Self {
        AddDiagnosticsReq {
            load_namespace: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for AddDiagnosticsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddDiagnosticsRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for AddDiagnosticsRes {
    fn default() -> Self {
        AddDiagnosticsRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for AddDiagnosticsRes {}


pub struct AddDiagnostics;
impl ros2_client::Service for AddDiagnostics {
    type Request = AddDiagnosticsReq;
    type Response = AddDiagnosticsRes;

    fn request_type_name(&self) -> &str { "AddDiagnosticsReq" }
    fn response_type_name(&self) -> &str { "AddDiagnosticsRes" }
}
