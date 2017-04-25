use specs;
use elements::Nodes;
use elements::resistor::Resistor;
use elements::current_source::CurrentSource;
use elements::voltage_source::VoltageSource;
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
        let mut v_sources = world.write::<VoltageSource>().pass();
        for (i, (ref mut vi,)) in (&mut v_sources,).join().enumerate() {
            vi.index = i;
        }
    }

    let nodes_ticket = world.read::<Nodes>().pass();
    let v_sources = world.read::<VoltageSource>().pass();
    let c_sources = world.read::<CurrentSource>().pass();
    let resistors = world.read::<Resistor>().pass();

    let mut equation = {
        let num_nodes: usize = match (&nodes_ticket,)
            .join()
            .flat_map(|(&Nodes(ref ns),)| ns.iter())
            .max_by(|n1, n2| n1.index.cmp(&n2.index)) {
            Some(node) => node.index + 1,
            None => 0,
        };
        let num_v_sources: usize = (&v_sources,).join().count();

        equation::Equation::new(num_nodes, num_v_sources)
    };

    // Current sources
    for (nodes, ci) in (&nodes_ticket, &c_sources).join() {
        let &Nodes(ref ns) = nodes;

        equation.stamp_current_source(ci.current,
                                      ns[ci.node_index_from()].index,
                                      ns[ci.node_index_to()].index);
    }

    // Voltage sources
    for (nodes, vi) in (&nodes_ticket, &v_sources).join() {
        let &Nodes(ref ns) = nodes;

        equation.stamp_voltage_source(vi.voltage,
                                      ns[vi.node_index_from()].index,
                                      ns[vi.node_index_to()].index,
                                      vi.index);
    }

    // Resistors
    for (nodes, res) in (&nodes_ticket, &resistors).join() {
        let &Nodes(ref ns) = nodes;

        equation.stamp_resistor(res.resistance,
                                ns[res.node_indexes.0].index,
                                ns[res.node_indexes.1].index);
    }

    equation
}
