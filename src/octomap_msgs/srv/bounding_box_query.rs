use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundingBoxQueryReq {
    pub min: crate::geometry_msgs::msg::Point,
    pub max: crate::geometry_msgs::msg::Point,
}

impl Default for BoundingBoxQueryReq {
    fn default() -> Self {
        BoundingBoxQueryReq {
            min: crate::geometry_msgs::msg::Point::default(),
            max: crate::geometry_msgs::msg::Point::default(),
        }
    }
}

impl ros2_client::Message for BoundingBoxQueryReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundingBoxQueryRes {

}

impl Default for BoundingBoxQueryRes {
    fn default() -> Self {
        BoundingBoxQueryRes {

        }
    }
}

impl ros2_client::Message for BoundingBoxQueryRes {}


pub struct BoundingBoxQuery;
impl ros2_client::Service for BoundingBoxQuery {
    type Request = BoundingBoxQueryReq;
    type Response = BoundingBoxQueryRes;

    fn request_type_name(&self) -> &str { "BoundingBoxQueryReq" }
    fn response_type_name(&self) -> &str { "BoundingBoxQueryRes" }
}
