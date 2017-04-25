use specs;
use elements::Type;
use elements::CircuitElement;
use elements::Nodes;
use elements::CalculatedCurrent;

pub const NAME: &'static str = "Voltage source";
pub const DEFAULT_VOLTAGE: f64 = 5.0;

#[derive(Debug, Clone, Copy)]
pub struct VoltageSource {
    pub voltage: f64,
    pub index: usize,
    node_indexes: (usize, usize),
}
impl VoltageSource {
    pub fn zero() -> Self {
        VoltageSource {
            voltage: 0f64,
            index: 0,
            node_indexes: (0, 1),
        }
    }
    pub fn node_index_from(&self) -> usize {
        self.node_indexes.0
    }
    pub fn node_index_to(&self) -> usize {
        self.node_indexes.1
    }
}
impl Default for VoltageSource {
    fn default() -> Self {
        VoltageSource {
            voltage: DEFAULT_VOLTAGE,
            index: 0,
            node_indexes: (0, 1),
        }
    }
}
impl specs::Component for VoltageSource {
    type Storage = specs::HashMapStorage<VoltageSource>;
}

pub fn create(world: &mut specs::World) -> specs::Entity {
    world.create_now()
        .with(CircuitElement {
            typ: Type::VoltageSource,
            display_name: NAME,
        })
        .with(Nodes::new(2))
        .with(VoltageSource::default())
        .with(CalculatedCurrent::default())
        .build()
}
