use std::{cell::RefCell, rc::Rc};

use shared::project::Project;

pub struct Schedule<'a> {
    projects: Vec<&'a Project<'a>>,
    resources: Vec<u32>,
}

impl<'a> Schedule<'a> {
    pub fn new(projects: Vec<&'a Project<'a>>, resources: Vec<u32>) -> Self {
        Self {
            projects,
            resources,
        }
    }

    pub fn projects(&'a self) -> &[&Project] {
        self.projects.as_ref()
    }
}
