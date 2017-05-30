use specs;
use elements::CircuitElement;
use elements::Nodes;
use elements::CalculatedCurrent;
use elements::voltage_source::VoltageSource;

pub const NAME: &'static str = "Wire";

pub fn create(world: &mut specs::World) -> specs::Entity {
    // model wire as a 0V voltage source
    // can't model as 0 ohm resistor because this causes division by zero to get
    // conductance
    world.create_now()
        .with(CircuitElement { display_name: NAME })
        .with(Nodes::new(2))
        .with(VoltageSource::zero())
        .with(CalculatedCurrent::default())
        .build()
}
