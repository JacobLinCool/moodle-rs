use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// The component for the icon.
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// Value to map the icon from.
    #[serde(rename = "pix")]
    pub r#pix: Option<String>,
    /// Value to map the icon to.
    #[serde(rename = "to")]
    pub r#to: Option<String>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut crate::client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_output_load_fontawesome_icon_map", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}
