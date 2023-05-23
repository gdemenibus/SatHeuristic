use std::{
    borrow::Borrow,
    cell::RefCell,
    fmt::{self, Display},
    rc::Rc,
};

use crate::{
    id_generator::IdGenerator,
    sat_seg_var::{Clause, SATSVar},
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
    pub(crate) variables: RefCell<Vec<SATSVar>>,
    pub(crate) early_start: u64,
    pub(crate) latest_start: u64,
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
        let variables: RefCell<Vec<SATSVar>> = RefCell::new(Vec::new());
        let early_start = 0;
        let latest_start = 0;
        Self {
            start_jiff,
            duration,
            precedence,
            id,
            parent_project,
            resource,
            variables,
            early_start,
            latest_start,
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
    pub(crate) fn add_set_up_time(&mut self, set_up_cost: u32) {
        let press_parents: Vec<u64> = self
            .precedence
            .borrow()
            .clone()
            .into_iter()
            .map(|o| o.parent_project)
            .collect();

        if press_parents.contains(&self.parent_project) {
            self.duration += set_up_cost;
        }
    }

    pub(crate) fn precedence(&self) -> &RefCell<Vec<Rc<Segment>>> {
        &self.precedence
    }

    pub(crate) fn duration(&self) -> u32 {
        self.duration
    }
}
impl Segment {
    #[allow(non_snake_case)]
    pub(crate) fn generate_SAT_vars(
        &mut self,
        id_gen: &mut IdGenerator,
        early_start: u64,
        latest_start: u64,
    ) {
        let mut sat_vars: Vec<SATSVar> = Vec::new();
        for t in early_start..latest_start + 1 {
            let sat_var = SATSVar::new(self.id(), self.duration(), t, id_gen);
            sat_vars.push(sat_var);
        }
        self.variables = RefCell::new(sat_vars);
        self.latest_start = latest_start;
        self.early_start = early_start;
    }
    pub(crate) fn generate_precedence_clauses(&self) -> Vec<Clause> {
        let mut clauses: Vec<Clause> = Vec::new();
        self.variables.borrow().into_iter().for_each(|sat_var| {
            let mut sat_var_clause = vec![-(sat_var.id() as i64)];
            for pred in self.precedence.borrow().iter() {
                for pred_sat in pred
                    .variables
                    .borrow()
                    .iter()
                    .filter(|v| v.time() <= self.early_start - (pred.duration() as u64))
                {
                    sat_var_clause.push(pred_sat.id() as i64);
                }
            }
            let clause = Clause::new(sat_var_clause);
            clauses.push(clause);
        });
        clauses
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
