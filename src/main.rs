pub mod project;

mod id_generator;
mod readerSM;
mod segment;
fn main() {
    readerSM::read_input("data/datasets/j30.sm/j3010_0.sm");
}
