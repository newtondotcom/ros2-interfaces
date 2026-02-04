use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagsFilter {
    pub tags: Vec<::std::string::String>,
    pub filter_mode: u8,
}

impl TagsFilter {
    pub const FILTER_MODE_ANY: u8 = 0;
    pub const FILTER_MODE_ALL: u8 = 1;
}

impl Default for TagsFilter {
    fn default() -> Self {
        TagsFilter {
            tags: Vec::new(),
            filter_mode: 0,
        }
    }
}

impl ros2_client::Message for TagsFilter {}
