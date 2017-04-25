use specs;
use elements::Type;
use elements::CircuitElement;
use elements::Nodes;

pub const NAME: &'static str = "Resistor";
pub const DEFAULT_RESISTANCE: f64 = 1000.0;

#[derive(Debug, Clone, Copy)]
pub struct Resistor {
    pub resistance: f64,
    pub node_indexes: (usize, usize),
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
        .with(CircuitElement {
            typ: Type::Resistor,
            display_name: NAME,
        })
        .with(Nodes::new(2))
        .with(Resistor::default())
        .build()
}
