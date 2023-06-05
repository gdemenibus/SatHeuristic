pub mod readerSM;
pub mod schedule;

use bumpalo::Bump;
//use clap::Parser;
use std::{
    cell::RefCell,
    env,
    fs::{self, read_dir},
    path::Path,
    rc::Rc,
};

use shared::{
    floyd_warshall,
    id_generator::IdGenerator,
    sat_seg_var::{Clause, SATUVar},
    segment::Segment,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    // Path provided as the first argument
    if args.len() <= 1 {
        let path = String::from("data/test/j301_0.sm");
        read_file(&path, 4, "data/test/test_file");
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
            for file in dir {
                //read_file(file.unwrap().path().to_str().unwrap(), 4, "NO");
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
fn batch_file(file: &str) {
    for set_up_time in 0..10 {
        // this is the file name that will be created
        let destination = &[
            "data/parsed/",
            strip_ending(file),
            "/",
            &set_up_time.to_string(),
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
        read_file(file, set_up_time, destination);
    }
}
fn get_sat_dir(file: &str) -> &str {
    ""
}
fn create_solved_dir(file: &str) -> &str {
    ""
}

fn read_file(file: &str, set_up_time: u32, destination: &str) {
    let arena = Bump::new();
    println!("Reading from: {:?}", file);
    let schedule = readerSM::read_input(file, &arena, set_up_time).unwrap();

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
        counter += segment.borrow().uvariables.borrow().len();
        for clause in segment.borrow().generate_precedence_clauses() {
            clauses.push(clause);
        }
        for clause in segment.borrow().generate_consistency_clause() {
            clauses.push(clause);
        }
        println!("{}", segment.borrow());
    }
    // Error is being thrown here
    for project in schedule.projects().iter() {
        for clause in project.generate_completion_clauses() {
            clauses.push(clause);
        }
    }
    // Soft clause (maxspan) generation (and the thing we measure at the end)
    let last = schedule.projects().last().unwrap();
    let mut uvars: Vec<Rc<SATUVar>> = Vec::new();
    for segment in last.segments().iter() {
        for variable in segment.borrow().uvariables.borrow().iter() {
            uvars.push(Rc::clone(variable));
        }
    }
    uvars.sort_by_key(|u| u.time_at());
    let weight = uvars.len();
    let weights: Vec<usize> = (1..weight + 1).rev().collect();
    let uclause = SATUVar::last_to_clause(&uvars);

    println!("Total Number of Vars: {:?}", counter);
    println!(
        "Total Number of clauses: {:?}",
        clauses.len() + uclause.len()
    );

    println!("{}", destination);
    fs::write(
        destination,
        Clause::write_first_line(clauses.len(), counter, weight)
            + &Clause::write_list_to_string_hard(clauses, weight)
            + &Clause::write_list_to_string_soft(uclause, weights),
    )
    .expect("Unable to write")
}
#[cfg(test)]
mod test {
    #[test]
    fn counterExampleFileProd() {}
}
