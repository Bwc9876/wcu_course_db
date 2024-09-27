mod course;
mod fetch;
//mod graph;

use course::Course;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct DatabaseOutput {
    courses: Vec<Course>,
    prefixes: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subjects = fetch::get_all_subjects().await;
    let courses = fetch::get_all_courses(&subjects).await;

    let out = DatabaseOutput {
        courses,
        prefixes: subjects,
    };

    std::fs::write("courses.json", serde_json::to_string(&out).unwrap()).unwrap();

    Ok(())
}
