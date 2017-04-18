use specs;
use elements::Type;
use elements::CircuitElement;
use elements::Nodes;

pub const NAME: &'static str = "Current source";
pub const DEFAULT_CURRENT: f64 = 0.01;

pub fn create(world: &mut specs::World) -> specs::Entity {
    world.create_now()
        .with(CircuitElement {
            typ: Type::CurrentSource(DEFAULT_CURRENT),
            display_name: NAME,
        })
        .with(Nodes::new(2))
        .build()
}
