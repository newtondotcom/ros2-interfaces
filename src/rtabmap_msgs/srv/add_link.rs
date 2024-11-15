use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddLinkReq {
    pub link: crate::rtabmap_msgs::msg::Link,
}

impl Default for AddLinkReq {
    fn default() -> Self {
        AddLinkReq {
            link: crate::rtabmap_msgs::msg::Link::default(),
        }
    }
}

impl ros2_client::Message for AddLinkReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddLinkRes {

}

impl Default for AddLinkRes {
    fn default() -> Self {
        AddLinkRes {

        }
    }
}

impl ros2_client::Message for AddLinkRes {}


pub struct AddLink;
impl ros2_client::Service for AddLink {
    type Request = AddLinkReq;
    type Response = AddLinkRes;

    fn request_type_name(&self) -> &str { "AddLinkReq" }
    fn response_type_name(&self) -> &str { "AddLinkRes" }
}
