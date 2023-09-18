use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The component name
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// Payment area in the component
    #[serde(rename = "paymentarea")]
    pub r#paymentarea: Option<String>,
    /// The item id in the context of the component area
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// The order id coming back from PayPal
    #[serde(rename = "orderid")]
    pub r#orderid: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Whether everything was successful or not.
    #[serde(rename = "success")]
    pub r#success: Option<bool>,
    /// Message (usually the error message).
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("paygw_paypal_create_transaction_complete", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("paygw_paypal_create_transaction_complete", params)
        .await
}
