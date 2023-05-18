use crate::segment::Segment;
use std::{cell::RefCell, rc::Rc};
#[derive()]
pub(crate) struct Project<'a> {
    duration: u32,
    id: u64,
    resource: Vec<u64>,
    precedence: Vec<&'a Project<'a>>,
}

impl<'a> Project<'a> {
    pub(crate) fn new(
        duration: u32,
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
    pub(crate) fn generate_segments(&self) -> Vec<Rc<Segment>> {
        let mut segments: Vec<Rc<Segment>> = Vec::new();
        //TODO: Replace this ID generation with a more distinct one
        let id = 0;

        for x in 1..self.duration + 1 {
            for y in 1..self.duration - x + 2 {
                // find the precednece
                // rule is: if x + y of old equal x of new, then new depends on old
                let precedents = segments
                    .iter()
                    .map(Rc::clone)
                    .filter(|old| old.start_jiff + old.duration == x)
                    .collect();
                let seg = Segment::new(x, y, RefCell::new(precedents), id, self.id);
                segments.push(Rc::new(seg));
            }
        }
        segments
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
    #[test]
    fn generte_seg_correct_amount() {
        let projct = Project {
            duration: 3,
            id: 1,
            resource: vec![1],
            precedence: Vec::new(),
        };
        let segments = projct.generate_segments();
        assert_eq!(segments.len(), (3 * 4) / 2);
    }
}
