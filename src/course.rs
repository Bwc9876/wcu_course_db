use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
/// Represents a full course code ex BIO 110, FYE 100-G
pub struct CourseCode {
    /// Prefix for the course ex: BIO, PSY, CSC
    pub prefix: String,
    /// Number for the course ex: 112, 101, 435
    pub number: u32,
}

impl CourseCode {
    pub fn new(prefix: &str, number: u32) -> Self {
        Self {
            prefix: prefix.to_string(),
            number,
        }
    }
}

impl fmt::Display for CourseCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.prefix, self.number)
    }
}

impl fmt::Debug for CourseCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.prefix, self.number)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Course {
    pub title: String,
    pub code: CourseCode,
    pub description: String,
    pub credits: f32,

    pub pre_requirements: Vec<String>,
    pub gen_ed_fulfillments: Vec<String>,
    pub distance_available: bool,
    pub offered_terms: Vec<String>,
}

impl Course {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_course_code_fmt() {
        let code = CourseCode {
            prefix: "BIO".to_string(),
            number: 110,
        };
        assert_eq!(code.to_string(), "BIO 110");
        let code = CourseCode {
            prefix: "BIO".to_string(),
            number: 110,
        };
        assert_eq!(code.to_string(), "BIO 110");
    }
}
