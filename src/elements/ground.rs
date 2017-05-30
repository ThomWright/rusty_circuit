use specs;
use elements::CircuitElement;
use elements::Nodes;
use elements::CalculatedCurrent;
use elements::voltage_source::VoltageSource;

pub const NAME: &'static str = "Ground";

pub fn create(world: &mut specs::World) -> specs::Entity {
    world.create_now()
        .with(CircuitElement { display_name: NAME })
        .with(Nodes::new(2))
        .with(VoltageSource::zero())
        .with(CalculatedCurrent::default())
        .build()
}
