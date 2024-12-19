use crate::grid::Grid;

/// # Grid function
/// This struct represents a function sampled on a grid of points.
/// Todo: improve comments for this struct.
#[derive(Debug)]
pub struct GridFunction {
    pub grid: Grid,
    pub function_values: Vec<f64>,
}

impl GridFunction {
    /// # New grid function
    ///
    /// Details:
    /// Author -> Joe Endacott (https://github.com/joeEndacott).
    /// Date of last modification -> 19/12/2024.
    ///
    /// Description:
    /// Generates a grid function, given a function func and a Grid grid.
    ///
    /// Inputs:
    /// func -> func takes in a real number, x, and returns a real number func(x).
    /// grid -> Grid on which func is to be sampled.
    ///
    /// Output: GridFunction { grid, function_values } -> returns grid, and the value of func evaluated at each point in grid.grid_points.
    ///
    pub fn new_grid_function<F>(grid: Grid, func: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        // Creates a vector containing the value of func at each grid point.
        let function_values: Vec<f64> =
            grid.grid_points.iter().map(|&x| func(x)).collect();

        GridFunction {
            grid,
            function_values,
        }
    }
}
