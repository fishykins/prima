
use glam::Vec2;

use crate::geom::Coord;

/// A grid based on a 2D coordinate system.
#[derive(Clone)]
pub struct Grid<T> {
    /// The number of cells in the x plane
    width: u32,
    /// The number of cells in the y plane
    height: u32,
    /// The size of each cell (which is assumed to be a square)
    pub cell_size: f32,
    /// World anchor point.
    pub anchor: Vec2,
    /// Cell data
    pub cells: Vec<Option<T>>,
}

impl<T> Grid<T> where T: Clone {
    /// Makes a new grid.
    pub fn new(width: u32, height: u32, cell_size: f32, anchor: Vec2) -> Grid<T> {
        let mut grid = Grid {
            width,
            height,
            cell_size,
            anchor,
            cells: Vec::with_capacity((width * height) as usize),
        };

        grid.cells.resize((width * height) as usize, None);
        grid
    }

    /// Clamps a given coordinate to the grid bounds.
    pub fn clamp_coord(&self, coord: Coord) -> Coord {
        Coord {
            x: coord.x.min(self.width - 1),
            y: coord.y.min(self.height - 1),
        }
    }

    /// Converts a coordinate into a Vec2, applying scale.
    pub fn map_coord(&self, coord: Coord) -> Vec2 {
        Vec2::new(
            coord.x.min(self.width - 1) as f32 * self.cell_size,
            coord.y.min(self.height - 1) as f32 * self.cell_size,
        ) + self.anchor
    }

    /// Converts a Vec2 into a coordinate, applying scale.
    pub fn map_vec(&self, vec: Vec2) -> Coord {
        let a = vec - self.anchor;
        self.clamp_coord(Coord {
            x: (a.x / self.cell_size) as u32,
            y: (a.y / self.cell_size) as u32,
        })
    }

    /// Returns the width of the grid.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the grid.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Adds a cell to the grid.
    pub fn insert_cell(&mut self, coord: Coord, cell: T) {
        let index = self.index(coord);
        self.cells[index] = Some(cell);
    }

    /// Gets the cell data at given coordinate.
    pub fn cell_data(&self, coord: Coord) -> Option<&T> {
        let i = self.index(coord);
        if let Some(cell) = self.cells.get(i) {
            cell.as_ref()
        } else {
            None
        }
    }

    /// Gets the cell data at given coordinate.
    pub fn cell_data_mut(&mut self, coord: Coord) -> Option<&mut T> {
        let i = self.index(coord);
        if let Some(cell) = self.cells.get_mut(i) {
            cell.as_mut()
        } else {
            None
        }
    }

    /// Removes a cell at given coordinate.
    pub fn remove_cell(&mut self, coord: Coord) {
        let index = self.index(coord);
        self.cells[index] = None;
    }

    /// Computes the index of the given coordinate.
    fn index(&self, coord: Coord) -> usize {
        (coord.y * self.width + coord.x) as usize
    }
}
