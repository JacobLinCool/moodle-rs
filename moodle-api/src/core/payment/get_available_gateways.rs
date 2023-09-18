use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// Payment area in the component
    #[serde(rename = "paymentarea")]
    pub r#paymentarea: Option<String>,
    /// An identifier for payment area in the component
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// Name of the plugin
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// Human readable name of the gateway
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// description of the gateway
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// percentage of surcharge when using the gateway
    #[serde(rename = "surcharge")]
    pub r#surcharge: Option<i64>,
    /// Cost in human-readable form (amount plus surcharge with currency sign)
    #[serde(rename = "cost")]
    pub r#cost: Option<String>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_payment_get_available_gateways", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_payment_get_available_gateways", params)
        .await
}
