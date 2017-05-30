use specs;

pub mod capacitor;
pub mod resistor;
pub mod voltage_source;
pub mod current_source;
pub mod wire;
pub mod ground;

#[derive(Debug, Clone, Copy)]
pub struct CircuitElement {
    display_name: &'static str,
}
impl specs::Component for CircuitElement {
    type Storage = specs::VecStorage<CircuitElement>;
}
impl CircuitElement {
    pub fn display_name(&self) -> &'static str {
        self.display_name
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub index: usize,
    pub voltage: f64,
}
impl Default for Node {
    fn default() -> Self {
        Node {
            index: 0,
            voltage: 0f64,
        }
    }
}

#[derive(Debug)]
pub struct Nodes(pub Vec<Node>);
impl Nodes {
    pub fn new(num: usize) -> Self {
        let mut nodes = Vec::new();
        for _ in 0..num {
            nodes.push(Node::default())
        }
        Nodes(nodes)
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

impl Default for CalculatedCurrent {
    fn default() -> Self {
        CalculatedCurrent(0f64)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DerivedCurrent(pub f64);
impl specs::Component for DerivedCurrent {
    type Storage = specs::HashMapStorage<DerivedCurrent>;
}

impl Default for DerivedCurrent {
    fn default() -> Self {
        DerivedCurrent(0f64)
    }
}
