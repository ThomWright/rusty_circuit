use specs;
use elements::Nodes;
use elements::CalculatedCurrent;
use elements::ground::Ground;
use elements::resistor::Resistance;
use elements::voltage_source::VoltageInput;
use elements::current_source::CurrentInput;
use solver::equation;
use Delta;

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

impl specs::System<Delta> for System {
    fn run(&mut self, arg: specs::RunArg, _: Delta) {
        use specs::Join;
        let (mut nodes, mut calc_currents, resistances, mut v_inputs, c_inputs, grounds) =
            arg.fetch(|w| {
                (w.write::<Nodes>(),
                 w.write::<CalculatedCurrent>(),
                 w.read::<Resistance>(),
                 w.write::<VoltageInput>(),
                 w.read::<CurrentInput>(),
                 w.read::<Ground>())
            });

        // assign all voltage inputs an index
        for (i, (ref mut vi,)) in (&mut v_inputs,).join().enumerate() {
            vi.index = i;
        }

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
