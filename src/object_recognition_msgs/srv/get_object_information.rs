use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetObjectInformationReq {
    #[serde(rename = "type")]    pub type_: crate::object_recognition_msgs::msg::ObjectType,
}

impl Default for GetObjectInformationReq {
    fn default() -> Self {
        GetObjectInformationReq {
            type_: crate::object_recognition_msgs::msg::ObjectType::default(),
        }
    }
}

impl ros2_client::Message for GetObjectInformationReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetObjectInformationRes {
    pub information: crate::object_recognition_msgs::msg::ObjectInformation,
}

impl Default for GetObjectInformationRes {
    fn default() -> Self {
        GetObjectInformationRes {
            information: crate::object_recognition_msgs::msg::ObjectInformation::default(),
        }
    }
}

impl ros2_client::Message for GetObjectInformationRes {}


pub struct GetObjectInformation;
impl ros2_client::Service for GetObjectInformation {
    type Request = GetObjectInformationReq;
    type Response = GetObjectInformationRes;

    fn request_type_name(&self) -> &str { "GetObjectInformationReq" }
    fn response_type_name(&self) -> &str { "GetObjectInformationRes" }
}
