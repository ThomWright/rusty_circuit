use specs;
use super::CircuitElement;
use super::Nodes;
use super::CalculatedCurrents;

pub const NAME: &'static str = "Voltage source";
const DEFAULT_VOLTAGE: f64 = 5.0;

pub struct VoltageSource();
impl specs::Component for VoltageSource {
    type Storage = specs::HashMapStorage<VoltageSource>;
}

pub struct Voltage(pub f64);
impl specs::Component for Voltage {
    type Storage = specs::HashMapStorage<Voltage>;
}

pub fn create_voltage_source(world: &mut specs::World) -> specs::Entity {
    world.create_now()
        .with(CircuitElement { display_name: NAME })
        .with(VoltageSource {})
        .with(Nodes::new(2))
        .with(Voltage(DEFAULT_VOLTAGE))
        .with(CalculatedCurrents::new(1))
        .build()
}
