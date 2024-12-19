/// # Grid
/// This struct represents a grid of points in 1D.
/// Todo: improve comments for this
#[derive(Debug)]
pub struct Grid {
    pub grid_points: Vec<f64>,
}

impl Grid {
    /// # New uniform grid
    ///
    /// Details:
    /// Author -> Joe Endacott (https://github.com/joeEndacott).
    /// Date of last modification -> 19/12/2024.
    ///
    /// Description:
    /// Creates a uniform grid of num_points points between start_point and end_point inclusive.
    ///
    /// Inputs:
    /// start_point -> coordinate of the first grid point.
    /// end_point -> coordinate of the final grid point.
    /// num_points -> number of grid points
    ///
    /// Output: Grid { grid_points } -> uniform grid of points, represented using the Grid struct.
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
