use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// component responsible for the update
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// type of the updated item inside the component
    #[serde(rename = "itemtype")]
    pub r#itemtype: Option<String>,
    /// identifier of the updated item
    #[serde(rename = "itemid")]
    pub r#itemid: Option<String>,
    /// new value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEditicon {
    /// Edit icon key
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// Edit icon component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// Edit icon title
    #[serde(rename = "title")]
    pub r#title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// display value (may contain link or other html tags)
    #[serde(rename = "displayvalue")]
    pub r#displayvalue: Option<String>,
    /// component responsible for the update
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// itemtype
    #[serde(rename = "itemtype")]
    pub r#itemtype: Option<String>,
    /// value of the item as it is stored
    #[serde(rename = "value")]
    pub r#value: Option<String>,
    /// identifier of the updated item
    #[serde(rename = "itemid")]
    pub r#itemid: Option<String>,
    /// hint for editing element
    #[serde(rename = "edithint")]
    pub r#edithint: Option<String>,
    /// label for editing element
    #[serde(rename = "editlabel")]
    pub r#editlabel: Option<String>,
    #[serde(rename = "editicon")]
    pub r#editicon: Option<ReturnsEditicon>,
    /// type of the element (text, toggle, select)
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// options of the element, format depends on type
    #[serde(rename = "options")]
    pub r#options: Option<String>,
    /// Should everything be wrapped in the edit link or link displayed separately
    #[serde(rename = "linkeverything")]
    pub r#linkeverything: Option<i64>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_update_inplace_editable", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_update_inplace_editable", params).await
}
