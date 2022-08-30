use crate::grid::{Boundary, BoundaryState, Cell, Grid};

use std::cmp::min;
use std::collections::VecDeque;

pub fn calculate_distances(grid: &Grid, starting_point: (usize, usize)) -> Vec<f32> {
    let (x, y) = starting_point;

    let mut visited = vec![false; grid.size.x * grid.size.y];
    let mut coors_to_visit: VecDeque<(usize, usize)> =
        VecDeque::with_capacity(grid.size.x * grid.size.y);
    let mut to_visit: VecDeque<&Cell> = VecDeque::with_capacity(grid.size.x * grid.size.y);
    let mut distances = vec![i32::MAX; grid.size.x * grid.size.y];

    distances[y * grid.size.x + x] = 0;

    coors_to_visit.push_back((x, y));
    to_visit.push_back(&grid[[x, y]]);

    while !to_visit.is_empty() {
        let cell = to_visit.pop_front().unwrap();
        let (x, y) = coors_to_visit.pop_front().unwrap();

        let cell_i = y * grid.size.x + x;

        if visited[cell_i] {
            continue;
        }

        visited[cell_i] = true;

        if cell.boundaries[Boundary::Up as usize] == BoundaryState::Open {
            coors_to_visit.push_back((x, y - 1));
            to_visit.push_back(&grid[[x, y - 1]]);

            let i = (y - 1) * grid.size.x + x;
            distances[i] = min(distances[i], distances[cell_i] + 1);
        }
        if cell.boundaries[Boundary::Right as usize] == BoundaryState::Open {
            coors_to_visit.push_back((x + 1, y));
            to_visit.push_back(&grid[[x + 1, y]]);

            let i = y * grid.size.x + x + 1;
            distances[i] = min(distances[i], distances[cell_i] + 1);
        }
        if cell.boundaries[Boundary::Down as usize] == BoundaryState::Open {
            coors_to_visit.push_back((x, y + 1));
            to_visit.push_back(&grid[[x, y + 1]]);

            let i = (y + 1) * grid.size.x + x;
            distances[i] = min(distances[i], distances[cell_i] + 1);
        }
        if cell.boundaries[Boundary::Left as usize] == BoundaryState::Open {
            coors_to_visit.push_back((x - 1, y));
            to_visit.push_back(&grid[[x - 1, y]]);

            let i = y * grid.size.x + x - 1;
            distances[i] = min(distances[i], distances[cell_i] + 1);
        }
    }

    let mut max_distance = distances[0];
    for d in &distances {
        if *d > max_distance {
            max_distance = *d;
        }
    }

    distances
        .iter()
        .map(|&x| 1. - x as f32 / max_distance as f32)
        .collect()
}
