use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Badges only for this user id, empty for current user
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Filter badges by course id, empty all the courses
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// The page of records to return.
    #[serde(rename = "page")]
    pub r#page: Option<i64>,
    /// The number of records to return per page
    #[serde(rename = "perpage")]
    pub r#perpage: Option<i64>,
    /// A simple string to search for
    #[serde(rename = "search")]
    pub r#search: Option<String>,
    /// Whether to return only public badges
    #[serde(rename = "onlypublic")]
    pub r#onlypublic: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsBadgesItemEndorsement {
    /// Endorsement id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Badge id
    #[serde(rename = "badgeid")]
    pub r#badgeid: Option<i64>,
    /// Endorsement issuer name
    #[serde(rename = "issuername")]
    pub r#issuername: Option<String>,
    /// Endorsement issuer URL
    #[serde(rename = "issuerurl")]
    pub r#issuerurl: Option<String>,
    /// Endorsement issuer email
    #[serde(rename = "issueremail")]
    pub r#issueremail: Option<String>,
    /// Claim URL
    #[serde(rename = "claimid")]
    pub r#claimid: Option<String>,
    /// Claim comment
    #[serde(rename = "claimcomment")]
    pub r#claimcomment: Option<String>,
    /// Date issued
    #[serde(rename = "dateissued")]
    pub r#dateissued: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsBadgesItemAlignmentItem {
    /// Alignment id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Badge id
    #[serde(rename = "badgeid")]
    pub r#badgeid: Option<i64>,
    /// Target name
    #[serde(rename = "targetName")]
    pub r#target_name: Option<String>,
    /// Target URL
    #[serde(rename = "targetUrl")]
    pub r#target_url: Option<String>,
    /// Target description
    #[serde(rename = "targetDescription")]
    pub r#target_description: Option<String>,
    /// Target framework
    #[serde(rename = "targetFramework")]
    pub r#target_framework: Option<String>,
    /// Target code
    #[serde(rename = "targetCode")]
    pub r#target_code: Option<String>,
}

/// Badge alignments
pub type r#ReturnsBadgesItemAlignment = Vec<ReturnsBadgesItemAlignmentItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsBadgesItemRelatedbadgesItem {
    /// Badge id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Badge name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Version
    #[serde(rename = "version")]
    pub r#version: Option<String>,
    /// Language
    #[serde(rename = "language")]
    pub r#language: Option<String>,
    /// Type
    #[serde(rename = "type")]
    pub r#type: Option<i64>,
}

/// Related badges
pub type r#ReturnsBadgesItemRelatedbadges = Vec<ReturnsBadgesItemRelatedbadgesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsBadgesItem {
    /// Badge id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Badge name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Badge description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Time created
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Time modified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// User created
    #[serde(rename = "usercreated")]
    pub r#usercreated: Option<i64>,
    /// User modified
    #[serde(rename = "usermodified")]
    pub r#usermodified: Option<i64>,
    /// Issuer name
    #[serde(rename = "issuername")]
    pub r#issuername: Option<String>,
    /// Issuer URL
    #[serde(rename = "issuerurl")]
    pub r#issuerurl: Option<String>,
    /// Issuer contact
    #[serde(rename = "issuercontact")]
    pub r#issuercontact: Option<String>,
    /// Expire date
    #[serde(rename = "expiredate")]
    pub r#expiredate: Option<i64>,
    /// Expire period
    #[serde(rename = "expireperiod")]
    pub r#expireperiod: Option<i64>,
    /// Type
    #[serde(rename = "type")]
    pub r#type: Option<i64>,
    /// Course id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// Message
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// Message subject
    #[serde(rename = "messagesubject")]
    pub r#messagesubject: Option<String>,
    /// Attachment
    #[serde(rename = "attachment")]
    pub r#attachment: Option<i64>,
    /// Whether to notify when badge is awarded
    #[serde(rename = "notification")]
    pub r#notification: Option<i64>,
    /// Next cron
    #[serde(rename = "nextcron")]
    pub r#nextcron: Option<i64>,
    /// Status
    #[serde(rename = "status")]
    pub r#status: Option<i64>,
    /// Issued id
    #[serde(rename = "issuedid")]
    pub r#issuedid: Option<i64>,
    /// Unique hash
    #[serde(rename = "uniquehash")]
    pub r#uniquehash: Option<String>,
    /// Date issued
    #[serde(rename = "dateissued")]
    pub r#dateissued: Option<i64>,
    /// Date expire
    #[serde(rename = "dateexpire")]
    pub r#dateexpire: Option<i64>,
    /// Visible
    #[serde(rename = "visible")]
    pub r#visible: Option<i64>,
    /// User email
    #[serde(rename = "email")]
    pub r#email: Option<String>,
    /// Version
    #[serde(rename = "version")]
    pub r#version: Option<String>,
    /// Language
    #[serde(rename = "language")]
    pub r#language: Option<String>,
    /// Name of the image author
    #[serde(rename = "imageauthorname")]
    pub r#imageauthorname: Option<String>,
    /// Email of the image author
    #[serde(rename = "imageauthoremail")]
    pub r#imageauthoremail: Option<String>,
    /// URL of the image author
    #[serde(rename = "imageauthorurl")]
    pub r#imageauthorurl: Option<String>,
    /// Caption of the image
    #[serde(rename = "imagecaption")]
    pub r#imagecaption: Option<String>,
    /// Badge URL
    #[serde(rename = "badgeurl")]
    pub r#badgeurl: Option<String>,
    #[serde(rename = "endorsement")]
    pub r#endorsement: Option<ReturnsBadgesItemEndorsement>,
    /// Badge alignments
    #[serde(rename = "alignment")]
    pub r#alignment: Option<r#ReturnsBadgesItemAlignment>,
    /// Related badges
    #[serde(rename = "relatedbadges")]
    pub r#relatedbadges: Option<r#ReturnsBadgesItemRelatedbadges>,
}

pub type r#ReturnsBadges = Vec<ReturnsBadgesItem>;

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
    #[serde(rename = "badges")]
    pub r#badges: Option<r#ReturnsBadges>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_badges_get_user_badges", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_badges_get_user_badges", params).await
}
