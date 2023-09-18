use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsEnrolmentsItem {
    /// The user that is going to be unenrolled
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The course to unenrol the user from
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// The user role
    #[serde(rename = "roleid")]
    pub r#roleid: Option<i64>,
}

pub type r#ParamsEnrolments = Vec<ParamsEnrolmentsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "enrolments")]
    pub r#enrolments: Option<r#ParamsEnrolments>,
}
