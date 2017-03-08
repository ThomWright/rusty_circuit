
use rulinalg::matrix::Matrix;
use rulinalg::matrix::decomposition::PartialPivLu;
use rulinalg::vector::Vector;

pub struct Equation {
    nodal_admittances: Matrix<f64>,
    inputs: Vector<f64>,
}

impl Equation {
    pub fn solve(self) -> Vector<f64> {
        let lu = PartialPivLu::decompose(self.nodal_admittances)
            .expect("Matrix should be invertible");
        lu.solve(self.inputs).expect("Matrix should be well-conditioned")
    }
}

pub struct EquationBuilder {
    nodal_admittances: Matrix<f64>,
    inputs: Vector<f64>,

    nodes: usize,
    voltage_sources: usize,
    voltage_sources_stamped: usize,
}

impl EquationBuilder {
    pub fn new(nodes: usize, voltage_sources: usize) -> Self {
        let size = nodes + voltage_sources - 1;
        EquationBuilder {
            nodal_admittances: Matrix::<f64>::zeros(size, size),
            inputs: Vector::<f64>::zeros(size),

            nodes: nodes,
            voltage_sources: voltage_sources,
            voltage_sources_stamped: 0,
        }
    }

    fn stamp_nodal_admittance(&mut self, row: usize, col: usize, x: f64) {
        if row != 0 && col != 0 {
            // ignore ground node
            let row = row - 1;
            let col = col - 1;
            self.nodal_admittances[[row, col]] += x;
        }
    }

    fn stamp_input(&mut self, row: usize, x: f64) {
        if row != 0 {
            let row = row - 1;
            self.inputs[row] += x;
        }
    }

    pub fn stamp_conductance(&mut self, conductance: f64, node1: usize, node2: usize) -> &mut Self {
        self.stamp_nodal_admittance(node1, node1, conductance);
        self.stamp_nodal_admittance(node2, node2, conductance);
        self.stamp_nodal_admittance(node1, node2, -conductance);
        self.stamp_nodal_admittance(node2, node1, -conductance);
        self
    }

    pub fn stamp_resistor(&mut self, resistance: f64, node1: usize, node2: usize) -> &mut Self {
        let conductance = 1.0 / resistance;
        self.stamp_conductance(conductance, node1, node2);
        self
    }

    pub fn stamp_voltage_source(&mut self,
                                voltage: f64,
                                from_node: usize,
                                to_node: usize,
                                v_num: usize)
                                -> &mut Self {
        self.voltage_sources_stamped += 1;
        if self.voltage_sources_stamped > self.voltage_sources {
            return self;
        }

        let v_index = self.nodes + v_num;
        self.stamp_nodal_admittance(v_index, from_node, -1.0);
        self.stamp_nodal_admittance(v_index, to_node, 1.0);
        self.stamp_nodal_admittance(from_node, v_index, 1.0);
        self.stamp_nodal_admittance(to_node, v_index, -1.0);
        self.stamp_input(v_index, voltage);
        self
    }

    pub fn stamp_current_source(&mut self,
                                current: f64,
                                from_node: usize,
                                to_node: usize)
                                -> &mut Self {
        self.stamp_input(from_node, -current);
        self.stamp_input(to_node, current);
        self
    }

    pub fn build(self) -> Result<Equation, String> {
        if self.voltage_sources != self.voltage_sources_stamped {
            return Err(format!("Expected {} voltage sources, stamped {}",
                               self.voltage_sources,
                               self.voltage_sources_stamped));
        }
        Ok(Equation {
            nodal_admittances: self.nodal_admittances,
            inputs: self.inputs,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stamp_resistor() {
        let mut builder = EquationBuilder::new(3, 0);
        builder.stamp_resistor(5.0, 1, 2);

        let equation = builder.build().unwrap();

        let expected = matrix![0.2, -0.2;
                               -0.2, 0.2];

        assert_matrix_eq!(equation.nodal_admittances, expected);
    }

    #[test]
    fn stamp_two_resistors() {
        let mut builder = EquationBuilder::new(3, 0);
        builder.stamp_resistor(5.0, 1, 2);
        builder.stamp_resistor(5.0, 0, 2);

        let equation = builder.build().unwrap();

        let expected = matrix![0.2, -0.2;
                               -0.2, 0.4];

        assert_matrix_eq!(equation.nodal_admittances, expected);
    }

    #[test]
    fn stamp_voltage_source() {
        let mut builder = EquationBuilder::new(3, 1);
        builder.stamp_voltage_source(5.0, 1, 2, 0);

        let equation = builder.build().unwrap();

        let expected_inputs = vector![0.0, 0.0, 5.0];
        assert_vector_eq!(equation.inputs, expected_inputs);

        let expected_admittances = matrix![0.0, 0.0, 1.0;
                                           0.0, 0.0, -1.0;
                                           -1.0, 1.0, 0.0];
        assert_matrix_eq!(equation.nodal_admittances,
                          expected_admittances,
                          comp = float);
    }

    #[test]
    fn stamp_current_source() {
        let mut builder = EquationBuilder::new(3, 0);
        builder.stamp_current_source(5.0, 1, 2);

        let equation = builder.build().unwrap();

        let expected_inputs = vector![-5.0, 5.0];
        assert_vector_eq!(equation.inputs, expected_inputs)
    }

    #[test]
    fn stamp_too_many_voltage_sources() {
        let mut builder = EquationBuilder::new(3, 0);
        builder.stamp_voltage_source(5.0, 1, 2, 0);

        let builder_result = builder.build();

        assert!(builder_result.is_err())
    }

    #[test]
    fn solve_simple_circuit() {
        let mut builder = EquationBuilder::new(2, 0);
        builder.stamp_current_source(1.0, 0, 1);
        builder.stamp_resistor(100.0, 1, 0);

        let equation = builder.build().unwrap();

        let solution = equation.solve();

        let expected_solution = vector![100.0];
        assert_vector_eq!(solution, expected_solution);
    }

    #[test]
    fn solve_simple_circuit_with_voltage_source() {
        let mut builder = EquationBuilder::new(2, 1);
        builder.stamp_voltage_source(10.0, 0, 1, 0);
        builder.stamp_resistor(10.0, 1, 0);

        let equation = builder.build().unwrap();

        let solution = equation.solve();

        let expected_solution = vector![10.0, 1.0];
        assert_vector_eq!(solution, expected_solution);
    }

    #[test]
    fn solve_simple_circuit_with_wire() {
        let mut builder = EquationBuilder::new(3, 1);
        builder.stamp_current_source(1.0, 0, 1);
        builder.stamp_voltage_source(0.0, 1, 2, 0);
        builder.stamp_resistor(100.0, 2, 0);

        let equation = builder.build().unwrap();

        let solution = equation.solve();

        let expected_solution = vector![100.0, 100.0, 1.0];
        assert_vector_eq!(solution, expected_solution);
    }
}
