use specs;
use super::CircuitElement;
use super::Nodes;

pub const NAME: &'static str = "Resistor";
pub const DEFAULT_RESISTANCE: f64 = 1000.0;

#[derive(Debug, Clone, Copy)]
pub struct Resistor();
impl specs::Component for Resistor {
    type Storage = specs::HashMapStorage<Resistor>;
}

#[derive(Debug, Clone, Copy)]
pub struct Resistance(pub f64);
impl specs::Component for Resistance {
    type Storage = specs::HashMapStorage<Resistance>;
}

pub fn create(world: &mut specs::World) -> specs::Entity {
    world.create_now()
        .with(CircuitElement { display_name: NAME })
        .with(Resistor {})
        .with(Nodes::new(2))
        .with(Resistance(DEFAULT_RESISTANCE))
        .build()
}
