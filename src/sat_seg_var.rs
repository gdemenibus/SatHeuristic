use crate::{id_generator::IdGenerator, segment::Segment};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SATSVar<'a> {
    id: u64,
    segment: &'a Segment<'a>,
    time: u64,
    u_vars: Vec<SATUVar>,
    seg_times: &'a Times,
}

impl<'a> SATSVar<'a> {
    pub(crate) fn new(
        segment: &'a Segment<'a>,
        segment_duration: u32,
        time: u64,
        id_gen: &mut IdGenerator,
        seg_times: &'a Times,
    ) -> Self {
        let id = id_gen.next_id();
        let u_vars = SATSVar::generate_u_vars(segment.id(), segment_duration, time, id_gen);
        Self {
            id,
            segment,
            time,
            u_vars,
            seg_times,
        }
    }
    pub(crate) fn generate_u_vars(
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
    pub(crate) fn generate_consistency_clause(&self) -> Vec<Clause> {
        let mut clauses: Vec<Clause> = Vec::new();
        for u_var in &self.u_vars {
            let s = -(self.id as i64);
            let u = u_var.id as i64;
            let clause = Clause::new(vec![s, u]);
            clauses.push(clause);
        }
        clauses
    }

    pub(crate) fn id(&self) -> u64 {
        self.id
    }

    pub(crate) fn time(&self) -> u64 {
        self.time
    }

    pub(crate) fn seg_times(&self) -> &Times {
        self.seg_times
    }

    pub(crate) fn segment(&self) -> &'a Segment {
        self.segment
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Times {
    early_start: u64,
    latest_start: u64,
    segment_id: u64,
}

impl Times {
    pub(crate) fn new(early_start: u64, latest_start: u64, segment_id: u64) -> Self {
        Self {
            early_start,
            latest_start,
            segment_id,
        }
    }

    pub(crate) fn early_start(&self) -> u64 {
        self.early_start
    }

    pub(crate) fn latest_start(&self) -> u64 {
        self.latest_start
    }

    pub(crate) fn segment_id(&self) -> u64 {
        self.segment_id
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SATUVar {
    id: u64,
    segment_id: u64,
    jiffy: u32,
}

impl SATUVar {
    pub(crate) fn new(id: u64, segment_id: u64, jiffy: u32) -> Self {
        Self {
            id,
            segment_id,
            jiffy,
        }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) struct Clause {
    arguments: Vec<i64>,
}

impl Clause {
    pub(crate) fn new(arguments: Vec<i64>) -> Self {
        Self { arguments }
    }
    pub(crate) fn append_clauses(vec_1: &mut Vec<Clause>, vec_2: &mut Vec<Clause>) -> Vec<Clause> {
        vec_1.append(vec_2);
        vec_1.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::{
        id_generator::IdGenerator,
        sat_seg_var::{Clause, Times},
        segment::Segment,
    };

    use super::SATSVar;

    #[test]
    fn clause_generation() {
        //only one segment, with duration 1
        let mut id_gen = IdGenerator::generator_for_sat();
        let s = Segment::new(1, 1, RefCell::new(vec![]), 1, 1, vec![]);
        let seg_times = Times {
            early_start: 1,
            latest_start: 3,
            segment_id: 1,
        };
        let s_var = SATSVar::new(&s, 1, 1, &mut id_gen, &seg_times);
        let expected_clause = Clause::new(vec![-1, 2]);
        let got_clauses = s_var.generate_consistency_clause();
        println!("{:?}", got_clauses);
        assert_eq!(expected_clause, got_clauses[0]);
    }
    #[test]
    fn clause_generation_segment() {
        let mut id_gen = IdGenerator::generator_for_sat();
        let mut segment = Segment::new(1, 1, RefCell::new(Vec::new()), 1, 1, Vec::new());
        let early_start = 1;
        let latest_start = 3;
        let generated = segment.generate_SAT_vars(&mut id_gen, early_start, latest_start);
        let expected_amount = 3;
        assert_eq!(expected_amount, generated.len());
    }
}
