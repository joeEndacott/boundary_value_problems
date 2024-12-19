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

    /// # Forward difference derivative
    ///
    /// Details:
    /// Author -> Joe Endacott (https://github.com/joeEndacott).
    /// Date of last modification -> 19/12/2024.
    ///
    /// Description:
    /// Calculates the derivative of a grid function using the forward difference scheme, and returns this derivative as a grid function.
    ///
    /// Inputs:
    /// grid_function -> grid function to be differentiated.
    ///
    /// Output: GridFunction { grid, function_values } -> returns grid, and the value of the first derivative evaluated at each point in grid.grid_points.
    ///
    pub fn forward_difference_derivative(grid_function: &GridFunction) -> Self {
        let grid = &grid_function.grid;
        let function_values = &grid_function.function_values;

        let grid_points = &grid.grid_points;
        let num_points = grid_points.len();

        let mut first_derivative_values = Vec::new();

        // Calculates the derivative at the starting grid point using the forwards difference scheme, and adds the value to first_derivative_values.
        first_derivative_values.push(
            (function_values[1] - function_values[0])
                / (grid_points[1] - grid_points[0]),
        );

        // Calculates the derivative at each interior grid point using the forwards difference scheme, and adds each value to first_derivative_values.
        for i in 1..(num_points - 1) {
            first_derivative_values.push(
                (function_values[i + 1] - function_values[i])
                    / (grid_points[i + 1] - grid_points[i]),
            );
        }

        // Calculates the derivative at the final grid point using the backwards difference scheme, and adds the value to first_derivative_values.
        first_derivative_values.push(
            (function_values[num_points - 1] - function_values[num_points - 2])
                / (grid_points[num_points - 1] - grid_points[num_points - 2]),
        );

        GridFunction {
            grid: grid.clone(),
            function_values: first_derivative_values,
        }
    }

    /// # Central difference derivative
    ///
    /// Details:
    /// Author -> Joe Endacott (https://github.com/joeEndacott).
    /// Date of last modification -> 19/12/2024.
    ///
    /// Description:
    /// Calculates the derivative of a grid function using the central difference scheme, and returns this derivative as a grid function.
    ///
    /// Inputs:
    /// grid_function -> grid function to be differentiated.
    ///
    /// Output: GridFunction { grid, function_values } -> returns grid, and the value of the first derivative evaluated at each point in grid.grid_points.
    ///
    /// To do: improve calculation of derivatives at the start and end points of the domain.
    ///
    pub fn central_difference_derivative(grid_function: &GridFunction) -> Self {
        let grid = &grid_function.grid;
        let function_values = &grid_function.function_values;

        let grid_points = &grid.grid_points;
        let num_points = grid_points.len();

        let mut first_derivative_values = Vec::new();

        // Calculates the derivative at the starting grid point using the forwards difference scheme, and adds the value to first_derivative_values.
        first_derivative_values.push(
            (function_values[1] - function_values[0])
                / (grid_points[1] - grid_points[0]),
        );

        // Calculates the derivative at each interior grid point using the central difference scheme, and adds each value to first_derivative_values.
        for i in 1..(num_points - 1) {
            first_derivative_values.push(
                (function_values[i + 1] - function_values[i - 1])
                    / (grid_points[i + 1] - grid_points[i - 1]),
            );
        }

        // Calculates the derivative at the final grid point using the backwards difference scheme, and adds the value to first_derivative_values.
        first_derivative_values.push(
            (function_values[num_points - 1] - function_values[num_points - 2])
                / (grid_points[num_points - 1] - grid_points[num_points - 2]),
        );

        GridFunction {
            grid: grid.clone(),
            function_values: first_derivative_values,
        }
    }
}
