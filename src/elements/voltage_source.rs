use specs;
use elements::Type;
use elements::CircuitElement;
use elements::Nodes;
use elements::CalculatedCurrent;

pub const NAME: &'static str = "Voltage source";
pub const DEFAULT_VOLTAGE: f64 = 5.0;

#[derive(Debug, Clone, Copy)]
pub struct VoltageInput(pub usize);
impl Default for VoltageInput {
    fn default() -> Self {
        VoltageInput(0)
    }
}
impl specs::Component for VoltageInput {
    type Storage = specs::HashMapStorage<VoltageInput>;
}

pub fn create(world: &mut specs::World) -> specs::Entity {
    world.create_now()
        .with(CircuitElement {
            typ: Type::VoltageSource(DEFAULT_VOLTAGE),
            display_name: NAME,
        })
        .with(Nodes::new(2))
        .with(VoltageInput::default())
        .with(CalculatedCurrent::default())
        .build()
}
