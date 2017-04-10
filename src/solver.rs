use specs;
use elements::*;
use elements::resistor::*;
use elements::voltage_source::*;
use equation::EquationBuilder;

pub struct System {}

impl System {
    pub fn new() -> Self {
        System {}
    }
}

impl specs::System<super::Delta> for System {
    fn run(&mut self, arg: specs::RunArg, _: super::Delta) {
        use specs::Join;
        let (mut nodes, mut calculated_currents, resistances, voltages) = arg.fetch(|w| {
            (w.write::<Nodes>(),
             w.write::<CalculatedCurrents>(),
             w.read::<Resistance>(),
             w.read::<Voltage>())
        });

        let num_nodes: usize = match (&nodes,)
            .join()
            .flat_map(|(ref ns,)| ns.ids.iter())
            .max() {
            Some(num) => num + 1,
            None => 0,
        };
        let num_calculated_currents: usize =
            (&calculated_currents,).join().map(|(ref ccs,)| ccs.num()).sum();

        let mut equation_builder = EquationBuilder::new(num_nodes, num_calculated_currents);

        for (&Resistance(resistance), ref ns) in (&resistances, &nodes).join() {
            equation_builder.stamp_resistor(resistance, ns.ids[0], ns.ids[1]);
        }
        for (i, (&Voltage(voltage), ref ns)) in (&voltages, &nodes).join().enumerate() {
            equation_builder.stamp_voltage_source(voltage, ns.ids[0], ns.ids[1], i);
        }

        if let Ok(solution) = equation_builder.build().and_then(|equation| equation.solve()) {
            let voltages = solution.voltages();
            let currents = solution.currents();

            for (ref mut ns,) in (&mut nodes,).join() {
                ns.voltages.clear();
                for &id in ns.ids.iter() {
                    ns.voltages.push(voltages[id]);
                }
            }
            for (ref mut ccs,) in (&mut calculated_currents,).join() {
                ccs.currents.clear();
                for &id in ccs.ids.iter() {
                    ccs.currents.push(currents[id]);
                }
            }
        } else {
            // oh no
            panic!("sadface")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testicles() {
        use specs::Gate;

        // Set up world
        let mut planner = {
            let mut world = specs::World::new();
            world.register::<CircuitElement>();
            world.register::<Nodes>();
            world.register::<CalculatedCurrents>();
            world.register::<Resistor>();
            world.register::<Resistance>();
            world.register::<VoltageSource>();
            world.register::<Voltage>();

            specs::Planner::with_num_threads(world, 1)
        };

        // Create a couple of circuit elements
        let (resistor, voltage_source) = {
            let mut world = planner.mut_world();
            let resistor = create_resistor(world);
            let voltage_source = create_voltage_source(world);
            (resistor, voltage_source)
        };

        // Assign node IDs
        {
            let world = planner.mut_world();
            let mut nodes = world.write::<Nodes>().pass();

            match nodes.get_mut(voltage_source) {
                Some(ref mut voltage_source_nodes) => {
                    voltage_source_nodes.ids.clear();
                    voltage_source_nodes.ids.push(0);
                    voltage_source_nodes.ids.push(1);
                }
                None => panic!("oh no"),
            }
            match nodes.get_mut(resistor) {
                Some(ref mut resistor_nodes) => {
                    resistor_nodes.ids.clear();
                    resistor_nodes.ids.push(0);
                    resistor_nodes.ids.push(1);
                }
                None => panic!("oh no"),
            }
        }

        // Run the solver
        planner.add_system(System::new(), "solver", 10);
        planner.dispatch(0.0);
        planner.wait();

        // Assert the circuit elements have the correct state
        let world = planner.mut_world();
        let nodes = world.read::<Nodes>().pass();
        match nodes.get(resistor) {
            Some(ref resistor_nodes) => {
                assert_eq!(resistor_nodes.voltages[0], 0f64);
                assert_eq!(resistor_nodes.voltages[1], 5f64);
            }
            None => panic!("oh no"),
        }
        match nodes.get(voltage_source) {
            Some(ref voltage_source_nodes) => {
                assert_eq!(voltage_source_nodes.voltages[0], 0f64);
                assert_eq!(voltage_source_nodes.voltages[1], 5f64);
            }
            None => panic!("oh no"),
        }
    }
}
