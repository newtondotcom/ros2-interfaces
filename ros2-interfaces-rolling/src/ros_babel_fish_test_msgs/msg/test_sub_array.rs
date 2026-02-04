use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestSubArray {
    pub ints: Vec<i32>,
    pub strings: Vec<::std::string::String>,
    #[serde_as(as = "[_; 42]")]
    pub times: [crate::builtin_interfaces::msg::Time; 42],
    pub floats: [f64; 12],
}

impl Default for TestSubArray {
    fn default() -> Self {
        TestSubArray {
            ints: Vec::new(),
            strings: Vec::new(),
            times: core::array::from_fn(|_| crate::builtin_interfaces::msg::Time::default()),
            floats: [0.0; 12],
        }
    }
}

impl ros2_client::Message for TestSubArray {}
