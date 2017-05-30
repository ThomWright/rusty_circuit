use specs;
use elements::CircuitElement;
use elements::Nodes;

pub const NAME: &'static str = "Current source";
pub const DEFAULT_CURRENT: f64 = 0.01;

#[derive(Debug, Clone, Copy)]
pub struct CurrentSource {
    pub current: f64,
    node_indexes: (usize, usize),
}
impl CurrentSource {
    pub fn node_index_from(&self) -> usize {
        self.node_indexes.0
    }
    pub fn node_index_to(&self) -> usize {
        self.node_indexes.1
    }
}
impl Default for CurrentSource {
    fn default() -> Self {
        CurrentSource {
            current: DEFAULT_CURRENT,
            node_indexes: (0, 1),
        }
    }
}
impl specs::Component for CurrentSource {
    type Storage = specs::HashMapStorage<CurrentSource>;
}

pub fn create(world: &mut specs::World) -> specs::Entity {
    world.create_now()
        .with(CircuitElement { display_name: NAME })
        .with(Nodes::new(2))
        .with(CurrentSource::default())
        .build()
}
