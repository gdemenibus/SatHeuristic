use std::{iter::zip, rc::Rc};

use itertools::Itertools;

use crate::id_generator::IdGenerator;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SATSVar {
    id: u64,
    segment_id: u64,
    time: u64,
    u_vars: Vec<Rc<SATUVar>>,
}

impl SATSVar {
    pub fn new(
        segment_id: u64,
        segment_duration: u32,
        time: u64,
        id_gen: &mut IdGenerator,
        resource_usage: Vec<u32>,
    ) -> Self {
        let id = id_gen.next_id();
        let u_vars =
            SATSVar::generate_u_vars(segment_id, segment_duration, time, id_gen, resource_usage);
        Self {
            id,
            segment_id,
            time,
            u_vars,
        }
    }
    pub fn generate_u_vars(
        segment_id: u64,
        segment_duration: u32,
        time: u64,
        id_gen: &mut IdGenerator,
        resource_usage: Vec<u32>,
    ) -> Vec<Rc<SATUVar>> {
        let mut u_vars: Vec<Rc<SATUVar>> = Vec::new();
        for l in (time as u32)..(time as u32) + segment_duration {
            let resource = resource_usage.clone();
            let u_var = SATUVar::new(id_gen.next_id(), segment_id, l, resource);
            u_vars.push(Rc::new(u_var));
        }
        u_vars
    }
    pub fn generate_consistency_clause(&self) -> Vec<Clause> {
        let mut clauses: Vec<Clause> = Vec::new();
        for u_var in &self.u_vars {
            let s = -(self.id as i64);
            let u = u_var.id as i64;
            let clause = Clause::new(vec![s, u]);
            clauses.push(clause);
        }
        clauses
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn time(&self) -> u64 {
        self.time
    }

    pub fn u_vars(&self) -> &[Rc<SATUVar>] {
        self.u_vars.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SATUVar {
    id: u64,
    segment_id: u64,
    time_at: u32,
    resource_usage: Vec<u32>,
}

impl SATUVar {
    pub fn new(id: u64, segment_id: u64, time_at: u32, resource_usage: Vec<u32>) -> Self {
        Self {
            id,
            segment_id,
            time_at,
            resource_usage,
        }
    }
    pub fn linear_sum_split(vars: &Vec<Rc<SATUVar>>) -> Vec<Vec<&Rc<SATUVar>>> {
        vars.iter()
            .into_group_map_by(|a| a.time_at())
            .into_values()
            .collect()
    }

    pub fn time_at(&self) -> u32 {
        self.time_at
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Clause {
    arguments: Vec<i64>,
}

impl Clause {
    pub fn new(arguments: Vec<i64>) -> Self {
        Self { arguments }
    }
    pub fn append_clauses(vec_1: &mut Vec<Clause>, vec_2: &mut Vec<Clause>) -> Vec<Clause> {
        vec_1.append(vec_2);
        vec_1.to_vec()
    }
    pub fn wite_to_string_hard(&self, top: usize) -> String {
        top.to_string()
            + " "
            + &self
                .arguments
                .iter()
                .map(|x| x.to_string() + " ")
                .collect::<String>()
    }
    pub fn write_to_string_soft(&self, weight: usize) -> String {
        weight.to_string()
            + " "
            + &self
                .arguments
                .iter()
                .map(|x| x.to_string() + " ")
                .collect::<String>()
    }
    pub fn write_list_to_string_hard(clauses: Vec<Clause>, top: usize) -> String {
        clauses
            .iter()
            .map(|x| x.wite_to_string_hard(top) + "\n")
            .collect::<String>()
    }
    pub fn write_list_to_string_soft(clauses: Vec<Clause>, weights: Vec<usize>) -> String {
        zip(clauses, weights).rfold("".to_owned(), |acc, (x, y)| {
            let join = format!("{}{}", acc, x.write_to_string_soft(y));
            let add_last = format!("{}{}", join, "\n".to_owned());
            return add_last;
        })
    }
    pub fn write_first_line(clauses_amount: usize, number_var: usize, top: usize) -> String {
        "p wcnf ".to_owned()
            + &clauses_amount.to_string()
            + " "
            + &number_var.to_string()
            + " "
            + &top.to_string()
            + "\n"
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::{id_generator::IdGenerator, sat_seg_var::Clause, segment::Segment};

    use super::SATSVar;

    #[test]
    fn clause_generation() {
        //only one segment, with duration 1
        let mut id_gen = IdGenerator::generator_for_sat();
        let resource = vec![1];
        let s_var = SATSVar::new(1, 1, 1, &mut id_gen, resource);
        let expected_clause = Clause::new(vec![-1, 2]);
        let got_clauses = s_var.generate_consistency_clause();
        println!("{:?}", got_clauses);
        assert_eq!(expected_clause, got_clauses[0]);
    }
    #[test]
    fn clause_generation_segment() {
        let mut id_gen = IdGenerator::generator_for_sat();
        let mut segment = Segment::new(1, 1, Vec::new(), 1, 1, Vec::new());
        let early_start = 1;
        let latest_start = 3;
        segment.generate_SAT_vars(&mut id_gen, early_start, latest_start);
        let generated = segment.variables;
        let expected_amount = 3;
        assert_eq!(expected_amount, generated.borrow().len());
    }
}
