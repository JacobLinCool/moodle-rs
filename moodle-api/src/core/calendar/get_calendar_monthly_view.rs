use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Year to be viewed
    #[serde(rename = "year")]
    pub r#year: Option<i64>,
    /// Month to be viewed
    #[serde(rename = "month")]
    pub r#month: Option<i64>,
    /// Course being viewed
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// Category being viewed
    #[serde(rename = "categoryid")]
    pub r#categoryid: Option<i64>,
    /// Whether to show course navigation
    #[serde(rename = "includenavigation")]
    pub r#includenavigation: Option<bool>,
    /// Whether to return the mini month view or not
    #[serde(rename = "mini")]
    pub r#mini: Option<bool>,
    /// Day to be viewed
    #[serde(rename = "day")]
    pub r#day: Option<i64>,
    /// The view mode of the calendar
    #[serde(rename = "view")]
    pub r#view: Option<String>,
}

/// prepadding
pub type r#ReturnsWeeksItemPrepadding = Vec<i64>;

/// postpadding
pub type r#ReturnsWeeksItemPostpadding = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWeeksItemDaysItemEventsItemIcon {
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
pub struct ReturnsWeeksItemDaysItemEventsItemCategory {
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
pub struct ReturnsWeeksItemDaysItemEventsItemCourse {
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
pub struct ReturnsWeeksItemDaysItemEventsItemSubscription {
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
pub struct ReturnsWeeksItemDaysItemEventsItemAction {
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
pub struct ReturnsWeeksItemDaysItemEventsItem {
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
    pub r#icon: Option<ReturnsWeeksItemDaysItemEventsItemIcon>,
    #[serde(rename = "category")]
    pub r#category: Option<ReturnsWeeksItemDaysItemEventsItemCategory>,
    #[serde(rename = "course")]
    pub r#course: Option<ReturnsWeeksItemDaysItemEventsItemCourse>,
    #[serde(rename = "subscription")]
    pub r#subscription: Option<ReturnsWeeksItemDaysItemEventsItemSubscription>,
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
    pub r#action: Option<ReturnsWeeksItemDaysItemEventsItemAction>,
    /// purpose
    #[serde(rename = "purpose")]
    pub r#purpose: Option<String>,
    /// url
    #[serde(rename = "url")]
    pub r#url: Option<String>,
    /// islastday
    #[serde(rename = "islastday")]
    pub r#islastday: Option<bool>,
    /// popupname
    #[serde(rename = "popupname")]
    pub r#popupname: Option<String>,
    /// mindaytimestamp
    #[serde(rename = "mindaytimestamp")]
    pub r#mindaytimestamp: Option<i64>,
    /// mindayerror
    #[serde(rename = "mindayerror")]
    pub r#mindayerror: Option<String>,
    /// maxdaytimestamp
    #[serde(rename = "maxdaytimestamp")]
    pub r#maxdaytimestamp: Option<i64>,
    /// maxdayerror
    #[serde(rename = "maxdayerror")]
    pub r#maxdayerror: Option<String>,
    /// draggable
    #[serde(rename = "draggable")]
    pub r#draggable: Option<bool>,
}

/// events
pub type r#ReturnsWeeksItemDaysItemEvents = Vec<ReturnsWeeksItemDaysItemEventsItem>;

/// calendareventtypes
pub type r#ReturnsWeeksItemDaysItemCalendareventtypes = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWeeksItemDaysItem {
    /// seconds
    #[serde(rename = "seconds")]
    pub r#seconds: Option<i64>,
    /// minutes
    #[serde(rename = "minutes")]
    pub r#minutes: Option<i64>,
    /// hours
    #[serde(rename = "hours")]
    pub r#hours: Option<i64>,
    /// mday
    #[serde(rename = "mday")]
    pub r#mday: Option<i64>,
    /// wday
    #[serde(rename = "wday")]
    pub r#wday: Option<i64>,
    /// year
    #[serde(rename = "year")]
    pub r#year: Option<i64>,
    /// yday
    #[serde(rename = "yday")]
    pub r#yday: Option<i64>,
    /// istoday
    #[serde(rename = "istoday")]
    pub r#istoday: Option<bool>,
    /// isweekend
    #[serde(rename = "isweekend")]
    pub r#isweekend: Option<bool>,
    /// timestamp
    #[serde(rename = "timestamp")]
    pub r#timestamp: Option<i64>,
    /// neweventtimestamp
    #[serde(rename = "neweventtimestamp")]
    pub r#neweventtimestamp: Option<i64>,
    /// viewdaylink
    #[serde(rename = "viewdaylink")]
    pub r#viewdaylink: Option<String>,
    /// viewdaylinktitle
    #[serde(rename = "viewdaylinktitle")]
    pub r#viewdaylinktitle: Option<String>,
    /// events
    #[serde(rename = "events")]
    pub r#events: Option<r#ReturnsWeeksItemDaysItemEvents>,
    /// hasevents
    #[serde(rename = "hasevents")]
    pub r#hasevents: Option<bool>,
    /// calendareventtypes
    #[serde(rename = "calendareventtypes")]
    pub r#calendareventtypes: Option<r#ReturnsWeeksItemDaysItemCalendareventtypes>,
    /// previousperiod
    #[serde(rename = "previousperiod")]
    pub r#previousperiod: Option<i64>,
    /// nextperiod
    #[serde(rename = "nextperiod")]
    pub r#nextperiod: Option<i64>,
    /// navigation
    #[serde(rename = "navigation")]
    pub r#navigation: Option<String>,
    /// haslastdayofevent
    #[serde(rename = "haslastdayofevent")]
    pub r#haslastdayofevent: Option<bool>,
    /// popovertitle
    #[serde(rename = "popovertitle")]
    pub r#popovertitle: Option<String>,
    /// daytitle
    #[serde(rename = "daytitle")]
    pub r#daytitle: Option<String>,
}

/// days
pub type r#ReturnsWeeksItemDays = Vec<ReturnsWeeksItemDaysItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWeeksItem {
    /// prepadding
    #[serde(rename = "prepadding")]
    pub r#prepadding: Option<r#ReturnsWeeksItemPrepadding>,
    /// postpadding
    #[serde(rename = "postpadding")]
    pub r#postpadding: Option<r#ReturnsWeeksItemPostpadding>,
    /// days
    #[serde(rename = "days")]
    pub r#days: Option<r#ReturnsWeeksItemDays>,
}

/// weeks
pub type r#ReturnsWeeks = Vec<ReturnsWeeksItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDaynamesItem {
    /// dayno
    #[serde(rename = "dayno")]
    pub r#dayno: Option<i64>,
    /// shortname
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// fullname
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
}

/// daynames
pub type r#ReturnsDaynames = Vec<ReturnsDaynamesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDate {
    /// seconds
    #[serde(rename = "seconds")]
    pub r#seconds: Option<i64>,
    /// minutes
    #[serde(rename = "minutes")]
    pub r#minutes: Option<i64>,
    /// hours
    #[serde(rename = "hours")]
    pub r#hours: Option<i64>,
    /// mday
    #[serde(rename = "mday")]
    pub r#mday: Option<i64>,
    /// wday
    #[serde(rename = "wday")]
    pub r#wday: Option<i64>,
    /// mon
    #[serde(rename = "mon")]
    pub r#mon: Option<i64>,
    /// year
    #[serde(rename = "year")]
    pub r#year: Option<i64>,
    /// yday
    #[serde(rename = "yday")]
    pub r#yday: Option<i64>,
    /// weekday
    #[serde(rename = "weekday")]
    pub r#weekday: Option<String>,
    /// month
    #[serde(rename = "month")]
    pub r#month: Option<String>,
    /// timestamp
    #[serde(rename = "timestamp")]
    pub r#timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPreviousperiod {
    /// seconds
    #[serde(rename = "seconds")]
    pub r#seconds: Option<i64>,
    /// minutes
    #[serde(rename = "minutes")]
    pub r#minutes: Option<i64>,
    /// hours
    #[serde(rename = "hours")]
    pub r#hours: Option<i64>,
    /// mday
    #[serde(rename = "mday")]
    pub r#mday: Option<i64>,
    /// wday
    #[serde(rename = "wday")]
    pub r#wday: Option<i64>,
    /// mon
    #[serde(rename = "mon")]
    pub r#mon: Option<i64>,
    /// year
    #[serde(rename = "year")]
    pub r#year: Option<i64>,
    /// yday
    #[serde(rename = "yday")]
    pub r#yday: Option<i64>,
    /// weekday
    #[serde(rename = "weekday")]
    pub r#weekday: Option<String>,
    /// month
    #[serde(rename = "month")]
    pub r#month: Option<String>,
    /// timestamp
    #[serde(rename = "timestamp")]
    pub r#timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsNextperiod {
    /// seconds
    #[serde(rename = "seconds")]
    pub r#seconds: Option<i64>,
    /// minutes
    #[serde(rename = "minutes")]
    pub r#minutes: Option<i64>,
    /// hours
    #[serde(rename = "hours")]
    pub r#hours: Option<i64>,
    /// mday
    #[serde(rename = "mday")]
    pub r#mday: Option<i64>,
    /// wday
    #[serde(rename = "wday")]
    pub r#wday: Option<i64>,
    /// mon
    #[serde(rename = "mon")]
    pub r#mon: Option<i64>,
    /// year
    #[serde(rename = "year")]
    pub r#year: Option<i64>,
    /// yday
    #[serde(rename = "yday")]
    pub r#yday: Option<i64>,
    /// weekday
    #[serde(rename = "weekday")]
    pub r#weekday: Option<String>,
    /// month
    #[serde(rename = "month")]
    pub r#month: Option<String>,
    /// timestamp
    #[serde(rename = "timestamp")]
    pub r#timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// url
    #[serde(rename = "url")]
    pub r#url: Option<String>,
    /// courseid
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// categoryid
    #[serde(rename = "categoryid")]
    pub r#categoryid: Option<i64>,
    /// filter_selector
    #[serde(rename = "filter_selector")]
    pub r#filter_selector: Option<String>,
    /// weeks
    #[serde(rename = "weeks")]
    pub r#weeks: Option<r#ReturnsWeeks>,
    /// daynames
    #[serde(rename = "daynames")]
    pub r#daynames: Option<r#ReturnsDaynames>,
    /// view
    #[serde(rename = "view")]
    pub r#view: Option<String>,
    #[serde(rename = "date")]
    pub r#date: Option<ReturnsDate>,
    /// periodname
    #[serde(rename = "periodname")]
    pub r#periodname: Option<String>,
    /// includenavigation
    #[serde(rename = "includenavigation")]
    pub r#includenavigation: Option<bool>,
    /// initialeventsloaded
    #[serde(rename = "initialeventsloaded")]
    pub r#initialeventsloaded: Option<bool>,
    #[serde(rename = "previousperiod")]
    pub r#previousperiod: Option<ReturnsPreviousperiod>,
    /// previousperiodlink
    #[serde(rename = "previousperiodlink")]
    pub r#previousperiodlink: Option<String>,
    /// previousperiodname
    #[serde(rename = "previousperiodname")]
    pub r#previousperiodname: Option<String>,
    #[serde(rename = "nextperiod")]
    pub r#nextperiod: Option<ReturnsNextperiod>,
    /// nextperiodname
    #[serde(rename = "nextperiodname")]
    pub r#nextperiodname: Option<String>,
    /// nextperiodlink
    #[serde(rename = "nextperiodlink")]
    pub r#nextperiodlink: Option<String>,
    /// larrow
    #[serde(rename = "larrow")]
    pub r#larrow: Option<String>,
    /// rarrow
    #[serde(rename = "rarrow")]
    pub r#rarrow: Option<String>,
    /// defaulteventcontext
    #[serde(rename = "defaulteventcontext")]
    pub r#defaulteventcontext: Option<i64>,
    /// calendarinstanceid
    #[serde(rename = "calendarinstanceid")]
    pub r#calendarinstanceid: Option<i64>,
    /// viewingmonth
    #[serde(rename = "viewingmonth")]
    pub r#viewingmonth: Option<bool>,
    /// showviewselector
    #[serde(rename = "showviewselector")]
    pub r#showviewselector: Option<bool>,
    /// viewinginblock
    #[serde(rename = "viewinginblock")]
    pub r#viewinginblock: Option<bool>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_calendar_get_calendar_monthly_view", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_calendar_get_calendar_monthly_view", params)
        .await
}
