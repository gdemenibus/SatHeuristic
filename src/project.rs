#[derive()]
pub(crate) struct Project<'a> {
    duration: u64,
    id: u64,
    resource: Vec<u64>,
    precedence: Vec<&'a Project<'a>>,
}

impl<'a> Project<'a> {
    pub(crate) fn new(
        duration: u64,
        id: u64,
        resource: Vec<u64>,
        precedence: Vec<&'a Project<'a>>,
    ) -> Self {
        Self {
            duration,
            id,
            resource,
            precedence,
        }
    }

    pub(crate) fn add_presedence(&mut self, other: &'a Project<'a>) {
        self.precedence.push(other);
    }
}

#[cfg(test)]
mod tests {
    use crate::project::Project;

    #[test]
    fn construct() {
        let project = Project {
            duration: 1,
            id: 1,
            resource: vec![1],
            precedence: vec![],
        };
        let mut project1 = Project {
            duration: 1,
            id: 1,
            resource: vec![1],
            precedence: vec![],
        };
        project1.add_presedence(&project);
    }
}
