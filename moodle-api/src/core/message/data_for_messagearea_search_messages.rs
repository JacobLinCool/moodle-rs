use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The id of the user who is performing the search
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The string being searched
    #[serde(rename = "search")]
    pub r#search: Option<String>,
    /// Limit from
    #[serde(rename = "limitfrom")]
    pub r#limitfrom: Option<i64>,
    /// Limit number
    #[serde(rename = "limitnum")]
    pub r#limitnum: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsContactsItem {
    /// The user's id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The user's name
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// User picture URL
    #[serde(rename = "profileimageurl")]
    pub r#profileimageurl: Option<String>,
    /// Small user picture URL
    #[serde(rename = "profileimageurlsmall")]
    pub r#profileimageurlsmall: Option<String>,
    /// If we are messaging the user
    #[serde(rename = "ismessaging")]
    pub r#ismessaging: Option<bool>,
    /// Was the last message sent from the current user?
    #[serde(rename = "sentfromcurrentuser")]
    pub r#sentfromcurrentuser: Option<bool>,
    /// The user's last message
    #[serde(rename = "lastmessage")]
    pub r#lastmessage: Option<String>,
    /// Timestamp for last message
    #[serde(rename = "lastmessagedate")]
    pub r#lastmessagedate: Option<i64>,
    /// The unique search message id
    #[serde(rename = "messageid")]
    pub r#messageid: Option<i64>,
    /// Show the user's online status?
    #[serde(rename = "showonlinestatus")]
    pub r#showonlinestatus: Option<bool>,
    /// The user's online status
    #[serde(rename = "isonline")]
    pub r#isonline: Option<bool>,
    /// If the user has read the message
    #[serde(rename = "isread")]
    pub r#isread: Option<bool>,
    /// If the user has been blocked
    #[serde(rename = "isblocked")]
    pub r#isblocked: Option<bool>,
    /// The number of unread messages in this conversation
    #[serde(rename = "unreadcount")]
    pub r#unreadcount: Option<i64>,
    /// The id of the conversation
    #[serde(rename = "conversationid")]
    pub r#conversationid: Option<i64>,
}

pub type r#ReturnsContacts = Vec<ReturnsContactsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "contacts")]
    pub r#contacts: Option<r#ReturnsContacts>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_message_data_for_messagearea_search_messages", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_message_data_for_messagearea_search_messages", params)
        .await
}
