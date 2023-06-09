use crate::segment::Segment;
use crate::{id_generator::IdGenerator, sat_seg_var::Clause};
use core::fmt;
use std::{cell::RefCell, cmp::Ordering, fmt::Display, rc::Rc};
#[derive(Eq, Debug)]
pub struct Project<'a> {
    duration: usize,
    id: usize,
    resource: Vec<usize>,
    precedence: RefCell<Vec<&'a Project<'a>>>,
    segments: Vec<Rc<RefCell<Segment>>>,
}

impl<'a> Project<'a> {
    /// Creates a new [`Project`].
    pub fn new(
        duration: usize,
        id: usize,
        resource: Vec<usize>,
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

    pub fn add_presedence(&self, other: &'a Project<'a>) {
        self.precedence.borrow_mut().push(other);
    }
    /// Generates the segments of a project with the following details
    pub fn generate_segments(
        parent_id: usize,
        id_gen: &mut IdGenerator,
        parent_resource: &Vec<usize>,
        duration: usize,
    ) -> Vec<Rc<RefCell<Segment>>> {
        let mut segments: Vec<Rc<RefCell<Segment>>> = Vec::new();
        //TODO: Replace this ID generation with a more distinct one

        if duration == 0 {
            let segment = Segment::new(
                0,
                0,
                Vec::new(),
                id_gen.next_id(),
                parent_id,
                parent_resource.clone(),
            );
            segments.push(Rc::new(RefCell::new(segment)));
        } else {
            for x in 1..duration + 1 {
                for y in 1..duration - x + 2 {
                    let id = id_gen.next_id();
                    // find the precednece
                    // rule is: if x + y of old equal x of new, then new depends on old
                    let precedents = segments
                        .clone()
                        .into_iter()
                        .filter(|old| old.borrow().start_jiff + old.borrow().og_duration == x)
                        .collect();
                    let resource = parent_resource.clone();

                    let seg = Segment::new(x, y, precedents, id, parent_id, resource);
                    let cell = RefCell::new(seg);
                    segments.push(Rc::new(cell));
                }
            }
        }
        segments
    }
    pub fn get_last_segments(&self) -> Vec<Rc<RefCell<Segment>>> {
        self.segments
            .clone()
            .into_iter()
            .filter(|last| last.borrow().parent_project == self.id)
            .filter(|last| {
                last.borrow().start_jiff + last.borrow().og_duration == self.duration + 1
                    || self.duration == 0
            })
            .collect()
    }
    pub fn get_first_segments(&self) -> Vec<Rc<RefCell<Segment>>> {
        self.segments
            .clone()
            .into_iter()
            .filter(|first| first.borrow().parent_project == self.id)
            .filter(|first| first.borrow().start_jiff <= 1)
            .collect()
    }
    pub fn link_with_precedents(&self) {
        let our = self.get_first_segments();
        for precedent in self.precedence.borrow().clone() {
            let theirs = precedent.get_last_segments();
            {
                let last = &theirs;
                let first = &our;
                Segment::precedence_link(last, first);
            };
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn precedence(&self) -> &RefCell<Vec<&'a Project<'a>>> {
        &self.precedence
    }

    pub fn segments(&self) -> &[Rc<RefCell<Segment>>] {
        self.segments.as_ref()
    }
    pub fn generate_completion_clauses(&self) -> Vec<Clause> {
        let mut clauses: Vec<Clause> = Vec::new();
        for jiffy in 1..self.duration + 1 {
            //Each jiffy will produce one mega clause
            let mut jiffy_clause: Vec<i64> = Vec::new();
            for segment in self.get_segments_for_jiffy(jiffy) {
                // Get the vars representing each sement
                assert!(
                    !segment.borrow().variables.borrow().is_empty(),
                    "Generate segment variable has not been called on segment {:?}",
                    segment.borrow()
                );
                let var_ids: Vec<i64> = Rc::clone(&segment)
                    .borrow()
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
    pub fn generate_last_project_segment(&self) -> Vec<Clause> {
        // First rules out all projects, second rules out starting project
        if self.duration > 0 || self.precedence().borrow().len() < 1 {
            println!("Called Last project seg on wrong project");
        }
        let mut clauses: Vec<Clause> = Vec::new();
        for segment in self.segments() {
            for var in segment.borrow().variables.borrow().iter() {
                let clause = Clause::new(vec![var.id() as i64]);
                // This has to be soft clause, so there needs to be more
                clauses.push(clause);
            }
        }
        clauses
    }
    pub fn get_segments_for_jiffy(&self, jiffy: usize) -> Vec<Rc<RefCell<Segment>>> {
        self.segments
            .clone()
            .into_iter()
            .filter(|s| {
                s.borrow().start_jiff <= jiffy
                    && s.borrow().start_jiff + s.borrow().og_duration >= jiffy
            })
            .collect()
    }

    pub fn duration(&self) -> usize {
        self.duration
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
        let press: Vec<usize> = self
            .precedence
            .borrow()
            .clone()
            .into_iter()
            .map(|o| o.id())
            .collect();
        let seg: Vec<usize> = self
            .segments()
            .iter()
            .map(|seg| seg.borrow().id())
            .collect();
        write!(
            f,
            "Project with ID: {} \nDuration: {}\n Resource: {:?}  \nPrecedents: {:?}\n Segments: {:?}",
            self.id, self.duration, self.resource, press, seg
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
        let mut id_gen = IdGenerator::generator_for_segment();
        let project1 = Project::new(1, 1, Vec::new(), RefCell::new(Vec::new()), &mut id_gen);
        let project2 = Project::new(1, 2, Vec::new(), RefCell::new(Vec::new()), &mut id_gen);

        project2.add_presedence(&project1);
        project2.link_with_precedents();

        let project_2_first = project2.get_first_segments();
        let project_1_last = project1.get_last_segments();

        for segment_2_first in &project_2_first {
            for segment_1_last in &project_1_last {
                assert!(segment_2_first
                    .borrow()
                    .precedence
                    .clone()
                    .contains(segment_1_last));
            }
        }
        for segment in project2.segments() {
            println!("{}", segment.borrow());
        }
    }
}
