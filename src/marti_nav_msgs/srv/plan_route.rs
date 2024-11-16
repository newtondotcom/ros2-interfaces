use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanRouteReq {
    pub header: crate::std_msgs::msg::Header,
    pub waypoints: Vec<crate::geometry_msgs::msg::Pose>,
    pub plan_from_vehicle: bool,
}

impl Default for PlanRouteReq {
    fn default() -> Self {
        PlanRouteReq {
            header: crate::std_msgs::msg::Header::default(),
            waypoints: Vec::new(),
            plan_from_vehicle: false,
        }
    }
}

impl ros2_client::Message for PlanRouteReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanRouteRes {
    pub route: crate::marti_nav_msgs::msg::Route,
    pub success: bool,
    pub message: ::std::string::String,
    pub cost: f64,
}

impl Default for PlanRouteRes {
    fn default() -> Self {
        PlanRouteRes {
            route: crate::marti_nav_msgs::msg::Route::default(),
            success: false,
            message: ::std::string::String::new(),
            cost: 0.0,
        }
    }
}

impl ros2_client::Message for PlanRouteRes {}


pub struct PlanRoute;
impl ros2_client::Service for PlanRoute {
    type Request = PlanRouteReq;
    type Response = PlanRouteRes;

    fn request_type_name(&self) -> &str { "PlanRouteReq" }
    fn response_type_name(&self) -> &str { "PlanRouteRes" }
}
