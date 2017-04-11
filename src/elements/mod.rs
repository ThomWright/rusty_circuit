use specs;

pub mod resistor;
pub mod voltage_source;

#[derive(Debug, Clone, Copy)]
pub struct CircuitElement {
    pub display_name: &'static str,
}
impl specs::Component for CircuitElement {
    type Storage = specs::VecStorage<CircuitElement>;
}

// TODO Make more specialised Node components? e.g. TwoNodeDirectional?
// Better type safety not indexing vectors directly.
#[derive(Debug)]
pub struct Nodes {
    pub ids: Vec<usize>,
    pub voltages: Vec<f64>,
    num: usize,
}
impl Nodes {
    pub fn new(num: usize) -> Self {
        Nodes {
            ids: Vec::with_capacity(2),
            voltages: Vec::with_capacity(2),
            num: num,
        }
    }

    pub fn num(&self) -> usize {
        self.num
    }
}
impl specs::Component for Nodes {
    type Storage = specs::VecStorage<Nodes>;
}

#[derive(Debug, Clone, Copy)]
pub struct CalculatedCurrent(pub f64);
impl specs::Component for CalculatedCurrent {
    type Storage = specs::HashMapStorage<CalculatedCurrent>;
}
