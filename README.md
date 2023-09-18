# moodle-rs

Moodle Webservice Client in Rust.

## Features

- Parameters and return values are fully typed.
  - However, due to the version differences of Moodle, sometimes you can use `call_raw` to get the raw `serde_json::Value` and parse it yourself.

## Example

See [examples](./examples/).

```rs
use moodle::api::core::course::get_enrolled_courses_by_timeline_classification::{call, Params};
use moodle::client::{login, MoodleClient};

#[tokio::main]
async fn main() {
    let base_url = std::env::var("MOODLE_URL").unwrap();
    let username = std::env::var("MOODLE_USERNAME").unwrap();
    let password = std::env::var("MOODLE_PASSWORD").unwrap();

    let token = login(&base_url, &username, &password).await.unwrap();
    let mut client = MoodleClient::new(&base_url, &token);

    let result = call(
        &mut client,
        &mut Params {
            classification: Some("all".to_string()),
            limit: Some(3),
            offset: Some(0),
            sort: None,
            customfieldname: None,
            customfieldvalue: None,
            searchvalue: None,
        },
    )
    .await;

    println!("{:#?}", result);
}
```
