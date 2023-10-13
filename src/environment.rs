use crate::grid::Grid;

pub struct Environment<'a> {
    grid: &'a Grid,
}

impl Environment<'_> {
    pub fn new<'a>(grid: &'a Grid) -> Environment {
        Environment { grid }
    }
    
    pub fn grid(&self) -> &Grid {
        self.grid
    }
}
