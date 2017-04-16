use specs;
use elements::Nodes;
use elements::ground::Ground;
use elements::resistor::Resistance;
use elements::voltage_source::VoltageInput;
use elements::current_source::CurrentInput;
use solver::equation;

// Create an equation builder with all static parts of the circuit stamped.
//
// Elements that change over time, (e.g. sine wave sources, capacitors), or need linearization
// (e.g. diodes) don't stamp here.
//
// Should be called whenever the circuit is modified.
//
// This is not a specs::System, because it runs before the 'update' event loop cycle, and has no
// dependency on Delta.
pub fn create_static_equation(world: &mut specs::World) -> equation::Equation {
    use specs::Join;
    use specs::Gate;

    // assign all voltage inputs an index
    {
        let mut v_inputs = world.write::<VoltageInput>().pass();
        for (i, (ref mut vi,)) in (&mut v_inputs,).join().enumerate() {
            vi.index = i;
        }
    }

    let nodes = world.read::<Nodes>().pass();
    let resistances = world.read::<Resistance>().pass();
    let v_inputs = world.read::<VoltageInput>().pass();
    let c_inputs = world.read::<CurrentInput>().pass();
    let grounds = world.read::<Ground>().pass();

    let mut equation_builder = {
        let num_nodes: usize = match (&nodes,)
            .join()
            .flat_map(|(&Nodes(ref ns),)| ns.iter())
            .max_by(|n1, n2| n1.index.cmp(&n2.index)) {
            Some(node) => node.index + 1,
            None => 0,
        };
        let num_v_inputs: usize = (&v_inputs,).join().count();

        equation::Equation::new(num_nodes, num_v_inputs)
    };

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

    equation_builder
}
