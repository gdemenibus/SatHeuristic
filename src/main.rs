use std::{cell::RefCell, rc::Rc};

use bumpalo::Bump;

use crate::{
    id_generator::IdGenerator,
    sat_seg_var::{Clause, SATUVar},
    segment::Segment,
};

pub mod project;

pub mod floyd_warshall;
mod id_generator;
mod readerSM;
pub mod sat_seg_var;
mod segment;
fn main() {
    let arena = Bump::new();
    let projects = readerSM::read_input("data/datasets/j30.sm/j3010_0.sm", &arena).unwrap();

    let mut segments: Vec<Rc<RefCell<Segment>>> = Vec::new();
    for project in projects.iter() {
        for segment in project.segments() {
            segments.push(segment.clone());
        }
    }
    // sort by id
    segments.sort_by_key(|k| k.borrow().id());
    let distances = floyd_warshall::segments_dist_shortest_vec(&segments);
    let critical_path = floyd_warshall::segments_dist_longest_vec(&segments)
        .iter()
        .flatten()
        .min()
        .unwrap()
        .unsigned_abs() as u64;
    let mut id_gen = IdGenerator::generator_for_segment();

    let mut clauses: Vec<Clause> = Vec::new();
    let mut counter = 0;
    for segment in segments.iter() {
        let early_start = distances[segment.borrow().id() as usize][0].unsigned_abs() as u64;

        segment
            .borrow_mut()
            .generate_SAT_vars(&mut id_gen, early_start, critical_path);
        counter += segment.borrow().variables.borrow().len();
        counter += segment
            .borrow()
            .variables
            .borrow()
            .iter()
            .flat_map(|v| v.u_vars())
            .collect::<Vec<&SATUVar>>()
            .len();
        for clause in segment.borrow().generate_precedence_clauses() {
            clauses.push(clause);
        }
    }
    for project in projects.iter() {
        for clause in project.generate_completion_clauses() {
            clauses.push(clause);
        }
    }

    for segment in segments.iter() {
        for variable in segment.borrow().variables.borrow().iter() {
            for clause in variable.generate_consistency_clause() {
                clauses.push(clause);
            }
        }
    }

    println!("Total Number of Vars: {:?}", counter);
    println!("Total Number of clauses: {:?}", clauses.len());
}
