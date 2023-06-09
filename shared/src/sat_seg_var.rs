use std::{iter::zip, rc::Rc};

use itertools::Itertools;

use crate::id_generator::IdGenerator;

use pyo3::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SATSVar {
    id: usize,
    segment_id: usize,
    segment_duration: usize,
    time: usize,
    weight: bool,
}

impl SATSVar {
    pub fn new(
        segment_id: usize,
        segment_duration: usize,
        time: usize,
        id_gen: &mut IdGenerator,
        resource_usage: Vec<usize>,
        weight: bool,
    ) -> Self {
        let id = id_gen.next_id();
        Self {
            id,
            segment_id,
            segment_duration,
            time,
            weight,
        }
    }
    pub fn generate_u_vars(
        segment_id: usize,
        segment_duration: usize,
        time: usize,
        id_gen: &mut IdGenerator,
        resource_usage: Vec<usize>,
    ) -> Vec<Rc<SATUVar>> {
        let mut u_vars: Vec<Rc<SATUVar>> = Vec::new();
        if segment_duration == 0 {
            let resource = resource_usage;
            let u_var = SATUVar::new(id_gen.next_id(), segment_id, time as usize, resource);
            u_vars.push(Rc::new(u_var));
        } else {
            for l in (time as usize)..(time as usize) + segment_duration {
                let resource = resource_usage.clone();
                let u_var = SATUVar::new(id_gen.next_id(), segment_id, l, resource);
                u_vars.push(Rc::new(u_var));
            }
        }
        u_vars
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn time(&self) -> usize {
        self.time
    }

    pub fn weight(&self) -> bool {
        self.weight
    }

    pub fn segment_duration(&self) -> usize {
        self.segment_duration
    }

    pub fn id_mut(&mut self) -> &mut usize {
        &mut self.id
    }
    pub fn last_to_clause(vars: &Vec<Rc<SATSVar>>) -> Vec<Clause> {
        vars.iter().map(|u| u.to_clause()).collect()
    }
    pub fn to_clause(&self) -> Clause {
        Clause::new(vec![self.id as i64])
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SATUVar {
    id: usize,
    segment_id: usize,
    time_at: usize,
    resource_usage: Vec<usize>,
}

impl SATUVar {
    pub fn new(id: usize, segment_id: usize, time_at: usize, resource_usage: Vec<usize>) -> Self {
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

    pub fn time_at(&self) -> usize {
        self.time_at
    }
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn resource_usage(&self) -> &[usize] {
        self.resource_usage.as_ref()
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
        let string = clauses
            .iter()
            .map(|x| x.wite_to_string_hard(top) + "\n")
            .collect::<String>();
        drop(clauses);
        string
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
    pub fn python_max(
        lit: Vec<usize>,
        weight: Vec<usize>,
        total: usize,
        highest_id_yet: usize,
    ) -> PyResult<Vec<Vec<i64>>> {
        Python::with_gil(|py| {
            let fun: Py<PyAny> = PyModule::from_code(
                py,
                "
def function(lits, weights, bound, top_id):
    import pysat.pb
    cnf = pysat.pb.PBEnc.atmost(lits=lits, weights=weights, bound=bound, top_id=top_id)
    return cnf.clauses",
                "",
                "",
            )?
            .getattr("function")?
            .into();
            let result: Vec<Vec<i64>> = fun
                .call1(py, (lit, weight, total, highest_id_yet))?
                .extract(py)?;
            Ok(result)
        })
    }
    pub fn integer_vec_to_clause(list: Vec<Vec<i64>>) -> Vec<Clause> {
        let mut output: Vec<Clause> = Vec::new();
        for l in list.into_iter() {
            let clause = Clause::new(l);
            output.push(clause);
        }
        output
    }
    pub fn u_vec_accum(
        u_vars: Vec<Rc<SATUVar>>,
        max_current: &mut IdGenerator,
        resources: Vec<usize>,
    ) -> Vec<Clause> {
        // split into times for u vars
        let mut output: Vec<Clause> = Vec::new();
        let split_by_time = SATUVar::linear_sum_split(&u_vars);
        for u_time in split_by_time.iter() {
            for (index, resource) in resources.iter().enumerate() {
                let u_ids: Vec<usize> = u_time.iter().map(|c| c.id()).collect();
                let u_weights: Vec<usize> =
                    u_time.iter().map(|u| u.resource_usage[index]).collect();
                let lits = Clause::python_max(
                    u_ids,
                    u_weights,
                    *resource,
                    max_current.current_asignment(),
                )
                .unwrap();
                if !lits.is_empty() {
                    let max_next = lits
                        .iter()
                        .flatten()
                        .map(|a| a.unsigned_abs())
                        .max()
                        .unwrap();
                    max_current.new_current(max_next as usize);
                }
                let mut gen_clauses = Clause::integer_vec_to_clause(lits);
                output.append(&mut gen_clauses);
            }
        }
        output

        // then generate for each resource
        // add them to the return
        //
        // return it all
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
        let s_var = SATSVar::new(1, 1, 1, &mut id_gen, resource, false);
        let expected_clause = Clause::new(vec![-1, 2]);
    }
    #[test]
    fn clause_generation_segment() {
        let mut id_gen = IdGenerator::generator_for_sat();
        let early_start = 1;
        let latest_start = 3;
    }
    #[test]
    fn python_in_rust() {
        println!(
            "{:?}",
            Clause::python_max(vec![1, 2, 3], vec![1, 2, 3], 3, 0)
        );
    }
}
