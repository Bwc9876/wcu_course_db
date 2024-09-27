use std::time::Duration;

use crate::course::{Course, CourseCode};

use indicatif::{ProgressBar, ProgressStyle};
use regex::RegexBuilder;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};

const WCU_CATALOG: &str = "https://catalog.wcupa.edu/ribbit/";
const WCU_COURSE_PREFIXES: &str = "https://catalog.wcupa.edu/general-information/index-course-prefix-guide/course-index/undergraduate/index.xml";
const GET_COURSES_FOR_SUBJECT: &str = "?page=getcourse.rjs&subject=";

const PRE_REQ_ATTR_ID: &str = "Pre / Co requisites:";
const GEN_ED_ATTR_ID: &str = "Gen Ed Attribute:";
const DISTANT_ED_ATTR_ID: &str = "Distance education offering may be available.";
const OFFERED_ATTR_ID: &str = "Typically offered in";

async fn get_with_retry(url: &str) -> String {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let resp = client.get(url).send().await.unwrap();

    resp.text().await.unwrap()
}

fn parse_course_block(code: CourseCode, raw: &str) -> Course {
    let re = RegexBuilder::new(r"<strong>.*&#160;.*\.  (.*)\.  ([+-]?(?:\d*\.)?\d+).*</strong>.*<p.*courseblockdesc.>(.*)<br />\n</p>").dot_matches_new_line(true).build().unwrap();

    let title: String;
    let credits: String;
    let mut description: String = String::from("No Description");
    let mut pre_requirements: Vec<String> = Vec::new();
    let mut gen_ed_fulfillments: Vec<String> = Vec::new();
    let mut distance_available = false;
    let mut offered_terms: Vec<String> = Vec::new();

    if let Some(capture) = re.captures(raw) {
        title = capture
            .get(1)
            .unwrap()
            .as_str()
            .to_string()
            .replace("&amp;", "&");
        credits = capture.get(2).unwrap().as_str().to_string();

        let attrs = capture
            .get(3)
            .unwrap()
            .as_str()
            .trim()
            .split("<br />\n")
            .collect::<Vec<&str>>();

        description = attrs.first().unwrap().to_string();

        for (i, attr) in attrs.iter().enumerate() {
            if i != 0 {
                if attr.contains(DISTANT_ED_ATTR_ID) {
                    distance_available = true;
                } else if attr.starts_with(PRE_REQ_ATTR_ID) {
                    let re = RegexBuilder::new(r"title=.([^ ]*).").build().unwrap();
                    for (j, cap) in re.captures_iter(attr).enumerate() {
                        if j != 0 {
                            let raw = cap.get(1).unwrap().as_str().to_string();
                            let c = raw.replace("&#160;", " ").replace('\"', "");
                            pre_requirements.push(c);
                        }
                    }
                } else if attr.starts_with(GEN_ED_ATTR_ID) {
                    gen_ed_fulfillments = attr
                        .replace(GEN_ED_ATTR_ID, "")
                        .replace("&amp;", "&")
                        .split(&[',', '&'])
                        .map(|s| s.trim().replace('.', "").to_string())
                        .collect::<Vec<String>>();
                } else if attr.starts_with(OFFERED_ATTR_ID) {
                    offered_terms = attr
                        .replace(OFFERED_ATTR_ID, "")
                        .replace("&amp;", "&")
                        .split(&[',', '&'])
                        .map(|s| s.trim().replace('.', "").to_string())
                        .collect::<Vec<String>>();
                }
            }
        }
    } else {
        let re = RegexBuilder::new(r"<strong>.*&#160;.*\.  (.*)\.  (\d+).*</strong>")
            .dot_matches_new_line(true)
            .build()
            .unwrap();
        let capture = re.captures(raw).unwrap();
        title = capture.get(1).unwrap().as_str().to_string();
        credits = capture.get(2).unwrap().as_str().to_string();
    }

    pre_requirements.dedup();

    Course {
        title: title.to_string(),
        code: code.clone(),
        description: description.to_string(),
        credits: credits.parse::<f32>().unwrap(),

        pre_requirements,
        gen_ed_fulfillments,
        distance_available,
        offered_terms,
    }
}

pub async fn get_all_courses_for_subject(subject: &str) -> Vec<Course> {
    let url = format!("{WCU_CATALOG}{GET_COURSES_FOR_SUBJECT}{subject}");

    let body = get_with_retry(&url).await;

    let re = RegexBuilder::new("<course code=\"([A-Z]+) (\\d+)\">\n<!\\[CDATA\\[(.*?)\\]\\]>")
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    let mut courses: Vec<Course> = Vec::new();

    for course in re.captures_iter(&body) {
        let code = course.get(2).unwrap().as_str();
        let raw_body = course.get(3).unwrap().as_str();

        let code = CourseCode::new(subject, code.parse::<u32>().unwrap());

        courses.push(parse_course_block(code, raw_body));
    }

    courses
}

pub async fn get_all_subjects() -> Vec<String> {
    let body = get_with_retry(WCU_COURSE_PREFIXES).await;

    let re = RegexBuilder::new(
        "general-information/index-course-prefix-guide/course-index/undergraduate/(.*?)/",
    )
    .dot_matches_new_line(true)
    .build()
    .unwrap();

    let mut subjects: Vec<String> = Vec::new();

    for subject in re.captures_iter(&body) {
        let subject = subject.get(1).unwrap().as_str().to_string();
        subjects.push(subject.to_string());
    }

    subjects
}

const BAR_TEMPLATE: &str = "{spinner} {wide_msg} [{bar:100.green/cyan}]";
const PROGRESS_CHARS: &str = "=>-";

pub async fn get_all_courses(subjects: &[String]) -> Vec<Course> {
    let mut all_courses: Vec<Course> = Vec::new();

    let style = ProgressStyle::with_template(BAR_TEMPLATE)
        .unwrap()
        .progress_chars(PROGRESS_CHARS);

    let bar = ProgressBar::new(subjects.len() as u64);

    bar.set_style(style);
    bar.enable_steady_tick(Duration::from_millis(100));

    for subject in subjects.iter() {
        let msg = format!("Fetching courses for {}", subject);
        bar.set_message(msg);
        let courses = get_all_courses_for_subject(subject).await;
        all_courses.extend(courses);
        bar.inc(1);
    }
    all_courses
}
