use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Time sort from
    #[serde(rename = "timesortfrom")]
    pub r#timesortfrom: Option<i64>,
    /// Time sort to
    #[serde(rename = "timesortto")]
    pub r#timesortto: Option<i64>,
    /// The last seen event id
    #[serde(rename = "aftereventid")]
    pub r#aftereventid: Option<i64>,
    /// Limit number
    #[serde(rename = "limitnum")]
    pub r#limitnum: Option<i64>,
    /// Limit the events to courses the user is not suspended in
    #[serde(rename = "limittononsuspendedevents")]
    pub r#limittononsuspendedevents: Option<bool>,
    /// The user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The value a user wishes to search against
    #[serde(rename = "searchvalue")]
    pub r#searchvalue: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEventsItemIcon {
    /// key
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// alttext
    #[serde(rename = "alttext")]
    pub r#alttext: Option<String>,
    /// iconurl
    #[serde(rename = "iconurl")]
    pub r#iconurl: Option<String>,
    /// iconclass
    #[serde(rename = "iconclass")]
    pub r#iconclass: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEventsItemCategory {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// idnumber
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// parent
    #[serde(rename = "parent")]
    pub r#parent: Option<i64>,
    /// coursecount
    #[serde(rename = "coursecount")]
    pub r#coursecount: Option<i64>,
    /// visible
    #[serde(rename = "visible")]
    pub r#visible: Option<i64>,
    /// timemodified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// depth
    #[serde(rename = "depth")]
    pub r#depth: Option<i64>,
    /// nestedname
    #[serde(rename = "nestedname")]
    pub r#nestedname: Option<String>,
    /// url
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEventsItemCourse {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// fullname
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// shortname
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// idnumber
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// summary
    #[serde(rename = "summary")]
    pub r#summary: Option<String>,
    /// summary format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "summaryformat")]
    pub r#summaryformat: Option<i64>,
    /// startdate
    #[serde(rename = "startdate")]
    pub r#startdate: Option<i64>,
    /// enddate
    #[serde(rename = "enddate")]
    pub r#enddate: Option<i64>,
    /// visible
    #[serde(rename = "visible")]
    pub r#visible: Option<bool>,
    /// showactivitydates
    #[serde(rename = "showactivitydates")]
    pub r#showactivitydates: Option<bool>,
    /// showcompletionconditions
    #[serde(rename = "showcompletionconditions")]
    pub r#showcompletionconditions: Option<bool>,
    /// pdfexportfont
    #[serde(rename = "pdfexportfont")]
    pub r#pdfexportfont: Option<String>,
    /// fullnamedisplay
    #[serde(rename = "fullnamedisplay")]
    pub r#fullnamedisplay: Option<String>,
    /// viewurl
    #[serde(rename = "viewurl")]
    pub r#viewurl: Option<String>,
    /// courseimage
    #[serde(rename = "courseimage")]
    pub r#courseimage: Option<String>,
    /// progress
    #[serde(rename = "progress")]
    pub r#progress: Option<i64>,
    /// hasprogress
    #[serde(rename = "hasprogress")]
    pub r#hasprogress: Option<bool>,
    /// isfavourite
    #[serde(rename = "isfavourite")]
    pub r#isfavourite: Option<bool>,
    /// hidden
    #[serde(rename = "hidden")]
    pub r#hidden: Option<bool>,
    /// timeaccess
    #[serde(rename = "timeaccess")]
    pub r#timeaccess: Option<i64>,
    /// showshortname
    #[serde(rename = "showshortname")]
    pub r#showshortname: Option<bool>,
    /// coursecategory
    #[serde(rename = "coursecategory")]
    pub r#coursecategory: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEventsItemSubscription {
    /// displayeventsource
    #[serde(rename = "displayeventsource")]
    pub r#displayeventsource: Option<bool>,
    /// subscriptionname
    #[serde(rename = "subscriptionname")]
    pub r#subscriptionname: Option<String>,
    /// subscriptionurl
    #[serde(rename = "subscriptionurl")]
    pub r#subscriptionurl: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEventsItemAction {
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// url
    #[serde(rename = "url")]
    pub r#url: Option<String>,
    /// itemcount
    #[serde(rename = "itemcount")]
    pub r#itemcount: Option<i64>,
    /// actionable
    #[serde(rename = "actionable")]
    pub r#actionable: Option<bool>,
    /// showitemcount
    #[serde(rename = "showitemcount")]
    pub r#showitemcount: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEventsItem {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// location
    #[serde(rename = "location")]
    pub r#location: Option<String>,
    /// categoryid
    #[serde(rename = "categoryid")]
    pub r#categoryid: Option<i64>,
    /// groupid
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// userid
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// repeatid
    #[serde(rename = "repeatid")]
    pub r#repeatid: Option<i64>,
    /// eventcount
    #[serde(rename = "eventcount")]
    pub r#eventcount: Option<i64>,
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// modulename
    #[serde(rename = "modulename")]
    pub r#modulename: Option<String>,
    /// activityname
    #[serde(rename = "activityname")]
    pub r#activityname: Option<String>,
    /// activitystr
    #[serde(rename = "activitystr")]
    pub r#activitystr: Option<String>,
    /// instance
    #[serde(rename = "instance")]
    pub r#instance: Option<i64>,
    /// eventtype
    #[serde(rename = "eventtype")]
    pub r#eventtype: Option<String>,
    /// timestart
    #[serde(rename = "timestart")]
    pub r#timestart: Option<i64>,
    /// timeduration
    #[serde(rename = "timeduration")]
    pub r#timeduration: Option<i64>,
    /// timesort
    #[serde(rename = "timesort")]
    pub r#timesort: Option<i64>,
    /// timeusermidnight
    #[serde(rename = "timeusermidnight")]
    pub r#timeusermidnight: Option<i64>,
    /// visible
    #[serde(rename = "visible")]
    pub r#visible: Option<i64>,
    /// timemodified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// overdue
    #[serde(rename = "overdue")]
    pub r#overdue: Option<bool>,
    #[serde(rename = "icon")]
    pub r#icon: Option<ReturnsEventsItemIcon>,
    #[serde(rename = "category")]
    pub r#category: Option<ReturnsEventsItemCategory>,
    #[serde(rename = "course")]
    pub r#course: Option<ReturnsEventsItemCourse>,
    #[serde(rename = "subscription")]
    pub r#subscription: Option<ReturnsEventsItemSubscription>,
    /// canedit
    #[serde(rename = "canedit")]
    pub r#canedit: Option<bool>,
    /// candelete
    #[serde(rename = "candelete")]
    pub r#candelete: Option<bool>,
    /// deleteurl
    #[serde(rename = "deleteurl")]
    pub r#deleteurl: Option<String>,
    /// editurl
    #[serde(rename = "editurl")]
    pub r#editurl: Option<String>,
    /// viewurl
    #[serde(rename = "viewurl")]
    pub r#viewurl: Option<String>,
    /// formattedtime
    #[serde(rename = "formattedtime")]
    pub r#formattedtime: Option<String>,
    /// formattedlocation
    #[serde(rename = "formattedlocation")]
    pub r#formattedlocation: Option<String>,
    /// isactionevent
    #[serde(rename = "isactionevent")]
    pub r#isactionevent: Option<bool>,
    /// iscourseevent
    #[serde(rename = "iscourseevent")]
    pub r#iscourseevent: Option<bool>,
    /// iscategoryevent
    #[serde(rename = "iscategoryevent")]
    pub r#iscategoryevent: Option<bool>,
    /// groupname
    #[serde(rename = "groupname")]
    pub r#groupname: Option<String>,
    /// normalisedeventtype
    #[serde(rename = "normalisedeventtype")]
    pub r#normalisedeventtype: Option<String>,
    /// normalisedeventtypetext
    #[serde(rename = "normalisedeventtypetext")]
    pub r#normalisedeventtypetext: Option<String>,
    #[serde(rename = "action")]
    pub r#action: Option<ReturnsEventsItemAction>,
    /// purpose
    #[serde(rename = "purpose")]
    pub r#purpose: Option<String>,
    /// url
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}

/// events
pub type r#ReturnsEvents = Vec<ReturnsEventsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// events
    #[serde(rename = "events")]
    pub r#events: Option<r#ReturnsEvents>,
    /// firstid
    #[serde(rename = "firstid")]
    pub r#firstid: Option<i64>,
    /// lastid
    #[serde(rename = "lastid")]
    pub r#lastid: Option<i64>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_calendar_get_action_events_by_timesort", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_calendar_get_action_events_by_timesort", params)
        .await
}
