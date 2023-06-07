pub mod readerSM;
pub mod schedule;

use bumpalo::Bump;
//use clap::Parser;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use schedule::Schedule;
use shared::{
    floyd_warshall,
    id_generator::IdGenerator,
    sat_seg_var::{Clause, SATSVar, SATUVar},
    segment::Segment,
};
use std::{
    cell::RefCell,
    env,
    fs::{self, read_dir},
    path::Path,
    rc::Rc,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    // Path provided as the first argument
    if args.len() <= 1 {
        let path = String::from("data/test/j301_0.sm");
        batch_file(&path);
    } else {
        let path_string = &args[1];
        // Turn into dir
        let dir_check = Path::new(path_string);
        // Single file option
        if dir_check.is_file() {
            batch_file(dir_check.to_str().unwrap());
        }
        // Entire directory given
        else if dir_check.is_dir() {
            let dir = read_dir(dir_check).unwrap();
            for file in dir.into_iter() {
                let file_path = file.unwrap().path();
                let file_name = file_path.to_str().unwrap();

                batch_file(file_name)
            }
        } else {
            println!(
                "Argument provided: [{:?}], but not path or directory :(",
                path_string
            );
        }
    }
}
fn strip_ending(file_name: &str) -> &str {
    file_name
        .split('/')
        .last()
        .unwrap()
        .split('.')
        .next()
        .unwrap()
}
fn get_ending(file_name: &str) -> &str {
    file_name
        .split('/')
        .last()
        .unwrap()
        .split('.')
        .last()
        .unwrap()
}
fn batch_file(file: &str) {
    if get_ending(file) != "sm" {
        return;
    }
    let arena = Bump::new();
    let schedule = read_file(file, &arena);

    for set_up_time in -1..6 {
        // this is the file name that will be created
        let destination = &[
            "data/parsed/",
            strip_ending(file),
            "/",
            &(set_up_time + 1).to_string(),
            "F",
            strip_ending(file),
            ".wcnf",
        ]
        .concat();
        fs::create_dir_all(
            Path::new(destination)
                .parent()
                .unwrap_or(Path::new(destination)),
        );

        write_file(&schedule, 1, destination);
    }
}
fn get_sat_dir(file: &str) -> &str {
    ""
}
fn create_solved_dir(file: &str) -> &str {
    ""
}
fn read_file<'a>(file: &'a str, arena: &'a Bump) -> Schedule<'a> {
    println!("Reading from: {:?}", file);
    let schedule = readerSM::read_input(file, arena).unwrap();
    schedule
}

fn write_file(schedule: &Schedule, set_up_addition: u32, destination: &str) {
    let mut segments: Vec<Rc<RefCell<Segment>>> = Vec::new();
    let mut horizon = 0;

    for project in schedule.projects.iter() {
        horizon += project.duration() as u64;
        for segment in project.segments() {
            // we are adding, and this is a side effect
            segment.borrow_mut().add_set_up_time(set_up_addition);
            segments.push(segment.clone());
        }
    }
    // sort by id
    segments.sort_by_key(|k| k.borrow().id());

    let distances = floyd_warshall::segments_dist_shortest_vec(&segments);
    let critical_path = horizon.max(
        floyd_warshall::segments_dist_longest_vec(&segments)
            .iter()
            .flatten()
            .min()
            .unwrap()
            .unsigned_abs() as u64,
    );
    println!("Floyd Warshall called");
    let mut id_gen = IdGenerator::generator_for_sat();

    let mut clauses: Vec<Clause> = Vec::new();
    for segment in segments.iter() {
        let early_start = distances[segment.borrow().id() as usize][0].unsigned_abs() as u64;

        segment
            .borrow_mut()
            .generate_SAT_vars(&mut id_gen, early_start, critical_path);
        for clause in segment.borrow().generate_precedence_clauses() {
            clauses.push(clause);
        }
        for clause in segment.borrow().generate_consistency_clause() {
            clauses.push(clause);
        }
    }

    for project in schedule.projects.iter() {
        for clause in project.generate_completion_clauses() {
            clauses.push(clause);
        }
    }
    println!("Clause generation done");
    let mut u_vars: Vec<Rc<SATUVar>> = Vec::new();
    let mut s_vars: Vec<Rc<SATSVar>> = Vec::new();
    for project in schedule.projects.iter() {
        for segment in project.segments().iter() {
            for u_var in segment.borrow().uvariables.borrow().iter() {
                u_vars.push(Rc::clone(u_var));
            }
            for s_var in segment.borrow().variables.borrow().iter() {
                s_vars.push(Rc::clone(s_var));
            }
        }
    }
    println!("Calling python");
    let mut resource_clauses = Clause::u_vec_accum(u_vars, &mut id_gen, schedule.resources.clone());
    clauses.append(&mut resource_clauses);
    s_vars.sort_by_key(|e| (e.weight(), e.time(), -(e.segment_duration() as i64)));
    let mut s_order: Vec<u64> = s_vars.iter().map(|s| s.id()).collect();
    // Soft clause (maxspan) generation (and the thing we measure at the end)
    let last = schedule.projects.last().unwrap();
    println!("Generating clauses for final variabel");
    let mut last_svars: Vec<Rc<SATSVar>> = Vec::new();
    for segment in last.segments().iter() {
        for variable in segment.borrow().variables.borrow().iter() {
            last_svars.push(Rc::clone(variable));
        }
    }
    last_svars.sort_by_key(|u| u.time());
    let min_end = distances[last.id() as usize][0].unsigned_abs() as usize;
    let weights: Vec<usize> = last_svars.iter().map(|x| x.time() as usize).collect();
    let weight = weights.iter().max().unwrap() + 1;
    //let weights: Vec<usize> = (1 + min_end..weight + 1 + min_end).rev().collect();
    let last_s_clause = SATSVar::last_to_clause(&last_svars);

    let mut unused: Vec<u64> = (1..id_gen.current_asignment() + 1)
        .filter(|a| !s_order.contains(a))
        .collect();
    s_order.append(&mut unused);

    println!("Writing to: {}", destination);
    fs::write(
        destination,
        first_line(s_order)
            + &Clause::write_first_line(
                clauses.len() + last_s_clause.len(),
                id_gen.current_asignment() as usize,
                weight + min_end,
            )
            + &Clause::write_list_to_string_hard(clauses, weight + min_end)
            + &Clause::write_list_to_string_soft(last_s_clause, weights),
    )
    .expect("Unable to write")
}
pub fn first_line(vec: Vec<u64>) -> String {
    "c ".to_string() + &vec.iter().map(|x| x.to_string() + " ").collect::<String>() + "\n"
}
#[cfg(test)]
mod test {
    #[test]
    fn counterExampleFileProd() {}
}
