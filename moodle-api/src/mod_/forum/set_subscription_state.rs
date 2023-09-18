use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Forum that the discussion is in
    #[serde(rename = "forumid")]
    pub r#forumid: Option<i64>,
    /// The discussion to subscribe or unsubscribe
    #[serde(rename = "discussionid")]
    pub r#discussionid: Option<i64>,
    /// The target state
    #[serde(rename = "targetstate")]
    pub r#targetstate: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsGroupUrls {
    /// picture
    #[serde(rename = "picture")]
    pub r#picture: Option<String>,
    /// userlist
    #[serde(rename = "userlist")]
    pub r#userlist: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsGroup {
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "urls")]
    pub r#urls: Option<ReturnsGroupUrls>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTimes {
    /// modified
    #[serde(rename = "modified")]
    pub r#modified: Option<i64>,
    /// start
    #[serde(rename = "start")]
    pub r#start: Option<i64>,
    /// end
    #[serde(rename = "end")]
    pub r#end: Option<i64>,
    /// locked
    #[serde(rename = "locked")]
    pub r#locked: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUserstate {
    /// subscribed
    #[serde(rename = "subscribed")]
    pub r#subscribed: Option<bool>,
    /// favourited
    #[serde(rename = "favourited")]
    pub r#favourited: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCapabilities {
    /// subscribe
    #[serde(rename = "subscribe")]
    pub r#subscribe: Option<bool>,
    /// move
    #[serde(rename = "move")]
    pub r#move: Option<bool>,
    /// pin
    #[serde(rename = "pin")]
    pub r#pin: Option<bool>,
    /// post
    #[serde(rename = "post")]
    pub r#post: Option<bool>,
    /// manage
    #[serde(rename = "manage")]
    pub r#manage: Option<bool>,
    /// favourite
    #[serde(rename = "favourite")]
    pub r#favourite: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUrls {
    /// view
    #[serde(rename = "view")]
    pub r#view: Option<String>,
    /// viewlatest
    #[serde(rename = "viewlatest")]
    pub r#viewlatest: Option<String>,
    /// viewfirstunread
    #[serde(rename = "viewfirstunread")]
    pub r#viewfirstunread: Option<String>,
    /// markasread
    #[serde(rename = "markasread")]
    pub r#markasread: Option<String>,
    /// subscribe
    #[serde(rename = "subscribe")]
    pub r#subscribe: Option<String>,
    /// pin
    #[serde(rename = "pin")]
    pub r#pin: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTimed {
    /// istimed
    #[serde(rename = "istimed")]
    pub r#istimed: Option<bool>,
    /// visible
    #[serde(rename = "visible")]
    pub r#visible: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// forumid
    #[serde(rename = "forumid")]
    pub r#forumid: Option<i64>,
    /// pinned
    #[serde(rename = "pinned")]
    pub r#pinned: Option<bool>,
    /// locked
    #[serde(rename = "locked")]
    pub r#locked: Option<bool>,
    /// istimelocked
    #[serde(rename = "istimelocked")]
    pub r#istimelocked: Option<bool>,
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// firstpostid
    #[serde(rename = "firstpostid")]
    pub r#firstpostid: Option<i64>,
    #[serde(rename = "group")]
    pub r#group: Option<ReturnsGroup>,
    #[serde(rename = "times")]
    pub r#times: Option<ReturnsTimes>,
    #[serde(rename = "userstate")]
    pub r#userstate: Option<ReturnsUserstate>,
    #[serde(rename = "capabilities")]
    pub r#capabilities: Option<ReturnsCapabilities>,
    #[serde(rename = "urls")]
    pub r#urls: Option<ReturnsUrls>,
    #[serde(rename = "timed")]
    pub r#timed: Option<ReturnsTimed>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_forum_set_subscription_state", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_forum_set_subscription_state", params)
        .await
}
