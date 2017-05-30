use specs;
use elements::resistor::Resistor;
use elements::current_source::CurrentSource;
use elements::CircuitElement;
use elements::Nodes;
use elements::DerivedCurrent;

pub const NAME: &'static str = "Capacitor";
pub const DEFAULT_CAPACITANCE: f64 = 10e-6;

#[derive(Debug, Clone, Copy)]
pub enum CompanionModel {
    Norton,
    Thevenin,
}

#[derive(Debug, Clone, Copy)]
pub enum IntegrationMethod {
    Trapezoidal,
    ForwardEuler,
    BackwardEuler,
}

#[derive(Debug, Clone, Copy)]
pub struct Capacitor {
    pub capacitance: f64,

    pub resistor: Resistor,
    pub current_source: CurrentSource,

    pub node_indexes: (usize, usize),

    pub companion_model: CompanionModel,
    pub integration_method: IntegrationMethod,
}
impl Default for Capacitor {
    fn default() -> Self {
        Capacitor {
            capacitance: DEFAULT_CAPACITANCE,

            resistor: Resistor::default(),
            current_source: CurrentSource::default(),

            node_indexes: (0, 1),

            // not used, but interesting to know
            companion_model: CompanionModel::Norton,
            integration_method: IntegrationMethod::Trapezoidal,
        }
    }
}
impl specs::Component for Capacitor {
    type Storage = specs::HashMapStorage<Capacitor>;
}

pub fn create(world: &mut specs::World) -> specs::Entity {
    world.create_now()
        .with(CircuitElement { display_name: NAME })
        .with(Nodes::new(2))
        .with(Capacitor::default())
        .with(DerivedCurrent::default())
        .build()
}
