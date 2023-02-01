use ndarray::{Array2};
#[derive(Debug)]
pub struct StateSpace{
    // A space state has A as matrix, B as input matrix, C as output matrix, D as feedforward matrix
    a: Array2<f64>,
    b: Array2<f64>,
    c: Array2<f64>,
    d: Array2<f64>,   
}

impl StateSpace{
    pub fn new(a: Array2<f64>, b: Array2<f64>, c: Array2<f64>, d: Array2<f64>) -> StateSpace{
        StateSpace{a, b, c, d}
    }

    pub fn get_params(&self) -> (Array2<f64>, Array2<f64>, Array2<f64>, Array2<f64>){
        (self.a.clone(), self.b.clone(), self.c.clone(), self.d.clone())
    }
}