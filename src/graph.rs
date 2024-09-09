use std::{io::Write, borrow::Cow};

use crate::course::Course;

struct CourseGraph<'a> {
    subject: String,
    courses: Vec<&'a Course>
}

type Nd = String;
type Ed = (String, String);

pub fn graph_program<W: Write>(out: &mut W, program_code: &str, courses: &Vec<Course>) {
    let courses = CourseGraph {
        subject: program_code.to_string(),
        courses: courses.into_iter().collect()
    };
    dot::render(&courses, out).unwrap();
}

impl<'a> dot::Labeller<'a, Nd, Ed> for CourseGraph<'a> {
    fn graph_id(&'a self) -> dot::Id<'a> { dot::Id::new("wcu").unwrap() }

    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        dot::Id::new(n.clone().to_ascii_lowercase().replace(' ', "_")).unwrap()
    }

    fn node_label<'b>(&'b self, n: &Nd) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(Cow::Owned(n.clone()))
    }
}

impl<'a> dot::GraphWalk<'a, Nd, Ed> for CourseGraph<'a> {
    fn nodes(&self) -> dot::Nodes<'a,Nd> {
        Cow::Owned(self.courses.iter().filter_map(|c| if c.code.prefix == self.subject || self.subject == "*" { Some(c.code.to_string()) } else { None }).collect())
    }

    fn edges(&'a self) -> dot::Edges<'a,Ed> {
        let mut edges: Vec<Ed> = vec![];
        for course in self.courses.iter().filter(|c| c.code.prefix == self.subject) {
            edges.extend(course.pre_requirements.iter().map(|c| {
                (c.clone(), course.code.to_string())
            }));
        }
        Cow::Owned(edges)
    }

    fn source(&self, e: &Ed) -> Nd { e.0.clone() }

    fn target(&self, e: &Ed) -> Nd { e.1.clone() }
}

