use std::{
    cell::RefCell,
    fmt::{self, Display},
    rc::Rc,
};

use crate::{
    id_generator::IdGenerator,
    sat_seg_var::{Clause, SATSVar, Times},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Segment<'a> {
    pub(crate) start_jiff: u32,
    pub(crate) duration: u32,
    //TODO: Replace with HashSet to prevent duplicates
    pub(crate) precedence: RefCell<Vec<Rc<Segment<'a>>>>,
    pub(crate) id: u64,
    pub(crate) parent_project: u64,
    // TODO: Perhaps there is a better way to deal with this resource array
    // Investigate
    pub(crate) resource: Vec<u32>,
}
impl<'a> Segment<'a> {
    pub(crate) fn new(
        start_jiff: u32,
        duration: u32,
        precedence: RefCell<Vec<Rc<Segment<'a>>>>,
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
    pub(crate) fn add_precedent(&self, precedent: &Rc<Segment<'a>>) {
        self.precedence.borrow_mut().push(Rc::clone(precedent));
    }
    pub(crate) fn add_precedents(&self, precedents: &Vec<Rc<Segment<'a>>>) {
        for precedent in precedents {
            self.add_precedent(precedent);
        }
    }
    fn precedence_link(last: &Vec<Rc<Segment<'a>>>, first: Vec<Rc<Segment<'a>>>) {
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

    pub(crate) fn precedence(&self) -> &RefCell<Vec<Rc<Segment<'a>>>> {
        &self.precedence
    }

    pub(crate) fn duration(&self) -> u32 {
        self.duration
    }
}
impl<'a> Segment<'a> {
    /// Generate all SAT variables based on this segment
    #[allow(non_snake_case)]
    pub(crate) fn generate_SAT_vars(
        &'a self,
        id_gen: &mut IdGenerator,
        seg_times: &'a Times,
    ) -> Vec<Rc<SATSVar>> {
        let mut sat_vars: Vec<Rc<SATSVar>> = Vec::new();
        for t in seg_times.early_start()..seg_times.latest_start() + 1 {
            let sat_var = SATSVar::new(&self, self.duration(), t, id_gen, seg_times);
            sat_vars.push(Rc::new(sat_var));
        }
        sat_vars
    }
    /// Generates precedence clause
    /// pred_variables must be from precedents of this segment
    pub(crate) fn generate_precedence_clauses(
        &self,
        our_variables: &'a Vec<Rc<SATSVar<'a>>>,
        pred_variables: &'a Vec<Rc<SATSVar<'a>>>,
    ) -> Vec<Clause> {
        let mut clauses: Vec<Clause> = Vec::new();
        for sat_var in our_variables.into_iter() {
            let mut sat_var_clause = vec![-(sat_var.id() as i64)];
            for pred_sat in pred_variables.iter().filter(|v| {
                v.time()
                    <= sat_var.clone().seg_times().early_start() - (v.segment().duration() as u64)
            }) {
                sat_var_clause.push(pred_sat.id() as i64);
            }
            let clause = Clause::new(sat_var_clause);
            clauses.push(clause);
        }
        clauses
    }
}
impl Display for Segment<'_> {
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
