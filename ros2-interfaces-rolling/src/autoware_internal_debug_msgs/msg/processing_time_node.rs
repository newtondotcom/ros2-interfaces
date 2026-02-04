use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessingTimeNode {
    pub id: i32,
    pub name: ::std::string::String,
    pub processing_time: f64,
    pub parent_id: i32,
    pub comment: ::std::string::String,
}

impl Default for ProcessingTimeNode {
    fn default() -> Self {
        ProcessingTimeNode {
            id: 0,
            name: ::std::string::String::new(),
            processing_time: 0.0,
            parent_id: 0,
            comment: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ProcessingTimeNode {}
