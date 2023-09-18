use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Forum instance id.
    #[serde(rename = "forumid")]
    pub r#forumid: Option<i64>,
}

/// warning
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWarningsItem {
    /// item
    #[serde(rename = "item")]
    pub r#item: Option<String>,
    /// item id
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// the warning code can be used by the client app to implement specific behaviour
    #[serde(rename = "warningcode")]
    pub r#warningcode: Option<String>,
    /// untranslated english message to explain the warning
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}

/// list of warnings
pub type r#ReturnsWarnings = Vec<ReturnsWarningsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
    /// Whether the user has the capability mod/forum:addinstance allowed.
    #[serde(rename = "canaddinstance")]
    pub r#canaddinstance: Option<bool>,
    /// Whether the user has the capability mod/forum:viewdiscussion allowed.
    #[serde(rename = "canviewdiscussion")]
    pub r#canviewdiscussion: Option<bool>,
    /// Whether the user has the capability mod/forum:viewhiddentimedposts allowed.
    #[serde(rename = "canviewhiddentimedposts")]
    pub r#canviewhiddentimedposts: Option<bool>,
    /// Whether the user has the capability mod/forum:startdiscussion allowed.
    #[serde(rename = "canstartdiscussion")]
    pub r#canstartdiscussion: Option<bool>,
    /// Whether the user has the capability mod/forum:replypost allowed.
    #[serde(rename = "canreplypost")]
    pub r#canreplypost: Option<bool>,
    /// Whether the user has the capability mod/forum:addnews allowed.
    #[serde(rename = "canaddnews")]
    pub r#canaddnews: Option<bool>,
    /// Whether the user has the capability mod/forum:replynews allowed.
    #[serde(rename = "canreplynews")]
    pub r#canreplynews: Option<bool>,
    /// Whether the user has the capability mod/forum:viewrating allowed.
    #[serde(rename = "canviewrating")]
    pub r#canviewrating: Option<bool>,
    /// Whether the user has the capability mod/forum:viewanyrating allowed.
    #[serde(rename = "canviewanyrating")]
    pub r#canviewanyrating: Option<bool>,
    /// Whether the user has the capability mod/forum:viewallratings allowed.
    #[serde(rename = "canviewallratings")]
    pub r#canviewallratings: Option<bool>,
    /// Whether the user has the capability mod/forum:rate allowed.
    #[serde(rename = "canrate")]
    pub r#canrate: Option<bool>,
    /// Whether the user has the capability mod/forum:postprivatereply allowed.
    #[serde(rename = "canpostprivatereply")]
    pub r#canpostprivatereply: Option<bool>,
    /// Whether the user has the capability mod/forum:readprivatereplies allowed.
    #[serde(rename = "canreadprivatereplies")]
    pub r#canreadprivatereplies: Option<bool>,
    /// Whether the user has the capability mod/forum:createattachment allowed.
    #[serde(rename = "cancreateattachment")]
    pub r#cancreateattachment: Option<bool>,
    /// Whether the user has the capability mod/forum:deleteownpost allowed.
    #[serde(rename = "candeleteownpost")]
    pub r#candeleteownpost: Option<bool>,
    /// Whether the user has the capability mod/forum:deleteanypost allowed.
    #[serde(rename = "candeleteanypost")]
    pub r#candeleteanypost: Option<bool>,
    /// Whether the user has the capability mod/forum:splitdiscussions allowed.
    #[serde(rename = "cansplitdiscussions")]
    pub r#cansplitdiscussions: Option<bool>,
    /// Whether the user has the capability mod/forum:movediscussions allowed.
    #[serde(rename = "canmovediscussions")]
    pub r#canmovediscussions: Option<bool>,
    /// Whether the user has the capability mod/forum:pindiscussions allowed.
    #[serde(rename = "canpindiscussions")]
    pub r#canpindiscussions: Option<bool>,
    /// Whether the user has the capability mod/forum:editanypost allowed.
    #[serde(rename = "caneditanypost")]
    pub r#caneditanypost: Option<bool>,
    /// Whether the user has the capability mod/forum:viewqandawithoutposting allowed.
    #[serde(rename = "canviewqandawithoutposting")]
    pub r#canviewqandawithoutposting: Option<bool>,
    /// Whether the user has the capability mod/forum:viewsubscribers allowed.
    #[serde(rename = "canviewsubscribers")]
    pub r#canviewsubscribers: Option<bool>,
    /// Whether the user has the capability mod/forum:managesubscriptions allowed.
    #[serde(rename = "canmanagesubscriptions")]
    pub r#canmanagesubscriptions: Option<bool>,
    /// Whether the user has the capability mod/forum:postwithoutthrottling allowed.
    #[serde(rename = "canpostwithoutthrottling")]
    pub r#canpostwithoutthrottling: Option<bool>,
    /// Whether the user has the capability mod/forum:exportdiscussion allowed.
    #[serde(rename = "canexportdiscussion")]
    pub r#canexportdiscussion: Option<bool>,
    /// Whether the user has the capability mod/forum:exportforum allowed.
    #[serde(rename = "canexportforum")]
    pub r#canexportforum: Option<bool>,
    /// Whether the user has the capability mod/forum:exportpost allowed.
    #[serde(rename = "canexportpost")]
    pub r#canexportpost: Option<bool>,
    /// Whether the user has the capability mod/forum:exportownpost allowed.
    #[serde(rename = "canexportownpost")]
    pub r#canexportownpost: Option<bool>,
    /// Whether the user has the capability mod/forum:addquestion allowed.
    #[serde(rename = "canaddquestion")]
    pub r#canaddquestion: Option<bool>,
    /// Whether the user has the capability mod/forum:allowforcesubscribe allowed.
    #[serde(rename = "canallowforcesubscribe")]
    pub r#canallowforcesubscribe: Option<bool>,
    /// Whether the user has the capability mod/forum:canposttomygroups allowed.
    #[serde(rename = "cancanposttomygroups")]
    pub r#cancanposttomygroups: Option<bool>,
    /// Whether the user has the capability mod/forum:canoverridediscussionlock allowed.
    #[serde(rename = "cancanoverridediscussionlock")]
    pub r#cancanoverridediscussionlock: Option<bool>,
    /// Whether the user has the capability mod/forum:canoverridecutoff allowed.
    #[serde(rename = "cancanoverridecutoff")]
    pub r#cancanoverridecutoff: Option<bool>,
    /// Whether the user has the capability mod/forum:cantogglefavourite allowed.
    #[serde(rename = "cancantogglefavourite")]
    pub r#cancantogglefavourite: Option<bool>,
    /// Whether the user has the capability mod/forum:grade allowed.
    #[serde(rename = "cangrade")]
    pub r#cangrade: Option<bool>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_forum_get_forum_access_information", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_forum_get_forum_access_information", params)
        .await
}
