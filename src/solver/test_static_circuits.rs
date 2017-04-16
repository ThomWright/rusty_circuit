
use solver::test::create_planner;
use solver::test::run_solver_system;

#[test]
fn resistor_voltagesource() {
    use specs::Gate;

    use elements::Nodes;
    use elements::CalculatedCurrent;
    use elements::resistor;
    use elements::voltage_source;

    // Set up world
    let mut planner = create_planner();

    // Create a couple of circuit elements
    let (resistor, voltage_source) = {
        let mut world = planner.mut_world();
        let resistor = resistor::create(world);
        let voltage_source = voltage_source::create(world);
        (resistor, voltage_source)
    };

    // Assign node IDs
    {
        let world = planner.mut_world();
        let mut nodes = world.write::<Nodes>().pass();

        match nodes.get_mut(voltage_source) {
            Some(&mut Nodes(ref mut voltage_source_nodes)) => {
                voltage_source_nodes[0].index = 0;
                voltage_source_nodes[1].index = 1;
            }
            None => panic!("oh no"),
        }
        match nodes.get_mut(resistor) {
            Some(&mut Nodes(ref mut resistor_nodes)) => {
                resistor_nodes[0].index = 0;
                resistor_nodes[1].index = 1;
            }
            None => panic!("oh no"),
        }
    }

    // Run the solver
    run_solver_system(&mut planner);

    // Assert the circuit elements have the correct state
    let expected_voltage = voltage_source::DEFAULT_VOLTAGE;
    let world = planner.mut_world();
    let nodes = world.read::<Nodes>().pass();
    match nodes.get(resistor) {
        Some(&Nodes(ref resistor_nodes)) => {
            assert_eq!(resistor_nodes[0].voltage, 0f64);
            assert_eq!(resistor_nodes[1].voltage, expected_voltage);
        }
        None => panic!("oh no"),
    }
    match nodes.get(voltage_source) {
        Some(&Nodes(ref voltage_source_nodes)) => {
            assert_eq!(voltage_source_nodes[0].voltage, 0f64);
            assert_eq!(voltage_source_nodes[1].voltage, expected_voltage);
        }
        None => panic!("oh no"),
    }

    let expected_current = voltage_source::DEFAULT_VOLTAGE / resistor::DEFAULT_RESISTANCE;
    let currents = world.read::<CalculatedCurrent>().pass();
    match currents.get(voltage_source) {
        Some(&CalculatedCurrent(current)) => {
            assert_eq!(current, expected_current);
        }
        None => panic!("oh no"),
    }
}

#[test]
fn resistor_voltagesource_wire() {
    use specs::Gate;

    use elements::Nodes;
    use elements::CalculatedCurrent;
    use elements::resistor;
    use elements::wire;
    use elements::voltage_source;

    // Set up world
    let mut planner = create_planner();

    // Create circuit elements
    let (resistor, voltage_source, wire1, wire2) = {
        let mut world = planner.mut_world();
        let resistor = resistor::create(world);
        let voltage_source = voltage_source::create(world);
        let wire1 = wire::create(world);
        let wire2 = wire::create(world);
        (resistor, voltage_source, wire1, wire2)
    };

    // Assign node IDs
    {
        let world = planner.mut_world();
        let mut nodes = world.write::<Nodes>().pass();

        match nodes.get_mut(voltage_source) {
            Some(&mut Nodes(ref mut voltage_source_nodes)) => {
                voltage_source_nodes[0].index = 0;
                voltage_source_nodes[1].index = 1;
            }
            None => panic!("oh no"),
        }
        match nodes.get_mut(resistor) {
            Some(&mut Nodes(ref mut resistor_nodes)) => {
                resistor_nodes[0].index = 3;
                resistor_nodes[1].index = 2;
            }
            None => panic!("oh no"),
        }
        match nodes.get_mut(wire1) {
            Some(&mut Nodes(ref mut wire1_nodes)) => {
                wire1_nodes[0].index = 1;
                wire1_nodes[1].index = 2;
            }
            None => panic!("oh no"),
        }
        match nodes.get_mut(wire2) {
            Some(&mut Nodes(ref mut wire2_nodes)) => {
                wire2_nodes[0].index = 3;
                wire2_nodes[1].index = 0;
            }
            None => panic!("oh no"),
        }
    }

    // Run the solver
    run_solver_system(&mut planner);

    // Assert the circuit elements have the correct state
    let expected_voltage = voltage_source::DEFAULT_VOLTAGE;
    let world = planner.mut_world();
    let nodes = world.read::<Nodes>().pass();
    match nodes.get(resistor) {
        Some(&Nodes(ref resistor_nodes)) => {
            assert_eq!(resistor_nodes[0].voltage, 0f64);
            assert_eq!(resistor_nodes[1].voltage, expected_voltage);
        }
        None => panic!("oh no"),
    }
    match nodes.get(voltage_source) {
        Some(&Nodes(ref voltage_source_nodes)) => {
            assert_eq!(voltage_source_nodes[0].voltage, 0f64);
            assert_eq!(voltage_source_nodes[1].voltage, expected_voltage);
        }
        None => panic!("oh no"),
    }

    let expected_current = voltage_source::DEFAULT_VOLTAGE / resistor::DEFAULT_RESISTANCE;
    let currents = world.read::<CalculatedCurrent>().pass();
    match currents.get(voltage_source) {
        Some(&CalculatedCurrent(current)) => {
            assert_eq!(current, expected_current);
        }
        None => panic!("oh no"),
    }
    match currents.get(wire1) {
        Some(&CalculatedCurrent(current)) => {
            assert_eq!(current, expected_current);
        }
        None => panic!("oh no"),
    }
    match currents.get(wire2) {
        Some(&CalculatedCurrent(current)) => {
            assert_eq!(current, expected_current);
        }
        None => panic!("oh no"),
    }
}

#[test]
fn resistor_currentsource() {
    use specs::Gate;

    use elements::Nodes;
    use elements::resistor;
    use elements::current_source;

    // Set up world
    let mut planner = create_planner();

    // Create a couple of circuit elements
    let (resistor, current_source) = {
        let mut world = planner.mut_world();
        let resistor = resistor::create(world);
        let current_source = current_source::create(world);
        (resistor, current_source)
    };

    // Assign node IDs
    {
        let world = planner.mut_world();
        let mut nodes = world.write::<Nodes>().pass();

        match nodes.get_mut(current_source) {
            Some(&mut Nodes(ref mut current_source_nodes)) => {
                current_source_nodes[0].index = 0;
                current_source_nodes[1].index = 1;
            }
            None => panic!("oh no"),
        }
        match nodes.get_mut(resistor) {
            Some(&mut Nodes(ref mut resistor_nodes)) => {
                resistor_nodes[0].index = 0;
                resistor_nodes[1].index = 1;
            }
            None => panic!("oh no"),
        }
    }

    // Run the solver
    run_solver_system(&mut planner);

    // Assert the circuit elements have the correct state
    let expected_voltage = current_source::DEFAULT_CURRENT * resistor::DEFAULT_RESISTANCE;
    let world = planner.mut_world();
    let nodes = world.read::<Nodes>().pass();
    match nodes.get(resistor) {
        Some(&Nodes(ref resistor_nodes)) => {
            assert_eq!(resistor_nodes[0].voltage, 0f64);
            assert_eq!(resistor_nodes[1].voltage, expected_voltage);
        }
        None => panic!("oh no"),
    }
    match nodes.get(current_source) {
        Some(&Nodes(ref current_source_nodes)) => {
            assert_eq!(current_source_nodes[0].voltage, 0f64);
            assert_eq!(current_source_nodes[1].voltage, expected_voltage);
        }
        None => panic!("oh no"),
    }
}

#[test]
fn resistor_voltagesource_ground() {
    use specs::Gate;

    use elements::Nodes;
    use elements::resistor;
    use elements::ground;
    use elements::voltage_source;
    use elements::CalculatedCurrent;

    // Set up world
    let mut planner = create_planner();

    // Create circuit elements
    let (resistor, voltage_source, ground) = {
        let mut world = planner.mut_world();
        let resistor = resistor::create(world);
        let voltage_source = voltage_source::create(world);
        let ground = ground::create(world);
        (resistor, voltage_source, ground)
    };

    // Assign node IDs
    {
        let world = planner.mut_world();
        let mut nodes = world.write::<Nodes>().pass();

        match nodes.get_mut(voltage_source) {
            Some(&mut Nodes(ref mut voltage_source_nodes)) => {
                voltage_source_nodes[0].index = 1;
                voltage_source_nodes[1].index = 2;
            }
            None => panic!("oh no"),
        }
        match nodes.get_mut(resistor) {
            Some(&mut Nodes(ref mut resistor_nodes)) => {
                resistor_nodes[0].index = 1;
                resistor_nodes[1].index = 2;
            }
            None => panic!("oh no"),
        }
        match nodes.get_mut(ground) {
            Some(&mut Nodes(ref mut ground_node)) => {
                ground_node[0].index = 1;
            }
            None => panic!("oh no"),
        }
    }

    // Run the solver
    run_solver_system(&mut planner);

    // Assert the circuit elements have the correct state
    let expected_voltage = voltage_source::DEFAULT_VOLTAGE;
    let world = planner.mut_world();
    let nodes = world.read::<Nodes>().pass();
    match nodes.get(resistor) {
        Some(&Nodes(ref resistor_nodes)) => {
            assert_eq!(resistor_nodes[0].voltage, 0f64);
            assert_eq!(resistor_nodes[1].voltage, expected_voltage);
        }
        None => panic!("oh no"),
    }
    match nodes.get(voltage_source) {
        Some(&Nodes(ref voltage_source_nodes)) => {
            assert_eq!(voltage_source_nodes[0].voltage, 0f64);
            assert_eq!(voltage_source_nodes[1].voltage, expected_voltage);
        }
        None => panic!("oh no"),
    }

    let expected_current = 0f64;
    let currents = world.read::<CalculatedCurrent>().pass();
    match currents.get(ground) {
        Some(&CalculatedCurrent(current)) => {
            assert_eq!(current, expected_current);
        }
        None => panic!("oh no"),
    }
}
