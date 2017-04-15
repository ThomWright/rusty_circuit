use specs;
use super::CircuitElement;
use super::Nodes;
use super::CalculatedCurrent;
use super::voltage_source::VoltageInput;

pub const NAME: &'static str = "Wire";

#[derive(Debug, Clone, Copy)]
pub struct Wire();
impl specs::Component for Wire {
    type Storage = specs::HashMapStorage<Wire>;
}

pub fn create(world: &mut specs::World) -> specs::Entity {
    world.create_now()
        .with(CircuitElement { display_name: NAME })
        .with(Wire {})
        // model wire as a 0V voltage source
        // can't model as 0 ohm resistor because this causes division by zero to get conductance
        .with(Nodes::new(2))
        .with(VoltageInput::new(0f64))
        .with(CalculatedCurrent::default())
        .build()
}
