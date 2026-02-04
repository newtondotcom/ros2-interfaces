use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: ::std::string::String, // default: "NA"
    pub value: ::std::string::String, // default: ""
}

impl Default for Parameter {
    fn default() -> Self {
        Parameter {
            name: ::std::string::String::from("NA"),
            value: ::std::string::String::from(""),
        }
    }
}

impl ros2_client::Message for Parameter {}
