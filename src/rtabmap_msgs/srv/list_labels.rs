use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListLabelsReq {

}

impl Default for ListLabelsReq {
    fn default() -> Self {
        ListLabelsReq {

        }
    }
}

impl ros2_client::Message for ListLabelsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListLabelsRes {
    pub ids: Vec<i32>,
    pub labels: Vec<::std::string::String>,
}

impl Default for ListLabelsRes {
    fn default() -> Self {
        ListLabelsRes {
            ids: Vec::new(),
            labels: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListLabelsRes {}


pub struct ListLabels;
impl ros2_client::Service for ListLabels {
    type Request = ListLabelsReq;
    type Response = ListLabelsRes;

    fn request_type_name(&self) -> &str { "ListLabelsReq" }
    fn response_type_name(&self) -> &str { "ListLabelsRes" }
}
