use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result {
    pub result: u8,
    pub error_message: ::std::string::String,
}

impl Result {
    pub const RESULT_FEATURE_UNSUPPORTED: u8 = 0;
    pub const RESULT_OK: u8 = 1;
    pub const RESULT_NOT_FOUND: u8 = 2;
    pub const RESULT_INCORRECT_STATE: u8 = 3;
    pub const RESULT_OPERATION_FAILED: u8 = 4;
}

impl Default for Result {
    fn default() -> Self {
        Result {
            result: 0,
            error_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Result {}
