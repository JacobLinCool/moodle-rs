use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsOptionsItem {
    /// The backup option name: "activities" (int) Include course activites (default to 1 that is equal to yes), "blocks" (int) Include course blocks (default to 1 that is equal to yes), "filters" (int) Include course filters  (default to 1 that is equal to yes)
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// the value for the option 1 (yes) or 0 (no)
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Course import options
pub type r#ParamsOptions = Vec<ParamsOptionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// the id of the course we are importing from
    #[serde(rename = "importfrom")]
    pub r#importfrom: Option<i64>,
    /// the id of the course we are importing to
    #[serde(rename = "importto")]
    pub r#importto: Option<i64>,
    /// whether to delete the course content where we are importing to (default to 0 = No)
    #[serde(rename = "deletecontent")]
    pub r#deletecontent: Option<i64>,
    /// Course import options
    #[serde(rename = "options")]
    pub r#options: Option<r#ParamsOptions>,
}
