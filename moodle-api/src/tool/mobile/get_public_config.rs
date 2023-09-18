use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsIdentityprovidersItem {
    /// The identity provider name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The icon URL for the provider.
    #[serde(rename = "iconurl")]
    pub r#iconurl: Option<String>,
    /// The URL of the provider.
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}

/// Identity providers
pub type r#ReturnsIdentityproviders = Vec<ReturnsIdentityprovidersItem>;

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
    /// Site URL.
    #[serde(rename = "wwwroot")]
    pub r#wwwroot: Option<String>,
    /// Site https URL (if httpslogin is enabled).
    #[serde(rename = "httpswwwroot")]
    pub r#httpswwwroot: Option<String>,
    /// Site name.
    #[serde(rename = "sitename")]
    pub r#sitename: Option<String>,
    /// Whether guest login is enabled.
    #[serde(rename = "guestlogin")]
    pub r#guestlogin: Option<i64>,
    /// Values: 0 for No, 1 for Yes, 2 for optional.
    #[serde(rename = "rememberusername")]
    pub r#rememberusername: Option<i64>,
    /// Whether log in via email is enabled.
    #[serde(rename = "authloginviaemail")]
    pub r#authloginviaemail: Option<i64>,
    /// Authentication method for user registration.
    #[serde(rename = "registerauth")]
    pub r#registerauth: Option<String>,
    /// Forgotten password URL.
    #[serde(rename = "forgottenpasswordurl")]
    pub r#forgottenpasswordurl: Option<String>,
    /// Authentication instructions.
    #[serde(rename = "authinstructions")]
    pub r#authinstructions: Option<String>,
    /// Whether auth none is enabled.
    #[serde(rename = "authnoneenabled")]
    pub r#authnoneenabled: Option<i64>,
    /// Whether Web Services are enabled.
    #[serde(rename = "enablewebservices")]
    pub r#enablewebservices: Option<i64>,
    /// Whether the Mobile service is enabled.
    #[serde(rename = "enablemobilewebservice")]
    pub r#enablemobilewebservice: Option<i64>,
    /// Whether site maintenance is enabled.
    #[serde(rename = "maintenanceenabled")]
    pub r#maintenanceenabled: Option<i64>,
    /// Maintenance message.
    #[serde(rename = "maintenancemessage")]
    pub r#maintenancemessage: Option<String>,
    /// The site logo URL
    #[serde(rename = "logourl")]
    pub r#logourl: Option<String>,
    /// The site compact logo URL
    #[serde(rename = "compactlogourl")]
    pub r#compactlogourl: Option<String>,
    /// The type of login. 1 for app, 2 for browser, 3 for embedded.
    #[serde(rename = "typeoflogin")]
    pub r#typeoflogin: Option<i64>,
    /// SSO login launch URL.
    #[serde(rename = "launchurl")]
    pub r#launchurl: Option<String>,
    /// Mobile custom CSS theme
    #[serde(rename = "mobilecssurl")]
    pub r#mobilecssurl: Option<String>,
    /// Disabled features in the app
    #[serde(rename = "tool_mobile_disabledfeatures")]
    pub r#tool_mobile_disabledfeatures: Option<String>,
    /// Identity providers
    #[serde(rename = "identityproviders")]
    pub r#identityproviders: Option<r#ReturnsIdentityproviders>,
    /// Default site country
    #[serde(rename = "country")]
    pub r#country: Option<String>,
    /// Whether age digital consent verification is enabled.
    #[serde(rename = "agedigitalconsentverification")]
    pub r#agedigitalconsentverification: Option<bool>,
    /// Site support contact name (only if age verification is enabled).
    #[serde(rename = "supportname")]
    pub r#supportname: Option<String>,
    /// Site support contact email (only if age verification is enabled).
    #[serde(rename = "supportemail")]
    pub r#supportemail: Option<String>,
    /// Site support page link.
    #[serde(rename = "supportpage")]
    pub r#supportpage: Option<String>,
    /// Determines who has access to contact site support.
    #[serde(rename = "supportavailability")]
    pub r#supportavailability: Option<i64>,
    /// Whether to detect default language from browser setting.
    #[serde(rename = "autolang")]
    pub r#autolang: Option<i64>,
    /// Default language for the site.
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
    /// Whether the language menu should be displayed.
    #[serde(rename = "langmenu")]
    pub r#langmenu: Option<i64>,
    /// Languages on language menu.
    #[serde(rename = "langlist")]
    pub r#langlist: Option<String>,
    /// Sitewide locale.
    #[serde(rename = "locale")]
    pub r#locale: Option<String>,
    /// Minimum required version to access.
    #[serde(rename = "tool_mobile_minimumversion")]
    pub r#tool_mobile_minimumversion: Option<String>,
    /// iOS app's unique identifier.
    #[serde(rename = "tool_mobile_iosappid")]
    pub r#tool_mobile_iosappid: Option<String>,
    /// Android app's unique identifier.
    #[serde(rename = "tool_mobile_androidappid")]
    pub r#tool_mobile_androidappid: Option<String>,
    /// App download page.
    #[serde(rename = "tool_mobile_setuplink")]
    pub r#tool_mobile_setuplink: Option<String>,
    /// QR login configuration.
    #[serde(rename = "tool_mobile_qrcodetype")]
    pub r#tool_mobile_qrcodetype: Option<i64>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("tool_mobile_get_public_config", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("tool_mobile_get_public_config", params).await
}
