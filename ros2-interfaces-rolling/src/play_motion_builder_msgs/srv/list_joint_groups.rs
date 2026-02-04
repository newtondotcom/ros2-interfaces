use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListJointGroupsRequest {

}

impl Default for ListJointGroupsRequest {
    fn default() -> Self {
        ListJointGroupsRequest {

        }
    }
}

impl ros2_client::Message for ListJointGroupsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListJointGroupsResponse {
    pub groups: Vec<::std::string::String>,
    pub additional_joints: Vec<::std::string::String>,
    pub available_joints: Vec<::std::string::String>,
}

impl Default for ListJointGroupsResponse {
    fn default() -> Self {
        ListJointGroupsResponse {
            groups: Vec::new(),
            additional_joints: Vec::new(),
            available_joints: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListJointGroupsResponse {}


pub struct ListJointGroups;
impl ros2_client::Service for ListJointGroups {
    type Request = ListJointGroupsRequest;
    type Response = ListJointGroupsResponse;

    fn request_type_name(&self) -> &str { "ListJointGroupsRequest" }
    fn response_type_name(&self) -> &str { "ListJointGroupsResponse" }
}
