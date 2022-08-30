use super::helpers::UsizeVec2;

#[derive(Clone, Copy, PartialEq)]
pub enum BoundaryState {
    Open,
    Closed,
}

#[derive(Clone, Copy)]
pub enum Boundary {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub boundaries: [BoundaryState; 4],
}

pub struct Grid {
    pub size: UsizeVec2,
    pub grid: Vec<Cell>,
}

impl Grid {
    pub fn new(size: UsizeVec2) -> Self {
        Grid {
            size: size,
            grid: vec![
                Cell {
                    boundaries: [BoundaryState::Closed; 4]
                };
                size.y * size.x
            ],
        }
    }

    pub fn change_boundary_state(
        &mut self,
        coors: (usize, usize),
        boundary: Boundary,
        state: BoundaryState,
    ) {
        self[[coors.0, coors.1]].boundaries[boundary as usize] = state;

        // THIS CAN OVERFLOW PLS FIX
        match boundary {
            Boundary::Up => {
                self[[coors.0, coors.1 - 1]].boundaries[Boundary::Down as usize] = state;
            }
            Boundary::Right => {
                self[[coors.0 + 1, coors.1]].boundaries[Boundary::Left as usize] = state;
            }
            Boundary::Down => {
                self[[coors.0, coors.1 + 1]].boundaries[Boundary::Up as usize] = state;
            }
            Boundary::Left => {
                self[[coors.0 - 1, coors.1]].boundaries[Boundary::Right as usize] = state;
            }
        }
    }
}

impl std::ops::Index<[usize; 2]> for Grid {
    type Output = Cell;

    fn index(&self, idx: [usize; 2]) -> &Self::Output {
        &self.grid[idx[1] * self.size.x + idx[0]]
    }
}

impl std::ops::IndexMut<[usize; 2]> for Grid {
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut Cell {
        &mut self.grid[idx[1] * self.size.x + idx[0]]
    }
}
