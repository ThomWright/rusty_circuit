
use solver::solve::SIM_TIME_PER_SEC;
use solver::tests::create_planner;
use solver::tests::run_loop_iteration_for_delta;

const V: f64 = 5.0;
const R: f64 = 100.0;
const C: f64 = 5e-6;

const TIME_CONSTANT: f64 = R * C;

const T: f64 = TIME_CONSTANT; // simulate one time constant

const ACCEPTABLE_DIFF: f64 = 0.02; // 2% leeway is a bit much, but hey-ho

#[test]
fn resistor_capacitor() {
    use specs::Gate;

    use elements::Nodes;
    use elements::CalculatedCurrent;
    use elements::DerivedCurrent;
    use elements::resistor;
    use elements::capacitor;
    use elements::voltage_source;

    // expectations after time = T
    let i_r = (V / R) * (-T / TIME_CONSTANT).exp(); // current through resistor and capacitor
    let q_c = C * V * (1.0 - (-T / TIME_CONSTANT).exp()); // charge on capacitor
    let v_c = q_c / C; // voltage across capacitor

    // Set up world
    let mut planner = create_planner();

    // Create an RC circuit
    let (resistor, capacitor, voltage_source) = {
        let mut world = planner.mut_world();
        let resistor = resistor::create(world);
        let capacitor = capacitor::create(world);
        let voltage_source = voltage_source::create(world);

        // TODO make 'create' functions which take values
        let mut rs = world.write::<resistor::Resistor>().pass();
        let r = rs.get_mut(resistor).unwrap();
        r.set_resistance(R);

        let mut cs = world.write::<capacitor::Capacitor>().pass();
        let c = cs.get_mut(capacitor).unwrap();
        c.capacitance = C;

        let mut vs = world.write::<voltage_source::VoltageSource>().pass();
        let v = vs.get_mut(voltage_source).unwrap();
        v.voltage = V;

        (resistor, capacitor, voltage_source)
    };

    // Assign node IDs
    {
        let world = planner.mut_world();
        let mut nodes = world.write::<Nodes>().pass();

        match nodes.get_mut(voltage_source).unwrap() {
            &mut Nodes(ref mut voltage_source_nodes) => {
                voltage_source_nodes[0].index = 0;
                voltage_source_nodes[1].index = 1;
            }
        }
        match nodes.get_mut(resistor).unwrap() {
            &mut Nodes(ref mut resistor_nodes) => {
                resistor_nodes[0].index = 1;
                resistor_nodes[1].index = 2;
            }
        }
        match nodes.get_mut(capacitor).unwrap() {
            &mut Nodes(ref mut capacitor_nodes) => {
                capacitor_nodes[0].index = 2;
                capacitor_nodes[1].index = 0;
            }
        }
    }

    run_loop_iteration_for_delta(&mut planner, T / SIM_TIME_PER_SEC);

    // Assert the circuit elements have the correct state
    let world = planner.mut_world();
    let nodes = world.read::<Nodes>().pass();
    match nodes.get(capacitor).unwrap() {
        &Nodes(ref capacitor_nodes) => {
            assert_eq!(capacitor_nodes[1].voltage, 0f64);
            assert_approx_eq!(capacitor_nodes[0].voltage, v_c, v_c * ACCEPTABLE_DIFF);
        }
    }

    let currents = world.read::<CalculatedCurrent>().pass();
    match currents.get(voltage_source).unwrap() {
        &CalculatedCurrent(current) => {
            assert_approx_eq!(current, i_r, i_r * ACCEPTABLE_DIFF);
        }
    }

    let currents = world.read::<DerivedCurrent>().pass();
    match currents.get(capacitor).unwrap() {
        &DerivedCurrent(current) => {
            assert_approx_eq!(current, i_r, i_r * ACCEPTABLE_DIFF);
        }
    }
}
