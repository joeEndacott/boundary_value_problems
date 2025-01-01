pub mod bvp_algorithms;
pub mod grid;
pub mod numerical_differentiation_algorithms;

use bvp_algorithms::BoundaryConditions;
use grid::GridFunction;
use numerical_differentiation_algorithms::central_difference_derivative as derivative;
pub const PI: f64 = std::f64::consts::PI;

fn main() {
    let boundary_conditions = BoundaryConditions::new_dirichlet_bcs(0.0, 0.0);

    // Creates a uniform grid from 0 to PI with num_grid_points grid points.
    let num_grid_points = 20;
    let grid = grid::Grid::new_uniform_grid(0.0, PI, num_grid_points);

    // True solution to the BVP.
    let grid_func_true_solution =
        GridFunction::new_grid_function(&grid, f64::sin);

    println!(
        "True solution:        {:?}",
        grid_func_true_solution
            .function_values
            .iter()
            .map(|x| format!("{:.2}", x))
            .collect::<Vec<_>>()
    );

    // Initial guess of the solution to the BVP.
    let grid_func_initial_guess =
        GridFunction::new_grid_function(&grid, initial_guess);

    println!(
        "Initial guess:        {:?}",
        grid_func_initial_guess
            .function_values
            .iter()
            .map(|x| format!("{:.2}", x))
            .collect::<Vec<_>>()
    );

    // Solves the BVP using Newton's method.
    let grid_func_approximate_solution = bvp_algorithms::newtons_method(
        differential_equation_function,
        &boundary_conditions,
        &grid_func_initial_guess,
        20,
    );

    println!(
        "Approximate solution: {:?}",
        grid_func_approximate_solution
            .function_values
            .iter()
            .map(|x| format!("{:.2}", x))
            .collect::<Vec<_>>()
    );
}

fn initial_guess(x: f64) -> f64 {
    x * (PI - x)
}

/// # Differential equation function
///
/// ## Description
/// We can write a general second order differential equation as
/// f(y, y', y'') = 0. We will refer to f as the "differential equation (DE)
/// function".
/// differential_equation_function takes an arbitrary GridFunction grid_func as
/// an input, and returns the DE function associated with grid_func.
/// If the input to differential_equation_function is the true solution to the
/// DE, the output will be a constant zero function.
///
/// ## Example use case
/// Suppose we want to solve the harmonic oscillator equation, y'' + y = 0.
/// First, we edit the body of differential_equation_function so that it
/// returns y'' + y.
/// ```
/// pub fn differential_equation_function(
///     grid_func: &GridFunction,
/// ) -> GridFunction {
///     GridFunction::grid_function_addition(
///         &derivative(&derivative(grid_func)),
///         &grid_func,
///     )
/// }
/// ```
/// We know that sin(x) is a solution to the DE y'' + y = 0. We can use
/// differential_equation_function to check this.
/// ```
/// let grid = Grid::new_uniform_grid(0.0, 1.0, 10);
/// let sin_func = GridFunction::new_grid_function(&grid, f64::sin);
/// let de_func = differential_equation(&sin_func);
/// ```
/// Finally, we can plot DE_func and a constant zero function on the same plot
/// to confirm that sin(x) is a solution to the DE.
///
/// ## Todo
/// Currently, to change what DE you are solving, you have to manually edit the
/// body of differential_equation_function. I would like to make it more user
/// friendly to change what DE you are solving, and move
/// differential_equation_function into the bvp_algorithms.rs file.
///
pub fn differential_equation_function(
    grid_func: &GridFunction,
) -> GridFunction {
    GridFunction::grid_function_addition(
        &derivative(&derivative(grid_func)),
        &grid_func,
    )
}
