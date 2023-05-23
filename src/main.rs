use bumpalo::Bump;

use crate::segment::Segment;

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
    let mut segments: Vec<&Segment> = Vec::new();
    for project in projects.unwrap() {
        for segment in project.segments() {
            segments.push(segment);
            //println!("{:?}", segment.id());
        }
    }
    let distances = floyd_warshall::segments_dist_shortest_vec(&mut segments);
    println!("Debug distance checker: {:?}", distances)
}
