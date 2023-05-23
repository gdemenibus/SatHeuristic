use std::{cell::RefCell, rc::Rc};

use bumpalo::Bump;

use crate::{id_generator::IdGenerator, sat_seg_var::Times, segment::Segment};

pub mod project;

pub mod floyd_warshall;
mod id_generator;
mod readerSM;
pub mod sat_seg_var;
mod segment;
fn main() {
    let arena = Bump::new();
    let projects = readerSM::read_input("data/datasets/j30.sm/j3010_0.sm", &arena);
    println!("file read successfully, got the following projects: ");
    let mut segments: Vec<&Rc<Segment>> = projects
        .unwrap()
        .iter()
        .flat_map(|p| p.segments())
        .collect();
    // Sort segments here
    segments.sort_by_key(|a| a.id());
    let distances = floyd_warshall::segments_dist_shortest_vec(segments.clone());
    let long_distance = floyd_warshall::segments_dist_longest_vec(segments.clone());
    let critical_path = long_distance.iter().flatten().min().unwrap() * -1;
    let id_gen = &mut IdGenerator::generator_for_sat();

    let mut sat_vars = Vec::new();
    let seg_times = Times::new(1, critical_path as u64, 0);
    for segment in segments {
        // Could be the other way around check dock
        let early_start = distances[segment.id() as usize][0];
        // Busted
        //let seg_times = Times::new(early_start as u64, critical_path as u64, segment.id());
        let sat_var = segment.generate_SAT_vars(id_gen, &seg_times);
        sat_vars.extend(sat_var);
    }
    println!("Var count: {:?}", sat_vars.len());
}
