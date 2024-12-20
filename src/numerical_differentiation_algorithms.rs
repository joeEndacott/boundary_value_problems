use crate::grid::GridFunction;

/// # Forwards difference derivative
///
/// Description: calculates the approximate derivative of a grid function using
/// the forwards difference scheme, and returns the derivative as a new grid
/// function.
/// The derivative at the first and final grid points are approximated using a
/// forwards difference and backwards difference scheme respectively.
///
/// Example use case: todo: add example use case.
///
pub fn forward_difference_derivative(
    grid_function: &GridFunction,
) -> GridFunction {
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
/// Description: calculates the approximate derivative of a grid function using
/// the central difference scheme, and returns the derivative as a new grid
/// function.
/// The derivative at the first and final grid points are approximated using a
/// forwards difference and backwards difference scheme respectively.
///
/// Example use case: todo: add example use case.
///
pub fn central_difference_derivative(
    grid_function: &GridFunction,
) -> GridFunction {
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
