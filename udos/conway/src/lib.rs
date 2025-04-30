#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Grid {
            rows,
            cols,
            grid: vec![T::default(); rows * cols],
        }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        Grid {
            rows,
            cols,
            grid: grid.to_vec(),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        let ind = row * self.cols + col;
        &self.grid[ind]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        let ind = row * self.cols + col;
        self.grid[ind] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbours_: Vec<(usize, usize)> = Vec::new();
        if row > 0 && col > 0 {
            neighbours_.push((row - 1_usize, col - 1_usize));
        }
        if row > 0 {
            neighbours_.push((row - 1_usize, col));
        }
        if row > 0 && col < self.cols - 1_usize {
            neighbours_.push((row - 1_usize, col + 1_usize));
        }
        if col > 0 {
            neighbours_.push((row, col - 1_usize));
        }
        if col < self.cols - 1_usize {
            neighbours_.push((row, col + 1_usize));
        }
        if row < self.rows - 1_usize && col > 0 {
            neighbours_.push((row + 1_usize, col - 1_usize));
        }
        if row < self.rows - 1_usize {
            neighbours_.push((row + 1_usize, col));
        }
        if row < self.rows - 1_usize && col < self.cols - 1_usize {
            neighbours_.push((row + 1_usize, col + 1_usize));
        }
        neighbours_
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        GameOfLife { grid }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    pub fn step(&mut self) {
        let ind = self.grid.size();
        let mut next_grid: Grid<Cell> = Grid::new(ind.0, ind.1);

        for row in 0..ind.0 {
            for col in 0..ind.1 {
                let neighbours_ = self.grid.neighbours(row, col);
                let mut count = 0;
                for neib in neighbours_ {
                    if self.grid.get(neib.0, neib.1) == &Cell::Alive {
                        count += 1;
                    }
                }
                match count {
                    2 => next_grid.set(*self.grid.get(row, col), row, col),
                    3 => next_grid.set(Cell::Alive, row, col),
                    _ => next_grid.set(Cell::Dead, row, col),
                }
            }
        }
        self.grid = next_grid;
    }
}
