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
                // TODO: This might change. Cloning might be too expensive
                let resource = self.resource.clone();
                let seg = Segment::new(x, y, RefCell::new(precedents), id, self.id, resource);
                segments.push(Rc::new(seg));
            }
        }
        segments
    }
    pub(crate) fn get_last_segments(&self, segments: &Vec<Rc<Segment>>) -> Vec<Rc<Segment>> {
        let last_segments = segments
            .iter()
            .map(Rc::clone)
            .filter(|last| last.parent_project == self.id)
            .filter(|last| last.start_jiff + last.duration == self.duration + 1)
            .collect();
        last_segments
    }
    pub(crate) fn get_first_segments(&self, segments: &Vec<Rc<Segment>>) -> Vec<Rc<Segment>> {
        let first_segements = segments
            .iter()
            .map(Rc::clone)
            .filter(|first| first.parent_project == self.id)
            .filter(|first| first.start_jiff == 1)
            .collect();
        first_segements
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
    #[test]
    fn generate_first_seg_amount() {
        let projct = Project {
            duration: 3,
            id: 1,
            resource: vec![1],
            precedence: Vec::new(),
        };
        let segments = projct.generate_segments();
        let first_segements = projct.get_first_segments(&segments);
        assert_eq!(first_segements.len(), 3);
    }
    #[test]
    fn generate_last_seg_amount() {
        let projct = Project {
            duration: 3,
            id: 1,
            resource: vec![1],
            precedence: Vec::new(),
        };
        let segments = projct.generate_segments();
        let last_segments = projct.get_last_segments(&segments);
        assert_eq!(last_segments.len(), 3);
    }
}
