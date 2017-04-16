
use specs;
use Delta;
use solver;

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

    let mut planner = specs::Planner::with_num_threads(world, 1);
    planner.add_system(solver::solve::System::default(), "solver", 10);

    planner
}

pub fn run_loop_iteration(planner: &mut specs::Planner<Delta>) {
    run_pre_loop(planner);
    run_update_loop(planner);
}

pub fn run_pre_loop(planner: &mut specs::Planner<Delta>) {
    let mut world = planner.mut_world();
    let equation = solver::create_static_equation(&mut world);
    world.add_resource(equation);
}

pub fn run_update_loop(planner: &mut specs::Planner<Delta>) {
    planner.dispatch(0.0);
    planner.wait();
}
