mod course;
mod fetch;
mod graph;

use std::fs::File;
use std::env::args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // If courses.json exists, read it, otherwise fetch all courses
    let all_courses = if std::path::Path::new("courses.json").exists() {
        let courses = std::fs::read_to_string("courses.json").unwrap();
        serde_json::from_str(&courses).unwrap()
    } else {
        let all_courses = fetch::get_all_courses().await;
        std::fs::write("courses.json", serde_json::to_string(&all_courses).unwrap()).unwrap();
        all_courses
    };

    let course_prefix = args().nth(1).unwrap_or("CSC".to_string());

    let mut f = File::create("course_graph.dot").unwrap();
    graph::graph_program(&mut f, &course_prefix, &all_courses);

    Ok(())
}
