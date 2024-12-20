pub mod grid;
pub mod numerical_differentiation_algorithms;
pub mod test_functions;

use grid::{Grid, GridFunction};
use numerical_differentiation_algorithms::{
    central_difference_derivative, forward_difference_derivative,
};
use test_functions::quadratic;

fn main() {
    let grid = Grid::new_uniform_grid(0.0, 0.1, 11);

    let grid_function = GridFunction::new_grid_function(grid, quadratic);

    let forward_difference_derivative =
        forward_difference_derivative(&grid_function);
    let central_difference_derivative =
        central_difference_derivative(&grid_function);

    println!("{:?}", grid_function.grid.grid_points);
    println!("{:?}", grid_function.function_values);
    println!("{:?}", forward_difference_derivative.function_values);
    println!("{:?}", central_difference_derivative.function_values);
}
