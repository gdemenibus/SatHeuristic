use std::iter::zip;

use crate::id_generator::IdGenerator;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SATSVar {
    id: u64,
    segment_id: u64,
    time: u64,
    u_vars: Vec<SATUVar>,
}

impl SATSVar {
    pub fn new(
        segment_id: u64,
        segment_duration: u32,
        time: u64,
        id_gen: &mut IdGenerator,
    ) -> Self {
        let id = id_gen.next_id();
        let u_vars = SATSVar::generate_u_vars(segment_id, segment_duration, time, id_gen);
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
    ) -> Vec<SATUVar> {
        let mut u_vars: Vec<SATUVar> = Vec::new();
        for l in (time as u32)..(time as u32) + segment_duration {
            let u_var = SATUVar::new(id_gen.next_id(), segment_id, l);
            u_vars.push(u_var);
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

    pub fn u_vars(&self) -> &[SATUVar] {
        self.u_vars.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SATUVar {
    id: u64,
    segment_id: u64,
    time_at: u32,
}

impl SATUVar {
    pub fn new(id: u64, segment_id: u64, time_at: u32) -> Self {
        Self {
            id,
            segment_id,
            time_at,
        }
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
    pub fn wite_to_string_hard(&self) -> String {
        self.arguments
            .iter()
            .map(|x| x.to_string() + " ")
            .collect::<String>()
            + " 0"
    }
    pub fn write_to_string_soft(&self, weight: usize) -> String {
        self.arguments
            .iter()
            .map(|x| x.to_string() + " ")
            .collect::<String>()
            + &weight.to_string()
    }
    pub fn write_list_to_string_hard(clauses: Vec<Clause>) -> String {
        clauses
            .iter()
            .map(|x| x.wite_to_string_hard() + "\n")
            .collect::<String>()
    }
    pub fn write_list_to_string_soft(clauses: Vec<Clause>, weights: Vec<usize>) -> String {
        zip(clauses, weights).rfold("".to_owned(), |acc, (x, y)| {
            let join = format!("{}{}", acc, x.write_to_string_soft(y));
            let add_last = format!("{}{}", join, "\n".to_owned());
            return add_last;
        })
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
        let s_var = SATSVar::new(1, 1, 1, &mut id_gen);
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
