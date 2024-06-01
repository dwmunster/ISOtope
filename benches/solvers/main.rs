use isotope::solvers::bfgs_solver::BFGSSolver;
use isotope::solvers::gradient_based_solver::GradientBasedSolver;
use isotope::solvers::Solver;
use std::time::Duration;

mod circle_with_lines;
mod stairs_with_lines;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(
consts = [3, 5, 10, 30, 50, 100, 300],
types=[BFGSSolver, GradientBasedSolver]
)]
fn circles_with_lines<const N: usize, T: Solver>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| (circle_with_lines::new_benchmark(N), T::new()))
        .bench_refs(|(sketch, solver)| {
            solver.solve(sketch).unwrap();
        });
}

#[divan::bench(
    consts = [3, 5, 10, 30, 50, 100, 300],
    types=[BFGSSolver, GradientBasedSolver],
)]
fn stairs_with_lines<const N: usize, T: Solver>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| (stairs_with_lines::new_benchmark(N), T::new()))
        .bench_refs(|(sketch, solver)| {
            solver.solve(sketch).unwrap();
        });
}
