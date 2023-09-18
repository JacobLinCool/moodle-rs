use moodle::api::core::course::*;
use moodle::client::{login, MoodleClient};

#[tokio::main]
async fn main() {
    let base_url = std::env::var("MOODLE_URL").unwrap();
    let username = std::env::var("MOODLE_USERNAME").unwrap();
    let password = std::env::var("MOODLE_PASSWORD").unwrap();

    let token = login(&base_url, &username, &password).await.unwrap();
    let mut client = MoodleClient::new(&base_url, &token);

    let result = get_enrolled_courses_by_timeline_classification::call(
        &mut client,
        &mut get_enrolled_courses_by_timeline_classification::Params {
            classification: Some("all".to_string()),
            limit: None,
            offset: None,
            sort: None,
            customfieldname: None,
            customfieldvalue: None,
            searchvalue: None,
        },
    )
    .await;
    let result = result.unwrap();

    println!("Enter a course id from the list below:");
    for course in result.courses.unwrap() {
        if let Some(course_id) = course.id {
            println!("{} - {}", course_id, course.fullname.unwrap());
        }
    }

    let mut course_id = String::new();
    std::io::stdin().read_line(&mut course_id).unwrap();
    let course_id = course_id.trim().parse::<i64>().unwrap();

    let mut params = get_contents::Params {
        courseid: Some(course_id),
        options: None,
    };
    let result = get_contents::call(&mut client, &mut params).await.unwrap();

    for module in result {
        println!("{}", module.name.unwrap());
        for m in module.modules.unwrap() {
            println!("  - {}", m.name.unwrap());
            println!(
                "    {}",
                m.description.unwrap_or("No description.".to_string())
            );
        }
    }
}
