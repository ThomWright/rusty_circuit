use specs;
use elements::Type;
use elements::CircuitElement;
use elements::Nodes;

pub const NAME: &'static str = "Resistor";
pub const DEFAULT_RESISTANCE: f64 = 1000.0;

pub fn create(world: &mut specs::World) -> specs::Entity {
    world.create_now()
        .with(CircuitElement {
            typ: Type::Resistor(DEFAULT_RESISTANCE),
            display_name: NAME,
        })
        .with(Nodes::new(2))
        .build()
}
