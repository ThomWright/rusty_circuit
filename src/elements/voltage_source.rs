use specs;
use super::CircuitElement;
use super::Nodes;
use super::CalculatedCurrent;

pub const NAME: &'static str = "Voltage source";
pub const DEFAULT_VOLTAGE: f64 = 5.0;

#[derive(Debug, Clone, Copy)]
pub struct VoltageSource();
impl specs::Component for VoltageSource {
    type Storage = specs::HashMapStorage<VoltageSource>;
}

#[derive(Debug, Clone, Copy)]
pub struct VoltageInput {
    pub voltage: f64,
    pub index: usize,
}
impl VoltageInput {
    pub fn new(voltage: f64) -> Self {
        VoltageInput {
            voltage: voltage,
            index: 0,
        }
    }
}
impl Default for VoltageInput {
    fn default() -> Self {
        VoltageInput::new(DEFAULT_VOLTAGE)
    }
}
impl specs::Component for VoltageInput {
    type Storage = specs::HashMapStorage<VoltageInput>;
}

pub fn create(world: &mut specs::World) -> specs::Entity {
    world.create_now()
        .with(CircuitElement { display_name: NAME })
        .with(VoltageSource {})
        .with(Nodes::new(2))
        .with(VoltageInput::default())
        .with(CalculatedCurrent::default())
        .build()
}
