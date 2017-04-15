use specs;
use super::CircuitElement;
use super::Nodes;
use super::CalculatedCurrent;
use super::voltage_source::VoltageInput;

pub const NAME: &'static str = "Ground";

#[derive(Debug, Clone, Copy)]
pub struct Ground();
impl specs::Component for Ground {
    type Storage = specs::HashMapStorage<Ground>;
}

pub fn create(world: &mut specs::World) -> specs::Entity {
    world.create_now()
        .with(CircuitElement { display_name: NAME })
        .with(Ground {})
        .with(Nodes::new(1))
        .with(VoltageInput::new(0f64))
        .with(CalculatedCurrent::default())
        .build()
}
