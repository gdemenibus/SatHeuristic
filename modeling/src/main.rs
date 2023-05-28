pub mod readerSM;
pub mod schedule;

use bumpalo::Bump;
//use clap::Parser;
use std::{cell::RefCell, env, fs, path::Path, rc::Rc};

use shared::{
    floyd_warshall,
    id_generator::IdGenerator,
    sat_seg_var::{Clause, SATUVar},
    segment::Segment,
};

fn main() {
    //let args: Vec<String> = env::args().collect();
    //let path_string = &args[0];
    //let mut path = String::from("");
    // no argument will be interpreted as test
    //if path_string.is_empty() {
    let path = String::from("data/test/j301_0.sm");
    //} else {
    //    path = String::from(path_string);
    //}
    // FIle provided
    if Path::new(&path).file_name().is_some() {
        read_file(&path);
    } //else {
      // let paths = fs::read_dir(&path).unwrap();
      // for p in paths {
      //     read_file(p.unwrap().path().to_str().unwrap());
      //}
      //}
}
fn read_file(file: &str) {
    let arena = Bump::new();
    println!("Reading from: {:?}", file);
    let schedule = readerSM::read_input(file, &arena).unwrap();

    let mut segments: Vec<Rc<RefCell<Segment>>> = Vec::new();
    for project in schedule.projects().iter() {
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
    let mut id_gen = IdGenerator::generator_for_sat();

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
    for project in schedule.projects().iter() {
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

    fs::write(
        "data/test/test_file",
        Clause::write_list_to_string_hard(clauses),
    )
    .expect("Unable to write")
}
