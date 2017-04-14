use specs;
use super::CircuitElement;
use super::Nodes;

pub const NAME: &'static str = "Current source";
pub const DEFAULT_CURRENT: f64 = 0.01;

#[derive(Debug, Clone, Copy)]
pub struct CurrentSource();
impl specs::Component for CurrentSource {
    type Storage = specs::HashMapStorage<CurrentSource>;
}

#[derive(Debug, Clone, Copy)]
pub struct CurrentInput(pub f64);
impl specs::Component for CurrentInput {
    type Storage = specs::HashMapStorage<CurrentInput>;
}

pub fn create(world: &mut specs::World) -> specs::Entity {
    world.create_now()
        .with(CircuitElement { display_name: NAME })
        .with(CurrentSource {})
        .with(Nodes::new(2))
        .with(CurrentInput(DEFAULT_CURRENT))
        .build()
}
