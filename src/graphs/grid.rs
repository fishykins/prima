use std::collections::HashMap;

use glam::Vec2;

use crate::{core::Plane2D, geom::Coord};

/// A grid based on a 2D coordinate system.
#[derive(Clone)]
pub struct Grid<T> {
    /// The number of cells in the x plane
    width: u32,
    /// The number of cells in the y plane
    height: u32,
    /// The size of each cell (which is assumed to be a square)
    cell_size: f32,
    /// World anchor point.
    anchor: Vec2,
    /// Cell data
    cells: HashMap<Coord, T>,
    /// A boolean map of the grid, using x as the outer index and y as the inner index.
    x_map: Vec<Vec<bool>>,
    /// A boolean map of the grid, using y as the outer index and x as the inner index.
    y_map: Vec<Vec<bool>>,
}

impl<T> Grid<T>
where
    T: Clone,
{
    /// Makes a new grid.
    pub fn new(width: u32, height: u32, cell_size: f32, anchor: Vec2) -> Grid<T> {
        Grid {
            width,
            height,
            cell_size,
            anchor,
            cells: HashMap::with_capacity((width * height) as usize),
            x_map: vec![vec![false; height as usize]; width as usize],
            y_map: vec![vec![false; width as usize]; height as usize],
        }
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
        self.cells.insert(coord, cell);
        self.x_map[coord.x as usize][coord.y as usize] = true;
        self.y_map[coord.y as usize][coord.x as usize] = true;
    }

    /// Gets the cell data at given coordinate.
    pub fn cell_data(&self, coord: Coord) -> Option<&T> {
        self.cells.get(&self.clamp_coord(coord))
    }

    /// Gets the cell data at given coordinate.
    pub fn cell_data_mut(&mut self, coord: Coord) -> Option<&mut T> {
        self.cells.get_mut(&self.clamp_coord(coord))
    }

    /// Removes a cell at given coordinate.
    pub fn remove_cell(&mut self, coord: Coord) {
        self.cells.remove(&self.clamp_coord(coord));
        self.x_map[coord.x as usize][coord.y as usize] = false;
        self.y_map[coord.y as usize][coord.x as usize] = false;
    }

    /// Gets all cells in the grid.
    pub fn cells(&self) -> Vec<(Coord, &T)> {
        self.cells.iter().map(|(c, t)| (*c, t)).collect()
    }

    /// Gets all cells lying on the given plane.
    pub fn cells_on_plane(&self, plane: Plane2D, value: u32) -> Vec<Coord> {
        match plane {
            Plane2D::X => self.x_map[value as usize]
                .iter()
                .enumerate()
                .filter(|(_, &b)| b)
                .map(|(y, _)| Coord::new(value, y as u32))
                .collect(),
            Plane2D::Y => self.y_map[value as usize]
                .iter()
                .enumerate()
                .filter(|(_, &b)| b)
                .map(|(x, _)| Coord::new(x as u32, value))
                .collect(),
        }
    }
}
