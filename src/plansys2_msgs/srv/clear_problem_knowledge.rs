use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearProblemKnowledgeReq {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for ClearProblemKnowledgeReq {
    fn default() -> Self {
        ClearProblemKnowledgeReq {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for ClearProblemKnowledgeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearProblemKnowledgeRes {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for ClearProblemKnowledgeRes {
    fn default() -> Self {
        ClearProblemKnowledgeRes {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ClearProblemKnowledgeRes {}


pub struct ClearProblemKnowledge;
impl ros2_client::Service for ClearProblemKnowledge {
    type Request = ClearProblemKnowledgeReq;
    type Response = ClearProblemKnowledgeRes;

    fn request_type_name(&self) -> &str { "ClearProblemKnowledgeReq" }
    fn response_type_name(&self) -> &str { "ClearProblemKnowledgeRes" }
}
