use crate::segment::Segment;
use crate::{id_generator::IdGenerator, sat_seg_var::Clause};
use core::fmt;
use std::borrow::Borrow;
use std::{cell::RefCell, cmp::Ordering, fmt::Display, rc::Rc};
#[derive(Eq, Debug)]
pub(crate) struct Project<'a> {
    duration: u32,
    id: u64,
    resource: Vec<u32>,
    precedence: RefCell<Vec<&'a Project<'a>>>,
    segments: Vec<Rc<Segment>>,
}

impl<'a> Project<'a> {
    /// Creates a new [`Project`].
    pub(crate) fn new(
        duration: u32,
        id: u64,
        resource: Vec<u32>,
        precedence: RefCell<Vec<&'a Project<'a>>>,
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

    pub(crate) fn add_presedence(&self, other: &'a Project<'a>) {
        self.precedence.borrow_mut().push(other);
    }
    /// Generates the segments of a project with the following details
    pub(crate) fn generate_segments(
        parent_id: u64,
        id_gen: &mut IdGenerator,
        parent_resource: &Vec<u32>,
        duration: u32,
    ) -> Vec<Rc<Segment>> {
        let mut segments: Vec<Rc<Segment>> = Vec::new();
        //TODO: Replace this ID generation with a more distinct one

        for x in 1..duration + 1 {
            for y in 1..duration - x + 2 {
                let id = id_gen.next_id();
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
    pub(crate) fn link_with_precedents(&self) {
        let our = self.get_first_segments();
        for precedent in &self.precedence.borrow().clone() {
            let theirs = precedent.get_last_segments();
            {
                let last = &theirs;
                let first = &our;
                for f in first {
                    f.add_precedents(last);
                }
            };
        }
    }

    pub(crate) fn id(&self) -> u64 {
        self.id
    }

    pub(crate) fn precedence(&self) -> &RefCell<Vec<&'a Project<'a>>> {
        &self.precedence
    }

    pub(crate) fn segments(&self) -> &[Rc<Segment>] {
        self.segments.as_ref()
    }
    pub(crate) fn generate_completion_clauses(&self) -> Vec<Clause> {
        let mut clauses: Vec<Clause> = Vec::new();
        for jiffy in 1..self.duration + 1 {
            //Each jiffy will produce one mega clause
            let mut jiffy_clause: Vec<i64> = Vec::new();
            for segment in self.get_segments_for_jiffy(jiffy) {
                // Get the vars representing each sement
                assert!(
                    !segment.variables.borrow().is_empty(),
                    "Generate segments has not been called on segment {:?}",
                    segment.id()
                );
                let var_ids: Vec<i64> = Rc::clone(&segment)
                    .variables
                    .borrow()
                    .iter()
                    .map(|x| x.id() as i64)
                    .collect();
                jiffy_clause.extend(var_ids);
            }
            let clause = Clause::new(jiffy_clause);
            clauses.push(clause);
        }
        clauses
    }
    pub(crate) fn get_segments_for_jiffy(&self, jiffy: u32) -> Vec<Rc<Segment>> {
        self.segments
            .iter()
            .map(Rc::clone)
            .filter(|s| s.start_jiff <= jiffy && s.start_jiff + s.duration >= jiffy)
            .collect()
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
impl Default for Project<'_> {
    fn default() -> Self {
        let mut id_gen = IdGenerator(10000);
        let resources = vec![1];
        Project::new(1, 1, resources, RefCell::new(Vec::new()), &mut id_gen)
    }
}
impl Display for Project<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let press: Vec<u64> = self
            .precedence
            .borrow()
            .clone()
            .into_iter()
            .map(|o| o.id())
            .collect();
        write!(
            f,
            "Project with ID: {} \nDuration: {}\n Resource: {:?}  \nPrecedents: {:?}\n",
            self.id, self.duration, self.resource, press
        )
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::{id_generator::IdGenerator, project::Project};

    #[test]
    fn generte_seg_correct_amount() {
        let mut id_gen = IdGenerator(0);
        let projct = Project::new(3, 1, vec![1], RefCell::new(Vec::new()), &mut id_gen);
        assert_eq!(projct.segments.len(), (3 * 4) / 2);
    }
    #[test]
    fn generate_first_seg_amount() {
        let mut id_gen = IdGenerator(0);
        let projct = Project::new(3, 1, vec![1], RefCell::new(Vec::new()), &mut id_gen);
        let first_segements = projct.get_first_segments();
        assert_eq!(first_segements.len(), 3);
    }
    #[test]
    fn generate_last_seg_amount() {
        let mut id_gen = IdGenerator(0);
        let projct = Project::new(3, 1, vec![1], RefCell::new(Vec::new()), &mut id_gen);
        let last_segments = projct.get_last_segments();
        assert_eq!(last_segments.len(), 3);
    }
    #[test]
    fn link_correctly() {
        let project1 = Project::default();
        let project2 = Project::default();
        project2.add_presedence(&project1);
        project2.link_with_precedents();
        let project_2_first = project2.get_first_segments();
        let project_1_last = project1.get_last_segments();

        for segment_2_first in &project_2_first {
            for segment_1_last in &project_1_last {
                assert!(segment_2_first
                    .precedence
                    .clone()
                    .into_inner()
                    .contains(segment_1_last));
            }
        }
    }
}
