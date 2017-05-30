use specs;
use elements::CircuitElement;
use elements::Nodes;

pub const NAME: &'static str = "Resistor";
pub const DEFAULT_RESISTANCE: f64 = 1e3;

#[derive(Debug, Clone, Copy)]
pub struct Resistor {
    resistance: f64,
    pub node_indexes: (usize, usize),
}
impl Resistor {
    pub fn resistance(&self) -> f64 {
        return self.resistance;
    }
    pub fn set_resistance(&mut self, resistance: f64) {
        self.resistance = resistance;
    }
    pub fn conductance(&self) -> f64 {
        return 1.0 / self.resistance;
    }
    pub fn set_conductance(&mut self, conductance: f64) {
        self.resistance = 1.0 / conductance;
    }
}
impl Default for Resistor {
    fn default() -> Self {
        Resistor {
            resistance: DEFAULT_RESISTANCE,
            node_indexes: (0, 1),
        }
    }
}
impl specs::Component for Resistor {
    type Storage = specs::HashMapStorage<Resistor>;
}

pub fn create(world: &mut specs::World) -> specs::Entity {
    world.create_now()
        .with(CircuitElement { display_name: NAME })
        .with(Nodes::new(2))
        .with(Resistor::default())
        .build()
}
