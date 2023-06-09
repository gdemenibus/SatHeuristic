use shared::project::Project;

pub struct Schedule<'a> {
    pub(crate) projects: Vec<&'a Project<'a>>,
    pub(crate) resources: Vec<usize>,
}

impl<'a> Schedule<'a> {
    pub fn new(projects: Vec<&'a Project<'a>>, resources: Vec<usize>) -> Self {
        Self {
            projects,
            resources,
        }
    }

    pub fn projects(&'a self) -> &[&Project] {
        self.projects.as_ref()
    }
}
