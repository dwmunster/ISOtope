use std::error::Error;

use crate::sketch::Sketch;
use crate::solvers::line_search::line_search_wolfe;

use super::Solver;

pub struct GradientBasedSolver {
    max_iterations: usize,
    min_loss: f64,
    min_grad: f64,
}

impl GradientBasedSolver {
    pub fn new() -> Self {
        Self {
            max_iterations: 200000,
            min_loss: 1e-14,
            min_grad: 1e-10,
        }
    }

    pub fn new_with_params(max_iterations: usize, min_loss: f64, min_grad: f64) -> Self {
        Self {
            max_iterations,
            min_loss,
            min_grad,
        }
    }
}

impl Solver for GradientBasedSolver {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self::new()
    }
    fn solve(&self, sketch: &mut Sketch) -> Result<(), Box<dyn Error>> {
        let mut iterations = 0;

        let mut gradient = sketch.get_gradient();
        let mut grad_norm = gradient.norm();
        let mut loss = sketch.get_loss();

        let mut direction = gradient.clone();
        while iterations < self.max_iterations {
            if grad_norm < self.min_grad {
                break;
            }
            if loss < self.min_loss {
                break;
            }
            let mut data = sketch.get_data();

            // Direction = -gradient + 0.0 * direction
            direction.axpy(-1.0, &gradient, 0.0);

            let alpha = line_search_wolfe(sketch, &direction, &gradient)?;
            // data = data + alpha * direction
            data.axpy(alpha, &direction, 1.0);
            sketch.set_data(data);

            // Update metrics
            loss = sketch.get_loss();
            gradient = sketch.get_gradient();
            grad_norm = gradient.norm();

            iterations += 1;
        }

        Ok(())
    }
}
