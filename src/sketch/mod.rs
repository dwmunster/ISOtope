pub mod primitives;
pub mod constraints;
pub mod point2;

use std::collections::VecDeque;


use self::{constraints::Constraints, primitives::Parametric};

pub struct Sketch<'a, const N: usize> {
    data: [f64; N],
    gradient: [f64; N],

    n: usize,

    constraints: VecDeque<Constraints<'a>>,
}

impl<'a, const N: usize> Sketch<'a, N> {
    pub fn new() -> Self {
        Self {
            data: [0.0; N],
            gradient: [0.0; N],
            n: 0,
            constraints: VecDeque::new(),
        }
    }

    pub fn add_primitive<P: Parametric<'a>>(&'a mut self) -> P {
        let data = &mut self.data[self.n..self.n + P::num_parameters()];
        let gradient = &mut self.gradient.as_mut_slice()[self.n..self.n + P::num_parameters()];
        let primitive = P::initialize(data, gradient);
        self.n += P::num_parameters();
        primitive
    }

    pub fn add_constraint(&mut self, constraint: Constraints<'a>) {
        self.constraints.push_back(constraint);
    }

    pub fn step(&'a mut self, step_size: f64) {
        for i in 0..N {
            self.gradient[i] = 0.0;
        }

        for constraint in self.constraints.iter_mut() {
            constraint.constraint_mut().update_gradient();
        }

        for i in 0..N {
            self.data[i] -= step_size * self.gradient[i];
        }
    }
}

