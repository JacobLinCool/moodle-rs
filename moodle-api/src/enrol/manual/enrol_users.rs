use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsEnrolmentsItem {
    /// Role to assign to the user
    #[serde(rename = "roleid")]
    pub r#roleid: Option<i64>,
    /// The user that is going to be enrolled
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The course to enrol the user role in
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// Timestamp when the enrolment start
    #[serde(rename = "timestart")]
    pub r#timestart: Option<i64>,
    /// Timestamp when the enrolment end
    #[serde(rename = "timeend")]
    pub r#timeend: Option<i64>,
    /// set to 1 to suspend the enrolment
    #[serde(rename = "suspend")]
    pub r#suspend: Option<i64>,
}

pub type r#ParamsEnrolments = Vec<ParamsEnrolmentsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "enrolments")]
    pub r#enrolments: Option<r#ParamsEnrolments>,
}
