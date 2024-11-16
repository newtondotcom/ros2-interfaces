use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicsAndRawTypesReq {

}

impl Default for TopicsAndRawTypesReq {
    fn default() -> Self {
        TopicsAndRawTypesReq {

        }
    }
}

impl ros2_client::Message for TopicsAndRawTypesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicsAndRawTypesRes {
    pub topics: Vec<::std::string::String>,
    pub types: Vec<::std::string::String>,
    pub typedefs_full_text: Vec<::std::string::String>,
}

impl Default for TopicsAndRawTypesRes {
    fn default() -> Self {
        TopicsAndRawTypesRes {
            topics: Vec::new(),
            types: Vec::new(),
            typedefs_full_text: Vec::new(),
        }
    }
}

impl ros2_client::Message for TopicsAndRawTypesRes {}


pub struct TopicsAndRawTypes;
impl ros2_client::Service for TopicsAndRawTypes {
    type Request = TopicsAndRawTypesReq;
    type Response = TopicsAndRawTypesRes;

    fn request_type_name(&self) -> &str { "TopicsAndRawTypesReq" }
    fn response_type_name(&self) -> &str { "TopicsAndRawTypesRes" }
}
