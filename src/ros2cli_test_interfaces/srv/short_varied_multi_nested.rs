use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShortVariedMultiNestedReq {
    pub short_varied_nested: crate::ros2cli_test_interfaces::msg::ShortVariedNested,
}

impl Default for ShortVariedMultiNestedReq {
    fn default() -> Self {
        ShortVariedMultiNestedReq {
            short_varied_nested: crate::ros2cli_test_interfaces::msg::ShortVariedNested::default(),
        }
    }
}

impl ros2_client::Message for ShortVariedMultiNestedReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShortVariedMultiNestedRes {
    pub bool_value: bool,
}

impl Default for ShortVariedMultiNestedRes {
    fn default() -> Self {
        ShortVariedMultiNestedRes {
            bool_value: false,
        }
    }
}

impl ros2_client::Message for ShortVariedMultiNestedRes {}


pub struct ShortVariedMultiNested;
impl ros2_client::Service for ShortVariedMultiNested {
    type Request = ShortVariedMultiNestedReq;
    type Response = ShortVariedMultiNestedRes;

    fn request_type_name(&self) -> &str { "ShortVariedMultiNestedReq" }
    fn response_type_name(&self) -> &str { "ShortVariedMultiNestedRes" }
}
