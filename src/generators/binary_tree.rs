use crate::generators::{GenerationState, MazeGenerator};
use crate::grid::{Boundary, BoundaryState, Grid};
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

pub struct BinaryTreeGenerator {
    pub grid: Grid,
    rng: ThreadRng,

    // For the state of the maze generation process
    x: usize,
    y: usize,
}

impl MazeGenerator for BinaryTreeGenerator {
    fn new(grid: Grid) -> BinaryTreeGenerator {
        BinaryTreeGenerator {
            grid: grid,
            rng: thread_rng(),
            x: 0,
            y: 0,
        }
    }

    fn step(&mut self) -> GenerationState {
        if self.x >= self.grid.size.x || self.y >= self.grid.size.y {
            return GenerationState::Finished;
        }

        // Checking boundaries first to see if one or both paths cannot be opened
        if self.x == self.grid.size.x - 1 {
            if self.y == 0 {
            } else {
                self.grid.change_boundary_state(
                    (self.x, self.y),
                    Boundary::Up,
                    BoundaryState::Open,
                );
            }
        } else if self.y == 0 {
            self.grid
                .change_boundary_state((self.x, self.y), Boundary::Right, BoundaryState::Open);
        }
        // Otherwise pick at random
        else {
            if self.rng.gen_range(0..2) == 0 {
                self.grid.change_boundary_state(
                    (self.x, self.y),
                    Boundary::Up,
                    BoundaryState::Open,
                );
            } else {
                self.grid.change_boundary_state(
                    (self.x, self.y),
                    Boundary::Right,
                    BoundaryState::Open,
                );
            }
        }

        self.x += 1;
        if self.x >= self.grid.size.x {
            self.x = 0;
            self.y += 1;
        }

        GenerationState::Unfinished
    }

    fn generate(&mut self) {
        let mut done = false;

        while !done {
            match self.step() {
                GenerationState::Unfinished => {}
                GenerationState::Finished => {
                    done = true;
                }
            }
        }
    }

    fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
    }
}
