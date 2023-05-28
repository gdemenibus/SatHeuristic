use std::{
    cell::RefCell,
    fmt::{self, Display},
    rc::Rc,
};

use crate::{
    id_generator::IdGenerator,
    sat_seg_var::{Clause, SATSVar},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Segment {
    pub start_jiff: u32,
    pub duration: u32,
    //TODO: Replace with HashSet to prevent duplicates
    pub precedence: Vec<Rc<RefCell<Segment>>>,
    pub id: u64,
    pub parent_project: u64,
    // TODO: Perhaps there is a better way to deal with this resource array
    // Investigate
    pub resource: Vec<u32>,
    pub variables: RefCell<Vec<SATSVar>>,
    pub early_start: u64,
    pub latest_start: u64,
}
impl Segment {
    pub fn new(
        start_jiff: u32,
        duration: u32,
        precedence: Vec<Rc<RefCell<Segment>>>,
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
    pub fn add_precedent(&mut self, precedent: &Rc<RefCell<Segment>>) {
        assert_ne!(self.parent_project, precedent.borrow().parent_project);
        self.precedence.push(precedent.clone());
    }
    pub fn add_precedents(&mut self, precedents: &Vec<Rc<RefCell<Segment>>>) {
        for precedent in precedents {
            self.add_precedent(precedent);
        }
    }
    pub fn precedence_link(last: &Vec<Rc<RefCell<Segment>>>, first: &Vec<Rc<RefCell<Segment>>>) {
        for f in first {
            f.borrow_mut().add_precedents(last);
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn add_set_up_time(&mut self, set_up_cost: u32) {
        let press_parents: Vec<u64> = self
            .precedence
            .iter()
            .map(|o| o.borrow().parent_project)
            .collect();

        if press_parents.contains(&self.parent_project) {
            self.duration += set_up_cost;
        }
    }

    pub fn precedence(&self) -> &Vec<Rc<RefCell<Segment>>> {
        &self.precedence
    }

    pub fn duration(&self) -> u32 {
        self.duration
    }
}
impl Segment {
    #[allow(non_snake_case)]
    pub fn generate_SAT_vars(
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
    pub fn generate_precedence_clauses(&self) -> Vec<Clause> {
        let mut clauses: Vec<Clause> = Vec::new();
        self.variables.borrow().iter().for_each(|sat_var| {
            if !self.precedence().is_empty() {
                let mut sat_var_clause = vec![-(sat_var.id() as i64)];
                for pred in self.precedence.iter() {
                    for pred_sat in pred.borrow().variables.borrow().iter().filter(|v| {
                        v.time() <= self.early_start - (pred.borrow().duration() as u64)
                    }) {
                        sat_var_clause.push(pred_sat.id() as i64);
                    }
                }
                let clause = Clause::new(sat_var_clause);
                clauses.push(clause);
            }
        });
        clauses
    }
}
impl Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let press: Vec<u64> = self
            .precedence
            .clone()
            .into_iter()
            .map(|o| o.borrow().id())
            .collect();
        write!(
            f,
            "Segment with ID: {} \nParent ID: {}\nDuration: {}\n Resource: {:?}  \nPrecedents: {:?}\n",
            self.id, self.parent_project, self.duration, self.resource, press
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::segment::Segment;
    #[test]
    fn link_test() {}
}
