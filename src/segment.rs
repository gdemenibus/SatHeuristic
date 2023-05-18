use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Segment {
    pub(crate) start_jiff: u32,
    pub(crate) duration: u32,
    pub(crate) precedence: RefCell<Vec<Rc<Segment>>>,
    pub(crate) id: u64,
    pub(crate) parent_project: u64,
}
impl Segment {
    pub(crate) fn new(
        start_jiff: u32,
        duration: u32,
        precedence: RefCell<Vec<Rc<Segment>>>,
        id: u64,
        parent_project: u64,
    ) -> Self {
        Self {
            start_jiff,
            duration,
            precedence,
            id,
            parent_project,
        }
    }

    /// .
    pub(crate) fn add_precedence(&self, pres: Rc<Segment>) {
        self.precedence.borrow_mut().push(pres);
    }
}
