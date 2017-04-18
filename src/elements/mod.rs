use specs;

pub mod resistor;
pub mod voltage_source;
pub mod current_source;
pub mod wire;
pub mod ground;

#[derive(Debug, Clone, Copy)]
pub enum Type {
    CurrentSource(f64),
    Ground,
    Resistor(f64),
    VoltageSource(f64),
}

#[derive(Debug, Clone, Copy)]
pub struct CircuitElement {
    typ: Type,
    display_name: &'static str,
}
impl specs::Component for CircuitElement {
    type Storage = specs::VecStorage<CircuitElement>;
}
impl CircuitElement {
    pub fn typ(&self) -> Type {
        self.typ
    }
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

// TODO Make more specialised Node components? e.g. TwoNodeDirectional?
// Better type safety not indexing vectors directly.
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
