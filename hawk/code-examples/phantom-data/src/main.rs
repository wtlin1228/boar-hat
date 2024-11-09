#[derive(Debug, Default)]
pub struct Building;

#[derive(Debug, Default)]
pub struct Built;

#[derive(Debug)]
pub struct Engine<S = Built> {
    marker: std::marker::PhantomData<S>,
    data: Vec<usize>,
}

impl Engine<Building> {
    pub fn new() -> Self {
        Self {
            marker: std::marker::PhantomData,
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, n: usize) -> () {
        self.data.push(n);
    }

    pub fn seal(self) -> Engine<Built> {
        let Engine { data, .. } = self;
        Engine {
            marker: std::marker::PhantomData,
            data,
        }
    }
}

impl Default for Engine<Building> {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine<Built> {
    pub fn get_data(&self) -> &Vec<usize> {
        &self.data
    }
}

fn main() {
    let mut engine: Engine<Building> = Engine::default();
    engine.push(42);
    engine.push(43);

    let engine: Engine<Built> = engine.seal();
    let data = engine.get_data(); // Engine<Built> has only one method
}
