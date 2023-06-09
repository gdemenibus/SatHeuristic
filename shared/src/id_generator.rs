#[derive()]
pub struct IdGenerator(pub usize);
impl IdGenerator {
    pub fn next_id(&mut self) -> usize {
        let res = self.0;
        self.0 += 1;
        res
    }
    pub fn generator_for_sat() -> Self {
        IdGenerator(1)
    }
    pub fn generator_for_segment() -> Self {
        IdGenerator(0)
    }
    pub fn current_asignment(&self) -> usize {
        self.0 - 1
    }
    pub fn new_current(&mut self, num: usize) {
        self.0 = num + 1;
    }
}

#[cfg(test)]
#[test]
fn unique_id_gen() {
    let mut generator = IdGenerator(0);
    let id1 = generator.next_id();
    let id2 = generator.next_id();
    assert_ne!(id1, id2);
}
