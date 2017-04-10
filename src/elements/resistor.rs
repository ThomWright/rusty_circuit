use specs;
use super::CircuitElement;
use super::Nodes;

pub const NAME: &'static str = "Resistor";
const DEFAULT_RESISTANCE: f64 = 1000.0;

pub struct Resistor();
impl specs::Component for Resistor {
    type Storage = specs::HashMapStorage<Resistor>;
}

pub struct Resistance(pub f64);
impl specs::Component for Resistance {
    type Storage = specs::HashMapStorage<Resistance>;
}

pub fn create_resistor(world: &mut specs::World) -> specs::Entity {
    world.create_now()
        .with(CircuitElement { display_name: NAME })
        .with(Resistor {})
        .with(Nodes::new(2))
        .with(Resistance(DEFAULT_RESISTANCE))
        .build()
}
