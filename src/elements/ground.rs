use specs;
use elements::Type;
use elements::CircuitElement;
use elements::Nodes;
use elements::CalculatedCurrent;
use elements::voltage_source::VoltageInput;

pub const NAME: &'static str = "Ground";

pub fn create(world: &mut specs::World) -> specs::Entity {
    world.create_now()
        .with(CircuitElement {
            typ: Type::Ground,
            display_name: NAME,
        })
        .with(Nodes::new(1))
        .with(VoltageInput::default())
        .with(CalculatedCurrent::default())
        .build()
}
