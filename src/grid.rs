/// # Grid
///
/// Description: this struct represents a grid of points in 1D. The
/// coordinate of each point corresponds to an element in the vector
/// grid_points.
///
/// Example use case: todo: add example use case.
///
#[derive(Debug, Clone)]
pub struct Grid {
    pub grid_points: Vec<f64>,
}

impl Grid {
    /// # New uniform grid
    ///
    /// Description: creates a uniform grid of num_points points between
    /// start_point and end_point inclusive.
    ///
    /// Example use case: todo: add example use case.
    ///
    pub fn new_uniform_grid(
        start_point: f64,
        end_point: f64,
        num_points: usize,
    ) -> Self {
        // Error handling for when start_point is greater than or equal to end_point or num_points is less than or equal to 1.
        if start_point >= end_point || num_points <= 1 {
            let grid_points: Vec<f64> = vec![start_point];
            return Grid { grid_points };
        }

        // step_size is the distance between adjacent grid points
        let step_size = (end_point - start_point) / (num_points as f64 - 1.0);

        // Creates a vector containing the grid points
        let grid_points: Vec<f64> = (0..num_points)
            .map(|i| start_point + (i as f64) * step_size)
            .collect();

        Grid { grid_points }
    }
}

/// # Grid function
///
/// Description: this struct represents a real-valued function of a real
/// variable sampled on a grid of 1D points. The value of the function at each
/// sampling point corresponds to an element in the vector function_values. The
/// sampling points are contained in the Grid grid.
///
/// Example use case: todo: add example use case.
///
#[derive(Debug, Clone)]
pub struct GridFunction {
    pub grid: Grid,
    pub function_values: Vec<f64>,
}

impl GridFunction {
    /// # New grid function
    ///
    /// Description: this function generates a grid function, given a
    /// real-valued function of a real variable func and a Grid grid.
    ///
    /// Example use case: todo: add example use case.
    ///
    pub fn new_grid_function<F>(grid: &Grid, func: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        // Creates a vector containing the value of func at each grid point.
        let function_values: Vec<f64> =
            grid.grid_points.iter().map(|&x| func(x)).collect();

        GridFunction {
            grid: grid.clone(),
            function_values,
        }
    }

    /// # New constant grid function
    ///
    /// Description: this function generates a constant (flat) grid function
    /// with height scalar, given a Grid grid.
    ///
    /// Example use case: todo: add example use case.
    ///
    pub fn new_constant_grid_function(grid: &Grid, scalar: f64) -> Self {
        let grid = grid.clone();

        let function_values: Vec<f64> =
            grid.grid_points.iter().map(|_| scalar).collect();

        GridFunction {
            grid,
            function_values,
        }
    }

    /// # Grid function addition
    ///
    /// Description: this function sums grid_function_1 and grid_function_2,
    /// and returns the result.
    /// The grid functions should both have the same grid. If they have
    /// different grids, the grid from grid_function_1 will be used.
    ///
    /// Example use case: todo: add example use case.
    ///
    pub fn grid_function_addition(
        grid_function_1: &GridFunction,
        grid_function_2: &GridFunction,
    ) -> Self {
        let grid = grid_function_1.grid.clone();

        let function_values_1 = grid_function_1.function_values.clone();
        let mut function_values_2 = grid_function_2.function_values.clone();

        let length_difference =
            function_values_1.len() - function_values_2.len();

        // If function_values_2 has fewer elements than function_values_1, adds
        // zeroes to the end of function_values_2 until the two vectors are the
        // same length.
        if length_difference > 0 {
            for _ in 0..length_difference {
                function_values_2.push(0.0);
            }
        }

        // Iterates over all the elements in function_values_1 and adds them to
        // the elements in function_values_2.
        let function_values: Vec<f64> = function_values_1
            .iter()
            .zip(function_values_2.iter())
            .map(|(x, y)| x + y)
            .collect();

        GridFunction {
            grid,
            function_values,
        }
    }

    /// # Grid function subtraction
    ///
    /// Description: this function subtracts grid_function_2 from
    /// grid_function_1, and returns the result.
    /// The grid functions should both have the same grid. If they have
    /// different grids, the grid from grid_function_1 will be used.
    ///
    /// Example use case: todo: add example use case.
    ///
    pub fn grid_function_subtraction(
        grid_function_1: &GridFunction,
        grid_function_2: &GridFunction,
    ) -> Self {
        let grid = grid_function_1.grid.clone();

        let function_values_1 = grid_function_1.function_values.clone();
        let mut function_values_2 = grid_function_2.function_values.clone();

        let length_difference =
            function_values_1.len() - function_values_2.len();

        // If function_values_2 has fewer elements than function_values_1, adds
        // zeroes to the end of function_values_2 until the two vectors are the
        // same length.
        if length_difference > 0 {
            for _ in 0..length_difference {
                function_values_2.push(0.0);
            }
        }

        // Iterates over all the elements in function_values_1 and adds them to
        // the elements in function_values_2.
        let function_values: Vec<f64> = function_values_1
            .iter()
            .zip(function_values_2.iter())
            .map(|(x, y)| x - y)
            .collect();

        GridFunction {
            grid,
            function_values,
        }
    }

    /// # Grid function multiplication
    ///
    /// Description: this function takes the product of grid_function_1 and
    /// grid_function_2, and returns the result.
    /// The grid functions should both have the same grid. If they have
    /// different grids, the grid from grid_function_1 will be used.
    ///
    /// Example use case: todo: add example use case.
    ///
    pub fn grid_function_multiplication(
        grid_function_1: &GridFunction,
        grid_function_2: &GridFunction,
    ) -> Self {
        let grid = grid_function_1.grid.clone();

        let function_values_1 = grid_function_1.function_values.clone();
        let mut function_values_2 = grid_function_2.function_values.clone();

        let length_difference =
            function_values_1.len() - function_values_2.len();

        // If function_values_2 has fewer elements than function_values_1, adds
        // zeroes to the end of function_values_2 until the two vectors are the
        // same length.
        if length_difference > 0 {
            for _ in 0..length_difference {
                function_values_2.push(0.0);
            }
        }

        // Iterates over all the elements in function_values_1 and adds them to
        // the elements in function_values_2.
        let function_values: Vec<f64> = function_values_1
            .iter()
            .zip(function_values_2.iter())
            .map(|(x, y)| x * y)
            .collect();

        GridFunction {
            grid,
            function_values,
        }
    }

    /// # Grid function division
    ///
    /// Description: this function divides grid_function_1 by grid_function_2,
    /// and returns the result.
    /// The grid functions should both have the same grid. If they have
    /// different grids, the grid from grid_function_1 will be used.
    ///
    /// Example use case: todo: add example use case.
    ///
    pub fn grid_function_division(
        grid_function_1: &GridFunction,
        grid_function_2: &GridFunction,
    ) -> Self {
        let grid = grid_function_1.grid.clone();

        let function_values_1 = grid_function_1.function_values.clone();
        let mut function_values_2 = grid_function_2.function_values.clone();

        let length_difference =
            function_values_1.len() - function_values_2.len();

        // If function_values_2 has fewer elements than function_values_1, adds
        // zeroes to the end of function_values_2 until the two vectors are the
        // same length.
        if length_difference > 0 {
            for _ in 0..length_difference {
                function_values_2.push(0.0);
            }
        }

        // Iterates over all the elements in function_values_1 and adds them to
        // the elements in function_values_2.
        let function_values: Vec<f64> = function_values_1
            .iter()
            .zip(function_values_2.iter())
            .map(|(x, y)| x / y)
            .collect();

        GridFunction {
            grid,
            function_values,
        }
    }

    /// # Grid function scalar multiplication
    ///
    /// Description: this function multiples grid_function by a scalar,
    /// and returns the result.
    ///
    /// Example use case: todo: add example use case.
    ///
    /// Todo: turn this into a method.
    ///
    pub fn grid_function_scalar_multiplication(
        grid_function: &GridFunction,
        scalar: f64,
    ) -> Self {
        let grid = grid_function.grid.clone();
        let function_values = grid_function.function_values.clone();

        // Iterates over all the elements in function_values multiplies each
        // value by scalar
        let function_values: Vec<f64> =
            function_values.iter().map(|x| scalar * x).collect();

        GridFunction {
            grid,
            function_values,
        }
    }
}
