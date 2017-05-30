use specs;
use elements::Nodes;
use elements::CalculatedCurrent;
use elements::DerivedCurrent;
use elements::voltage_source::VoltageSource;
use elements::capacitor::Capacitor;
use solver::equation;
use Delta;

// Run the simulation 1000x slower than reality
pub const SIM_TIME_PER_SEC: f64 = 1.0 / 1000.0;
pub const SIM_TIMESTEP: f64 = 5e-6; // 5µs (seconds)

#[derive(Debug, Clone, Copy)]
pub struct System {
    prev_unsimulated_time: f64,
    sim_time: f64,
}

impl System {
    pub fn new() -> Self {
        System {
            prev_unsimulated_time: 0f64,
            sim_time: 0f64,
        }
    }
}

impl Default for System {
    fn default() -> Self {
        System::new()
    }
}

impl specs::System<Delta> for System {
    fn run(&mut self, arg: specs::RunArg, delta: Delta) {
        use specs::Join;

        let (mut nodes_ticket,
             mut calc_currents,
             mut derived_currents,
             v_inputs,
             mut capacitors,
             static_equation) = arg.fetch(|w| {
            (w.write::<Nodes>(),
             w.write::<CalculatedCurrent>(),
             w.write::<DerivedCurrent>(),
             w.read::<VoltageSource>(),
             w.write::<Capacitor>(),
             w.read_resource::<equation::Equation>())
        });

        // convert into the slower circuit time
        // TODO override SIM_TIME_PER_SEC
        let sim_delta = (delta * SIM_TIME_PER_SEC) + self.prev_unsimulated_time;
        let mut time_to_simulate = sim_delta;

        if time_to_simulate < SIM_TIMESTEP {
            return;
        }

        while time_to_simulate > SIM_TIMESTEP {
            self.sim_time += SIM_TIMESTEP;
            time_to_simulate -= SIM_TIMESTEP;

            let mut equation = static_equation.clone();

            // Capacitors
            for (nodes, prev_current, mut cap) in
                (&nodes_ticket, &derived_currents, &mut capacitors).join() {

                let conductance = (2.0 * cap.capacitance) / SIM_TIMESTEP;
                cap.resistor.set_resistance(1.0 / conductance);

                let &Nodes(ref ns) = nodes;
                let n0 = ns[cap.node_indexes.0];
                let n1 = ns[cap.node_indexes.1];
                let previous_voltage = n0.voltage - n1.voltage;
                let current = prev_current.0 + (conductance * previous_voltage);
                cap.current_source.current = current;

                equation.stamp_conductance(cap.resistor.conductance(), n0.index, n1.index);
                equation.stamp_current_source(cap.current_source.current, n1.index, n0.index);
            }

            // Solve the circuit equation, and update all circuit elements with their
            // calculated state.
            match equation.solve() {
                Ok(solution) => {
                    let voltages = solution.voltages();
                    let currents = solution.currents();

                    // update circuit element states
                    for (nodes,) in (&mut nodes_ticket,).join() {
                        let &mut Nodes(ref mut ns) = nodes;
                        for ref mut node in ns.iter_mut() {
                            node.voltage = voltages[node.index];
                        }
                    }
                    for (v_input, calc_current) in (&v_inputs, &mut calc_currents).join() {
                        let &mut CalculatedCurrent(ref mut current) = calc_current;
                        *current = currents[v_input.index];
                    }

                    // update any derived state
                    for (nodes, mut current, capacitor) in
                        (&nodes_ticket, &mut derived_currents, &capacitors).join() {
                        let &Nodes(ref ns) = nodes;
                        let n0 = ns[capacitor.node_indexes.0];
                        let n1 = ns[capacitor.node_indexes.1];

                        let resistor_current = (n0.voltage - n1.voltage) *
                                               capacitor.resistor.conductance();

                        current.0 = resistor_current - capacitor.current_source.current;
                    }
                }
                Err(error) => println!("Unsolvable circuit: {}", error),
            }
        }

        // for next time
        self.prev_unsimulated_time = time_to_simulate;
    }
}
