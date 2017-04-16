use std;
use std::fmt;
use rulinalg;
use rulinalg::matrix::Matrix;
use rulinalg::matrix::decomposition::PartialPivLu;
use rulinalg::vector::Vector;

#[derive(Debug)]
pub struct Solution {
    voltages: Vec<f64>,
    currents: Vec<f64>,
}

impl Solution {
    pub fn voltages(&self) -> &Vec<f64> {
        &self.voltages
    }
    pub fn currents(&self) -> &Vec<f64> {
        &self.currents
    }
}

#[derive(Debug, Clone)]
pub struct Equation {
    nodal_admittances: Matrix<f64>,
    inputs: Vector<f64>,

    nodes: usize,
    voltage_sources: usize,
    voltage_sources_stamped: usize,
}

impl Equation {
    pub fn new(nodes: usize, voltage_sources: usize) -> Self {
        let size = nodes + voltage_sources - 1;
        Equation {
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

    fn solve_internal(eq: Equation) -> Result<Solution, Error> {
        if eq.voltage_sources != eq.voltage_sources_stamped {
            return Err(Error::IncorrectNumberOfVoltageSources(
                format!("Expected {} voltage sources, stamped {}",
                               eq.voltage_sources,
                               eq.voltage_sources_stamped).to_owned()));
        }

        let lu = PartialPivLu::decompose(eq.nodal_admittances)?;
        let solution = lu.solve(eq.inputs)?;

        let (voltages, currents) = solution.data()
            .as_slice()
            .split_at(eq.nodes - 1);

        let mut vs = voltages.to_vec();
        vs.insert(0, 0.0); // ground node

        Ok(Solution {
            voltages: vs,
            currents: currents.to_vec(),
        })
    }

    pub fn solve(&self) -> Result<Solution, Error> {
        Equation::solve_internal(self.clone())
    }
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "
Number of nodes: {}
Expected number of voltage sources: {}
Voltage sources stamped: {}
Nodal admittances:
{}
Inputs:
{}",
               self.voltage_sources,
               self.voltage_sources_stamped,
               self.nodes,
               self.nodal_admittances,
               self.inputs)
    }
}

#[derive(Debug)]
pub enum Error {
    IncorrectNumberOfVoltageSources(String),
    Unsolvable(rulinalg::error::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Unsolvable(ref err) => write!(f, "{}", err),
            Error::IncorrectNumberOfVoltageSources(ref s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Unsolvable(ref err) => err.description(),
            Error::IncorrectNumberOfVoltageSources(ref s) => s,
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::Unsolvable(ref err) => Some(err),
            Error::IncorrectNumberOfVoltageSources(_) => None,
        }
    }
}

impl From<rulinalg::error::Error> for Error {
    fn from(err: rulinalg::error::Error) -> Error {
        Error::Unsolvable(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use test::Bencher;

    #[test]
    fn stamp_resistor() {
        let mut equation = Equation::new(3, 0);
        equation.stamp_resistor(5.0, 1, 2);

        let expected = matrix![0.2, -0.2;
                               -0.2, 0.2];

        assert_matrix_eq!(equation.nodal_admittances, expected);
    }

    #[test]
    fn stamp_two_resistors() {
        let mut equation = Equation::new(3, 0);
        equation.stamp_resistor(5.0, 1, 2);
        equation.stamp_resistor(5.0, 0, 2);

        let expected = matrix![0.2, -0.2;
                               -0.2, 0.4];

        assert_matrix_eq!(equation.nodal_admittances, expected);
    }

    #[test]
    fn stamp_voltage_source() {
        let mut equation = Equation::new(3, 1);
        equation.stamp_voltage_source(5.0, 1, 2, 0);

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
        let mut equation = Equation::new(3, 0);
        equation.stamp_current_source(5.0, 1, 2);

        let expected_inputs = vector![-5.0, 5.0];
        assert_vector_eq!(equation.inputs, expected_inputs);
    }

    #[test]
    fn stamp_too_many_voltage_sources() {
        let mut equation = Equation::new(3, 0);
        equation.stamp_voltage_source(5.0, 1, 2, 0);

        let solution = equation.solve();

        assert!(solution.is_err());
    }

    #[test]
    fn solve_simple_circuit() {
        let mut equation = Equation::new(2, 0);
        equation.stamp_current_source(1.0, 0, 1);
        equation.stamp_resistor(100.0, 1, 0);

        let solution = equation.solve().unwrap();

        let expected_voltages = vec![0.0, 100.0];
        assert_eq!(solution.voltages(), &expected_voltages);
    }

    #[test]
    fn solve_simple_circuit_with_voltage_source() {
        let mut equation = Equation::new(2, 1);
        equation.stamp_voltage_source(10.0, 0, 1, 0);
        equation.stamp_resistor(10.0, 1, 0);

        let solution = equation.solve().unwrap();

        let expected_voltages = vec![0.0, 10.0];
        let expected_currents = vec![1.0];
        assert_eq!(solution.voltages(), &expected_voltages);
        assert_eq!(solution.currents(), &expected_currents);
    }

    #[test]
    fn solve_simple_circuit_with_wire() {
        let mut equation = Equation::new(3, 1);
        equation.stamp_current_source(1.0, 0, 1);
        equation.stamp_voltage_source(0.0, 1, 2, 0);
        equation.stamp_resistor(100.0, 2, 0);

        let solution = equation.solve().unwrap();

        let expected_voltages = vec![0.0, 100.0, 100.0];
        let expected_currents = vec![1.0];
        assert_eq!(solution.voltages(), &expected_voltages);
        assert_eq!(solution.currents(), &expected_currents);
    }

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        b.iter(|| solve_simple_circuit_with_voltage_source());
    }
}
