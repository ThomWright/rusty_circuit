
use rulinalg::matrix::Matrix;
use rulinalg::matrix::decomposition::PartialPivLu;
use rulinalg::vector::Vector;

struct Equation {
    nodalAdmittances: Matrix<f64>,
    inputs: Vector<f64>,
}

impl Equation {
    fn solve(self) -> Vector<f64> {
        let lu = PartialPivLu::decompose(self.nodalAdmittances)
            .expect("Matrix should be invertible");
        lu.solve(self.inputs).expect("Matrix should be well-conditioned")
    }
}
