mod course;
mod fetch;

use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

const BAR_TEMPLATE: &str = "{spinner} {wide_msg} [{bar:100.green/cyan}]";
const PROGRESS_CHARS: &str = "=>-";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut all_courses: Vec<course::Course> = Vec::new();
    let subjects = fetch::get_all_subjects().await;

    let style = ProgressStyle::with_template(BAR_TEMPLATE)
        .unwrap()
        .progress_chars(PROGRESS_CHARS);

    let bar = ProgressBar::new(subjects.len() as u64);

    bar.set_style(style);
    bar.enable_steady_tick(Duration::from_millis(100));

    for subject in subjects.iter() {
        let msg = format!("Fetching courses for {}", subject);
        bar.set_message(msg);
        let courses = fetch::get_all_courses_for_subject(subject).await;
        all_courses.extend(courses);
        bar.inc(1);
    }

    std::fs::write("courses.json", serde_json::to_string(&all_courses).unwrap()).unwrap();

    println!(
        "Fetched {} total courses from {} total subjects",
        all_courses.len(),
        subjects.len()
    );

    // let course = dbg!(fetch::get_all_courses_for_subject("CSC").await);

    Ok(())
}
