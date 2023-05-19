use crate::id_generator::IdGenerator;
use crate::{id_generator, segment::Segment};
use std::{cell::RefCell, cmp::Ordering, rc::Rc};
#[derive(Eq)]
pub(crate) struct Project<'a> {
    duration: u32,
    id: u64,
    resource: Vec<u64>,
    precedence: Vec<&'a Project<'a>>,
    segments: Vec<Rc<Segment>>,
}

impl<'a> Project<'a> {
    pub(crate) fn new(
        duration: u32,
        id: u64,
        resource: Vec<u64>,
        precedence: Vec<&'a Project<'a>>,
        id_gen: &mut IdGenerator,
    ) -> Self {
        let segments = Project::generate_segments(id, id_gen, &resource, duration);
        Self {
            duration,
            id,
            resource,
            precedence,
            segments,
        }
    }

    pub(crate) fn add_presedence(&mut self, other: &'a Project<'a>) {
        self.precedence.push(other);
    }
    /// .
    pub(crate) fn generate_segments(
        parent_id: u64,
        id_gen: &mut IdGenerator,
        parent_resource: &Vec<u64>,
        duration: u32,
    ) -> Vec<Rc<Segment>> {
        let mut segments: Vec<Rc<Segment>> = Vec::new();
        //TODO: Replace this ID generation with a more distinct one
        let id = id_gen.next_id();
        for x in 1..duration + 1 {
            for y in 1..duration - x + 2 {
                // find the precednece
                // rule is: if x + y of old equal x of new, then new depends on old
                let precedents = segments
                    .iter()
                    .map(Rc::clone)
                    .filter(|old| old.start_jiff + old.duration == x)
                    .collect();
                // TODO: This might change. Cloning might be too expensive
                let resource = parent_resource.clone();
                let seg = Segment::new(x, y, RefCell::new(precedents), id, parent_id, resource);
                segments.push(Rc::new(seg));
            }
        }
        segments
    }
    pub(crate) fn get_last_segments(&self) -> Vec<Rc<Segment>> {
        let last_segments = self
            .segments
            .iter()
            .map(Rc::clone)
            .filter(|last| last.parent_project == self.id)
            .filter(|last| last.start_jiff + last.duration == self.duration + 1)
            .collect();
        last_segments
    }
    pub(crate) fn get_first_segments(&self) -> Vec<Rc<Segment>> {
        let first_segements = self
            .segments
            .iter()
            .map(Rc::clone)
            .filter(|first| first.parent_project == self.id)
            .filter(|first| first.start_jiff == 1)
            .collect();
        first_segements
    }
}
impl PartialEq for Project<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl PartialOrd for Project<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}
impl Ord for Project<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{id_generator::IdGenerator, project::Project};

    #[test]
    fn generte_seg_correct_amount() {
        let mut id_gen = IdGenerator::default();
        let projct = Project::new(1, 1, vec![1], Vec::new(), &mut id_gen);
        assert_eq!(projct.segments.len(), (3 * 4) / 2);
    }
    #[test]
    fn generate_first_seg_amount() {
        let mut id_gen = IdGenerator::default();
        let projct = Project::new(1, 1, vec![1], Vec::new(), &mut id_gen);
        let first_segements = projct.get_first_segments();
        assert_eq!(first_segements.len(), 3);
    }
    #[test]
    fn generate_last_seg_amount() {
        let mut id_gen = IdGenerator::default();
        let projct = Project::new(1, 1, vec![1], Vec::new(), &mut id_gen);
        let last_segments = projct.get_last_segments();
        assert_eq!(last_segments.len(), 3);
    }
}
