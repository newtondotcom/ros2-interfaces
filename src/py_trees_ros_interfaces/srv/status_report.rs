use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusReportReq {

}

impl Default for StatusReportReq {
    fn default() -> Self {
        StatusReportReq {

        }
    }
}

impl ros2_client::Message for StatusReportReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusReportRes {
    pub report: ::std::string::String,
}

impl Default for StatusReportRes {
    fn default() -> Self {
        StatusReportRes {
            report: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for StatusReportRes {}


pub struct StatusReport;
impl ros2_client::Service for StatusReport {
    type Request = StatusReportReq;
    type Response = StatusReportRes;

    fn request_type_name(&self) -> &str { "StatusReportReq" }
    fn response_type_name(&self) -> &str { "StatusReportRes" }
}
