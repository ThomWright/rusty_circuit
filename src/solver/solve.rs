use specs;
use elements::Nodes;
use elements::CalculatedCurrent;
use elements::voltage_source::VoltageInput;
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
        // currently all this does it solve equation created for a static circuit
        // in future, it will solve dynamic and non-linear circuits
        update_circuit_state(&arg);
    }
}

// Solve the circuit equation, and update all circuit elements with their calculated state.
fn update_circuit_state(arg: &specs::RunArg) {
    use specs::Join;

    let (mut nodes, mut calc_currents, v_inputs, equation) = arg.fetch(|w| {
        (w.write::<Nodes>(),
         w.write::<CalculatedCurrent>(),
         w.read::<VoltageInput>(),
         w.read_resource::<equation::Equation>())
    });

    match equation.solve() {
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
