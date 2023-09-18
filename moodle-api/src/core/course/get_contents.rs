use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsOptionsItem {
    /// The expected keys (value format) are: excludemodules (bool) Do not return modules, return only the sections structure excludecontents (bool) Do not return module contents (i.e: files inside a resource) includestealthmodules (bool) Return stealth modules for students in a special section (with id -1) sectionid (int) Return only this section sectionnumber (int) Return only this section with number (order) cmid (int) Return only this module information (among the whole sections structure) modname (string) Return only modules with this name "label, forum, etc..." modid (int) Return only the module with this id (to be used with modname
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// the value of the option, this param is personaly validated in the external function.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Options, used since Moodle 2.9
pub type r#ParamsOptions = Vec<ParamsOptionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// course id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// Options, used since Moodle 2.9
    #[serde(rename = "options")]
    pub r#options: Option<r#ParamsOptions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemModulesItemCompletiondataDetailsItemRulevalue {
    /// status
    #[serde(rename = "status")]
    pub r#status: Option<i64>,
    /// description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemModulesItemCompletiondataDetailsItem {
    /// rulename
    #[serde(rename = "rulename")]
    pub r#rulename: Option<String>,
    #[serde(rename = "rulevalue")]
    pub r#rulevalue: Option<ReturnsItemModulesItemCompletiondataDetailsItemRulevalue>,
}

/// An array of completion details containing the description and status.
pub type r#ReturnsItemModulesItemCompletiondataDetails =
    Vec<ReturnsItemModulesItemCompletiondataDetailsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemModulesItemCompletiondata {
    /// overall completion state of this course module.
    #[serde(rename = "state")]
    pub r#state: Option<i64>,
    /// course completion timestamp.
    #[serde(rename = "timecompleted")]
    pub r#timecompleted: Option<i64>,
    /// user ID that has overridden the completion state of this activity for the user.
    #[serde(rename = "overrideby")]
    pub r#overrideby: Option<i64>,
    /// True if module is used in a condition, false otherwise.
    #[serde(rename = "valueused")]
    pub r#valueused: Option<bool>,
    /// Whether this activity module has completion enabled.
    #[serde(rename = "hascompletion")]
    pub r#hascompletion: Option<bool>,
    /// Whether this activity module instance tracks completion automatically.
    #[serde(rename = "isautomatic")]
    pub r#isautomatic: Option<bool>,
    /// Checks whether completion is being tracked for this user.
    #[serde(rename = "istrackeduser")]
    pub r#istrackeduser: Option<bool>,
    /// Whether this activity is visible to user.
    #[serde(rename = "uservisible")]
    pub r#uservisible: Option<bool>,
    /// An array of completion details containing the description and status.
    #[serde(rename = "details")]
    pub r#details: Option<r#ReturnsItemModulesItemCompletiondataDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemModulesItemDatesItem {
    /// date label
    #[serde(rename = "label")]
    pub r#label: Option<String>,
    /// date timestamp
    #[serde(rename = "timestamp")]
    pub r#timestamp: Option<i64>,
    /// relative date timestamp
    #[serde(rename = "relativeto")]
    pub r#relativeto: Option<i64>,
    /// cm data id
    #[serde(rename = "dataid")]
    pub r#dataid: Option<String>,
}

/// Course dates
pub type r#ReturnsItemModulesItemDates = Vec<ReturnsItemModulesItemDatesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemModulesItemContentsItemTagsItem {
    /// Tag id.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Tag name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The raw, unnormalised name for the tag as entered by users.
    #[serde(rename = "rawname")]
    pub r#rawname: Option<String>,
    /// Whether this tag is standard.
    #[serde(rename = "isstandard")]
    pub r#isstandard: Option<bool>,
    /// Tag collection id.
    #[serde(rename = "tagcollid")]
    pub r#tagcollid: Option<i64>,
    /// Tag instance id.
    #[serde(rename = "taginstanceid")]
    pub r#taginstanceid: Option<i64>,
    /// Context the tag instance belongs to.
    #[serde(rename = "taginstancecontextid")]
    pub r#taginstancecontextid: Option<i64>,
    /// Id of the record tagged.
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// Tag ordering.
    #[serde(rename = "ordering")]
    pub r#ordering: Option<i64>,
    /// Whether the tag is flagged as inappropriate.
    #[serde(rename = "flag")]
    pub r#flag: Option<i64>,
}

/// Tags
pub type r#ReturnsItemModulesItemContentsItemTags = Vec<ReturnsItemModulesItemContentsItemTagsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemModulesItemContentsItem {
    /// a file or a folder or external link
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// filename
    #[serde(rename = "filename")]
    pub r#filename: Option<String>,
    /// filepath
    #[serde(rename = "filepath")]
    pub r#filepath: Option<String>,
    /// filesize
    #[serde(rename = "filesize")]
    pub r#filesize: Option<i64>,
    /// downloadable file url
    #[serde(rename = "fileurl")]
    pub r#fileurl: Option<String>,
    /// Raw content, will be used when type is content
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// Time created
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Time modified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// Content sort order
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<i64>,
    /// File mime type.
    #[serde(rename = "mimetype")]
    pub r#mimetype: Option<String>,
    /// Whether is an external file.
    #[serde(rename = "isexternalfile")]
    pub r#isexternalfile: Option<bool>,
    /// The repository type for external files.
    #[serde(rename = "repositorytype")]
    pub r#repositorytype: Option<String>,
    /// User who added this content to moodle
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Content owner
    #[serde(rename = "author")]
    pub r#author: Option<String>,
    /// Content license
    #[serde(rename = "license")]
    pub r#license: Option<String>,
    /// Tags
    #[serde(rename = "tags")]
    pub r#tags: Option<r#ReturnsItemModulesItemContentsItemTags>,
}

/// Course contents
pub type r#ReturnsItemModulesItemContents = Vec<ReturnsItemModulesItemContentsItem>;

/// Files mime types.
pub type r#ReturnsItemModulesItemContentsinfoMimetypes = Vec<String>;

/// Contents summary information.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemModulesItemContentsinfo {
    /// Total number of files.
    #[serde(rename = "filescount")]
    pub r#filescount: Option<i64>,
    /// Total files size.
    #[serde(rename = "filessize")]
    pub r#filessize: Option<i64>,
    /// Last time files were modified.
    #[serde(rename = "lastmodified")]
    pub r#lastmodified: Option<i64>,
    /// Files mime types.
    #[serde(rename = "mimetypes")]
    pub r#mimetypes: Option<r#ReturnsItemModulesItemContentsinfoMimetypes>,
    /// The repository type for the main file.
    #[serde(rename = "repositorytype")]
    pub r#repositorytype: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemModulesItem {
    /// activity id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// activity url
    #[serde(rename = "url")]
    pub r#url: Option<String>,
    /// activity module name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// instance id
    #[serde(rename = "instance")]
    pub r#instance: Option<i64>,
    /// Activity context id.
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// activity description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// is the module visible
    #[serde(rename = "visible")]
    pub r#visible: Option<i64>,
    /// Is the module visible for the user?
    #[serde(rename = "uservisible")]
    pub r#uservisible: Option<bool>,
    /// Availability information.
    #[serde(rename = "availabilityinfo")]
    pub r#availabilityinfo: Option<String>,
    /// is the module visible on course page
    #[serde(rename = "visibleoncoursepage")]
    pub r#visibleoncoursepage: Option<i64>,
    /// activity icon url
    #[serde(rename = "modicon")]
    pub r#modicon: Option<String>,
    /// activity module type
    #[serde(rename = "modname")]
    pub r#modname: Option<String>,
    /// activity module plural name
    #[serde(rename = "modplural")]
    pub r#modplural: Option<String>,
    /// module availability settings
    #[serde(rename = "availability")]
    pub r#availability: Option<String>,
    /// number of identation in the site
    #[serde(rename = "indent")]
    pub r#indent: Option<i64>,
    /// Onclick action.
    #[serde(rename = "onclick")]
    pub r#onclick: Option<String>,
    /// After link info to be displayed.
    #[serde(rename = "afterlink")]
    pub r#afterlink: Option<String>,
    /// Custom data (JSON encoded).
    #[serde(rename = "customdata")]
    pub r#customdata: Option<String>,
    /// Whether the module has no view page
    #[serde(rename = "noviewlink")]
    pub r#noviewlink: Option<bool>,
    /// Type of completion tracking: 0 means none, 1 manual, 2 automatic.
    #[serde(rename = "completion")]
    pub r#completion: Option<i64>,
    #[serde(rename = "completiondata")]
    pub r#completiondata: Option<ReturnsItemModulesItemCompletiondata>,
    /// The download content value
    #[serde(rename = "downloadcontent")]
    pub r#downloadcontent: Option<i64>,
    /// Course dates
    #[serde(rename = "dates")]
    pub r#dates: Option<r#ReturnsItemModulesItemDates>,
    /// Course contents
    #[serde(rename = "contents")]
    pub r#contents: Option<r#ReturnsItemModulesItemContents>,
    /// Contents summary information.
    #[serde(rename = "contentsinfo")]
    pub r#contentsinfo: Option<ReturnsItemModulesItemContentsinfo>,
}

/// list of module
pub type r#ReturnsItemModules = Vec<ReturnsItemModulesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// Section ID
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Section name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// is the section visible
    #[serde(rename = "visible")]
    pub r#visible: Option<i64>,
    /// Section description
    #[serde(rename = "summary")]
    pub r#summary: Option<String>,
    /// summary format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "summaryformat")]
    pub r#summaryformat: Option<i64>,
    /// Section number inside the course
    #[serde(rename = "section")]
    pub r#section: Option<i64>,
    /// Whether is a section hidden in the course format
    #[serde(rename = "hiddenbynumsections")]
    pub r#hiddenbynumsections: Option<i64>,
    /// Is the section visible for the user?
    #[serde(rename = "uservisible")]
    pub r#uservisible: Option<bool>,
    /// Availability information.
    #[serde(rename = "availabilityinfo")]
    pub r#availabilityinfo: Option<String>,
    /// list of module
    #[serde(rename = "modules")]
    pub r#modules: Option<r#ReturnsItemModules>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_course_get_contents", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_course_get_contents", params).await
}
