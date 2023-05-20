use bumpalo::Bump;

pub mod project;

mod id_generator;
mod readerSM;
mod segment;
fn main() {
    let arena = Bump::new();
    let projects = readerSM::read_input("data/datasets/j30.sm/j3010_0.sm", &arena);
    println!("file read successfully, got the following projects: ");

    for project in projects.unwrap() {
        for segment in project.segments() {
            //println!("{}", segment);
        }
    }
}
