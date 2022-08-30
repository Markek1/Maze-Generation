pub mod binary_tree;
pub mod sidewinder;

use crate::grid::Grid;

pub enum GenerationState {
    Unfinished,
    Finished,
}

pub trait MazeGenerator {
    fn new(grid: Grid) -> Self;

    /// Does the next step in maze generation.
    /// Used when the generation is supposed to be recorded or debugged
    fn step(&mut self) -> GenerationState;

    /// Calls the step function until it returns GenerationState::Finished
    fn generate(&mut self);

    fn reset(&mut self);
}
