pub mod grid;
pub mod grid_function;
pub mod test_functions;

use grid::Grid;
use grid_function::GridFunction;
use test_functions::quadratic;

fn main() {
    let grid: Grid = Grid::new_uniform_grid(0.0, 10.0, 11);

    let grid_function: GridFunction =
        GridFunction::new_grid_function(grid, quadratic);

    println!("{:?}", grid_function.grid.grid_points);
    println!("{:?}", grid_function.function_values);
}
