use specs;
use elements::Nodes;
use elements::CalculatedCurrent;
use elements::ground::Ground;
use elements::resistor::Resistance;
use elements::voltage_source::VoltageInput;
use elements::current_source::CurrentInput;
use equation;

#[derive(Debug, Clone, Copy)]
pub struct System {}

impl System {
    pub fn new() -> Self {
        System {}
    }
}

impl Default for System {
    fn default() -> Self {
        System::new()
    }
}

impl specs::System<super::Delta> for System {
    fn run(&mut self, arg: specs::RunArg, _: super::Delta) {
        use specs::Join;
        let (mut nodes, mut calc_currents, resistances, v_inputs, c_inputs, grounds) =
            arg.fetch(|w| {
                (w.write::<Nodes>(),
                 w.write::<CalculatedCurrent>(),
                 w.read::<Resistance>(),
                 w.read::<VoltageInput>(),
                 w.read::<CurrentInput>(),
                 w.read::<Ground>())
            });

        // calculate basic circuit properties
        let num_nodes: usize = match (&nodes,)
            .join()
            .flat_map(|(&Nodes(ref ns),)| ns.iter())
            .max_by(|n1, n2| n1.index.cmp(&n2.index)) {
            Some(node) => node.index + 1,
            None => 0,
        };
        let num_v_inputs: usize = (&v_inputs,).join().count();

        let mut equation_builder = equation::Builder::new(num_nodes, num_v_inputs);

        // stamp circuit elements into the equation
        for (&Resistance(resistance), &Nodes(ref ns)) in (&resistances, &nodes).join() {
            equation_builder.stamp_resistor(resistance, ns[0].index, ns[1].index);
        }
        for (voltage_input, &Nodes(ref ns), _) in (&v_inputs, &nodes, !&grounds).join() {
            equation_builder.stamp_voltage_source(voltage_input.voltage,
                                                  ns[0].index,
                                                  ns[1].index,
                                                  voltage_input.index);
        }
        // ground voltage inputs are a special case, they always connect to node 0
        for (voltage_input, &Nodes(ref ns), _) in (&v_inputs, &nodes, &grounds).join() {
            equation_builder.stamp_voltage_source(voltage_input.voltage,
                                                  0,
                                                  ns[0].index,
                                                  voltage_input.index);
        }
        for (&CurrentInput(current), &Nodes(ref ns)) in (&c_inputs, &nodes).join() {
            equation_builder.stamp_current_source(current, ns[0].index, ns[1].index);
        }

        // solve
        match equation_builder.build().and_then(|equation| equation.solve()) {
            Ok(solution) => {
                let voltages = solution.voltages();
                let currents = solution.currents();

                // update circuit element states
                for (&mut Nodes(ref mut ns),) in (&mut nodes,).join() {
                    for ref mut node in ns.iter_mut() {
                        node.voltage = voltages[node.index];
                    }
                }
                for (v_input, &mut CalculatedCurrent(ref mut current)) in
                    (&v_inputs, &mut calc_currents).join() {
                    *current = currents[v_input.index];
                }
            }
            Err(error) => println!("Unsolvable circuit: {}", error),
        }
    }
}

#[cfg(test)]
mod tests {
    use specs;
    use Delta;

    #[test]
    fn circuit_with_resistor_voltagesource() {
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

        assign_voltage_source_indexes(&mut planner);

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
    fn circuit_with_resistor_voltagesource_wire() {
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

        assign_voltage_source_indexes(&mut planner);

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
    fn circuit_with_resistor_currentsource() {
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
    fn circuit_with_resistor_voltagesource_ground() {
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

        assign_voltage_source_indexes(&mut planner);

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

    fn create_planner() -> specs::Planner<Delta> {
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

    fn assign_voltage_source_indexes(planner: &mut specs::Planner<Delta>) {
        use elements::voltage_source::VoltageInput;
        use specs::Join;
        use specs::Gate;

        let world = planner.mut_world();
        let mut v_inputs = world.write::<VoltageInput>().pass();

        let mut i = 0;
        for (ref mut vi,) in (&mut v_inputs,).join() {
            vi.index = i;
            i += 1;
        }
    }

    fn run_solver_system(planner: &mut specs::Planner<Delta>) {
        use solver::System;
        planner.add_system(System::new(), "solver", 10);
        planner.dispatch(0.0);
        planner.wait();
    }
}
