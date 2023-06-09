use std::{
    cell::RefCell,
    fmt::{self, Display},
    mem,
    rc::Rc,
};

use crate::{
    id_generator::IdGenerator,
    sat_seg_var::{Clause, SATSVar, SATUVar},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Segment {
    pub start_jiff: usize,
    pub duration: usize,
    //TODO: Replace with HashSet to prevent duplicates
    pub precedence: Vec<Rc<RefCell<Segment>>>,
    pub id: usize,
    pub parent_project: usize,
    // TODO: Perhaps there is a better way to deal with this resource array
    // Investigate
    pub resource: Vec<usize>,
    pub variables: RefCell<Vec<Rc<SATSVar>>>,
    pub uvariables: RefCell<Vec<Rc<SATUVar>>>,
    pub early_start: usize,
    pub latest_start: usize,
    pub og_duration: usize,
    pub has_set_up: bool,
}
impl Segment {
    pub fn new(
        start_jiff: usize,
        duration: usize,
        precedence: Vec<Rc<RefCell<Segment>>>,
        id: usize,
        parent_project: usize,
        resource: Vec<usize>,
    ) -> Self {
        let variables: RefCell<Vec<Rc<SATSVar>>> = RefCell::new(Vec::new());
        let uvariables: RefCell<Vec<Rc<SATUVar>>> = RefCell::new(Vec::new());
        let early_start = 0;
        let latest_start = 0;
        let og_duration = duration;
        let duration = duration;
        let mut has_set_up = false;
        if start_jiff == 1 && duration > 0 {
            has_set_up = true;
        }

        Self {
            start_jiff,
            duration,
            precedence,
            id,
            parent_project,
            resource,
            variables,
            uvariables,
            early_start,
            latest_start,
            og_duration,
            has_set_up,
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

    pub fn id(&self) -> usize {
        self.id
    }
    pub fn add_set_up_time(&mut self, set_up_cost: usize) {
        let press_parents: Vec<usize> = self
            .precedence
            .iter()
            .map(|o| o.borrow().parent_project)
            .collect();

        if press_parents.contains(&self.parent_project) && self.duration() > 0 {
            self.duration += set_up_cost;
        }
    }

    pub fn precedence(&self) -> &Vec<Rc<RefCell<Segment>>> {
        &self.precedence
    }

    pub fn duration(&self) -> usize {
        self.duration
    }
}
impl Segment {
    #[allow(non_snake_case)]
    pub fn generate_SAT_vars(
        &mut self,
        id_gen: &mut IdGenerator,
        early_start: usize,
        latest_start: usize,
    ) {
        let mut sat_vars: Vec<Rc<SATSVar>> = Vec::new();
        let mut sat_u_vars: Vec<Rc<SATUVar>> = Vec::new();
        for t in early_start..latest_start + 1 {
            let resource = self.resource.clone();
            let sat_var = SATSVar::new(
                self.id(),
                self.duration(),
                t,
                id_gen,
                resource,
                self.has_set_up,
            );
            sat_vars.push(Rc::new(sat_var));
        }
        for t in early_start..latest_start + 1 + self.duration() {
            let resource = self.resource.clone();
            let u_var = SATUVar::new(id_gen.next_id(), self.id(), t, resource);
            sat_u_vars.push(Rc::new(u_var));
        }
        let mut new_u_var = RefCell::new(sat_u_vars);
        let mut new_s_var = RefCell::new(sat_vars);

        mem::swap(&mut self.uvariables, &mut new_u_var);
        mem::swap(&mut self.variables, &mut new_s_var);

        self.latest_start = latest_start;
        self.early_start = early_start;
        drop(new_s_var);
        drop(new_u_var);
    }
    pub fn generate_precedence_clauses(&self) -> Vec<Clause> {
        let mut clauses: Vec<Clause> = Vec::new();
        self.variables.borrow().iter().for_each(|sat_var| {
            if !self.precedence().is_empty() {
                let mut sat_var_clause = vec![-(sat_var.id() as i64)];
                for pred in self.precedence.iter() {
                    for pred_sat in pred.borrow().variables.borrow().iter().filter(|v| {
                        v.time() <= self.early_start - (pred.borrow().duration() as usize)
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

    pub fn generate_consistency_clause(&self) -> Vec<Clause> {
        let mut clauses: Vec<Clause> = Vec::new();
        for s_var in self.variables.borrow().iter() {
            for u_var in self.uvariables.borrow().iter() {
                if u_var.time_at() >= s_var.time()
                    && u_var.time_at() <= s_var.time() + self.duration()
                {
                    let s = -(s_var.id() as i64);
                    let u = u_var.id() as i64;
                    let clause = Clause::new(vec![s, u]);
                    clauses.push(clause);
                }
            }
        }
        clauses
    }
}
impl Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let press: Vec<usize> = self
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
