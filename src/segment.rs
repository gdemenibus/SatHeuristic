use std::{
    cell::RefCell,
    fmt::{self, Display},
    rc::Rc,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Segment {
    pub(crate) start_jiff: u32,
    pub(crate) duration: u32,
    //TODO: Replace with HashSet to prevent duplicates
    pub(crate) precedence: RefCell<Vec<Rc<Segment>>>,
    pub(crate) id: u64,
    pub(crate) parent_project: u64,
    // TODO: Perhaps there is a better way to deal with this resource array
    // Investigate
    pub(crate) resource: Vec<u32>,
}
impl Segment {
    pub(crate) fn new(
        start_jiff: u32,
        duration: u32,
        precedence: RefCell<Vec<Rc<Segment>>>,
        id: u64,
        parent_project: u64,
        resource: Vec<u32>,
    ) -> Self {
        Self {
            start_jiff,
            duration,
            precedence,
            id,
            parent_project,
            resource,
        }
    }

    /// .
    pub(crate) fn add_precedent(&self, precedent: &Rc<Segment>) {
        self.precedence.borrow_mut().push(Rc::clone(precedent));
    }
    pub(crate) fn add_precedents(&self, precedents: &Vec<Rc<Segment>>) {
        for precedent in precedents {
            self.add_precedent(precedent);
        }
    }
    fn precedence_link(last: &Vec<Rc<Segment>>, first: Vec<Rc<Segment>>) {
        for f in first {
            f.add_precedents(last);
        }
    }

    pub(crate) fn id(&self) -> u64 {
        self.id
    }
}
impl Display for Segment {
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
            "Segment with ID: {} \nDuration: {}\n Resource: {:?}  \nPrecedents: {:?}\n",
            self.id, self.duration, self.resource, press
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::segment::Segment;
    #[test]
    fn link_test() {}
}
