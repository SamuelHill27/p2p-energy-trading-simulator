use super::environment::Environment;
use super::grid::Grid;

pub struct World {
    grid: Grid,
    environment: Environment
}

impl World {
    pub fn new(grid: Grid, environment: Environment) -> World {
        World {
            grid,
            environment
        }
    }

    pub fn environment(&self) -> &Environment {
        &self.environment
    }

    pub fn environment_mut(&mut self) -> &mut Environment {
        &mut self.environment
    }

    pub fn grid(&self) -> &Grid {
        &self.grid
    }
}