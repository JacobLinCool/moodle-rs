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
pub struct Returns {
    /// PayPal client ID
    #[serde(rename = "clientid")]
    pub r#clientid: Option<String>,
    /// Brand name
    #[serde(rename = "brandname")]
    pub r#brandname: Option<String>,
    /// Cost with gateway surcharge
    #[serde(rename = "cost")]
    pub r#cost: Option<f64>,
    /// Currency
    #[serde(rename = "currency")]
    pub r#currency: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("paygw_paypal_get_config_for_js", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("paygw_paypal_get_config_for_js", params).await
}
