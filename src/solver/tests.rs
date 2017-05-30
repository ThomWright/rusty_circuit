use specs;
use Delta;
use solver;

const SINGLE_FRAME: f64 = 1.0 / 60.0;

pub fn create_planner() -> specs::Planner<Delta> {
    use elements::Nodes;
    use elements::CalculatedCurrent;
    use elements::DerivedCurrent;
    use elements::CircuitElement;
    use elements::resistor::Resistor;
    use elements::current_source::CurrentSource;
    use elements::voltage_source::VoltageSource;
    use elements::capacitor::Capacitor;

    let mut world = specs::World::new();
    world.register::<CircuitElement>();
    world.register::<Nodes>();
    world.register::<CalculatedCurrent>();
    world.register::<DerivedCurrent>();
    world.register::<VoltageSource>();
    world.register::<CurrentSource>();
    world.register::<Resistor>();
    world.register::<Capacitor>();

    let mut planner = specs::Planner::with_num_threads(world, 1);
    planner.add_system(solver::solve::System::default(), "solver", 10);

    planner
}

pub fn run_loop_iteration(planner: &mut specs::Planner<Delta>) {
    run_pre_loop(planner);
    run_update_loop(planner);
}

pub fn run_loop_iteration_for_delta(planner: &mut specs::Planner<Delta>, delta: Delta) {
    run_pre_loop(planner);
    run_update_loop_for_delta(planner, delta);
}

fn run_pre_loop(planner: &mut specs::Planner<Delta>) {
    let mut world = planner.mut_world();
    let equation = solver::create_static_equation(&mut world);
    world.add_resource(equation);
}

fn run_update_loop(planner: &mut specs::Planner<Delta>) {
    planner.dispatch(SINGLE_FRAME);
    planner.wait();
}

fn run_update_loop_for_delta(planner: &mut specs::Planner<Delta>, delta: Delta) {
    planner.dispatch(delta);
    planner.wait();
}
