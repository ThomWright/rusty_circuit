
use specs;
use Delta;

pub fn create_planner() -> specs::Planner<Delta> {
    use elements::Nodes;
    use elements::CalculatedCurrent;
    use elements::CircuitElement;
    use elements::wire::Wire;
    use elements::ground::Ground;
    use elements::resistor::Resistance;
    use elements::resistor::Resistor;
    use elements::voltage_source::VoltageInput;
    use elements::voltage_source::VoltageSource;
    use elements::current_source::CurrentInput;
    use elements::current_source::CurrentSource;

    let mut world = specs::World::new();
    world.register::<CircuitElement>();
    world.register::<Nodes>();
    world.register::<CalculatedCurrent>();
    world.register::<Wire>();
    world.register::<Ground>();
    world.register::<Resistor>();
    world.register::<Resistance>();
    world.register::<VoltageSource>();
    world.register::<VoltageInput>();
    world.register::<CurrentSource>();
    world.register::<CurrentInput>();

    specs::Planner::with_num_threads(world, 1)
}

pub fn run_solver_system(planner: &mut specs::Planner<Delta>) {
    use solver::System;
    planner.add_system(System::new(), "solver", 10);
    planner.dispatch(0.0);
    planner.wait();
}
