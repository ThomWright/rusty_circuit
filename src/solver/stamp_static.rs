use specs;
use elements::Type;
use elements::CircuitElement;
use elements::Nodes;
use elements::voltage_source::VoltageInput;
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
            vi.0 = i;
        }
    }

    let entities = world.entities().pass();
    let elements = world.read::<CircuitElement>().pass();
    let nodes_ticket = world.read::<Nodes>().pass();
    let v_inputs = world.read::<VoltageInput>().pass();

    let mut equation = {
        let num_nodes: usize = match (&nodes_ticket,)
            .join()
            .flat_map(|(&Nodes(ref ns),)| ns.iter())
            .max_by(|n1, n2| n1.index.cmp(&n2.index)) {
            Some(node) => node.index + 1,
            None => 0,
        };
        let num_v_inputs: usize = (&v_inputs,).join().count();

        equation::Equation::new(num_nodes, num_v_inputs)
    };

    for (entity, &elem, nodes) in (&entities, &elements, &nodes_ticket).join() {
        let &Nodes(ref ns) = nodes;
        match elem.typ() {
            Type::CurrentSource(current) => {
                equation.stamp_current_source(current, ns[0].index, ns[1].index);
            }
            Type::Ground => {
                let vi = v_inputs.get(entity).unwrap();
                equation.stamp_voltage_source(0f64, 0, ns[0].index, vi.0);
            }
            Type::Resistor(resistance) => {
                equation.stamp_resistor(resistance, ns[0].index, ns[1].index);
            }
            Type::VoltageSource(voltage) => {
                let vi = v_inputs.get(entity).unwrap();
                equation.stamp_voltage_source(voltage, ns[0].index, ns[1].index, vi.0);
            }
        }
    }

    equation
}
