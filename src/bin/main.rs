use macroquad::prelude::*;

use maze_generators::{
    djikstra_distances::calculate_distances,
    generators::binary_tree::BinaryTreeGenerator,
    generators::sidewinder::SidewinderGenerator,
    generators::{GenerationState, MazeGenerator},
    grid::{Boundary, BoundaryState, Grid},
    helpers::UsizeVec2,
};

const WINDOW_SIZE: UsizeVec2 = UsizeVec2 { x: 800, y: 800 };
const GRID_SIZE: UsizeVec2 = UsizeVec2 { x: 80, y: 80 };
const BOUNDARY_LINE_WIDTH: f32 = 3.;

fn draw_grid(grid: &Grid, cell_size_px: usize, boundary_line_width: f32) {
    // Draw far left boundary
    for y in 0..grid.size.y {
        let x = 0;

        if grid[[x, y]].boundaries[Boundary::Left as usize] == BoundaryState::Closed {
            draw_line(
                x as f32,
                (y * cell_size_px) as f32,
                x as f32,
                ((y + 1) * cell_size_px) as f32,
                boundary_line_width,
                BLACK,
            );
        }
    }

    // Draw far down boundary
    for x in 0..grid.size.x {
        let y = grid.size.y - 1;

        if grid[[x, y]].boundaries[Boundary::Down as usize] == BoundaryState::Closed {
            draw_line(
                x as f32,
                ((y + 1) * cell_size_px) as f32,
                ((x + 1) * cell_size_px) as f32,
                ((y + 1) * cell_size_px) as f32,
                boundary_line_width,
                BLACK,
            );
        }
    }

    // Draw up and right boundaries for the rest of the cells
    for y in 0..grid.size.y {
        for x in 0..grid.size.x {
            if grid[[x, y]].boundaries[Boundary::Up as usize] == BoundaryState::Closed {
                draw_line(
                    (x * cell_size_px) as f32,
                    (y * cell_size_px) as f32,
                    ((x + 1) * cell_size_px) as f32,
                    (y * cell_size_px) as f32,
                    boundary_line_width,
                    BLACK,
                );
            }

            if grid[[x, y]].boundaries[Boundary::Right as usize] == BoundaryState::Closed {
                draw_line(
                    ((x + 1) * cell_size_px) as f32,
                    (y * cell_size_px) as f32,
                    ((x + 1) * cell_size_px) as f32,
                    ((y + 1) * cell_size_px) as f32,
                    boundary_line_width,
                    BLACK,
                );
            }
        }
    }
}

fn window_config() -> Conf {
    Conf {
        window_title: "Maze Generators".to_owned(),
        window_width: WINDOW_SIZE.x as i32,
        window_height: WINDOW_SIZE.y as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut grid = Grid::new(GRID_SIZE);

    assert_eq!(WINDOW_SIZE.x / grid.size.x, WINDOW_SIZE.y / grid.size.y);
    let cell_size_px = WINDOW_SIZE.x / grid.size.x;

    let mut maze_generator = SidewinderGenerator::new(grid);
    maze_generator.generate();

    let mut paused = false;

    let mut centre_of_distance: (usize, usize) = (GRID_SIZE.x / 2, GRID_SIZE.y / 2);
    let mut distances = calculate_distances(&maze_generator.grid, centre_of_distance);
    let mut should_distances_be_calculated = false;

    loop {
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::R) {
            for cell in &mut maze_generator.grid.grid {
                cell.boundaries = [BoundaryState::Closed; 4];
            }
            maze_generator.reset();
            should_distances_be_calculated = true;
        }
        if is_mouse_button_down(MouseButton::Left) {
            centre_of_distance = (
                (mouse_position().0 / cell_size_px as f32) as usize,
                (mouse_position().1 / cell_size_px as f32) as usize,
            );
            should_distances_be_calculated = true;
        }

        if !paused {
            maze_generator.generate();
        }
        if should_distances_be_calculated {
            distances = calculate_distances(&maze_generator.grid, centre_of_distance);
            should_distances_be_calculated = false;
        }

        clear_background(WHITE);

        for y in 0..maze_generator.grid.size.y {
            for x in 0..maze_generator.grid.size.x {
                draw_rectangle(
                    (x * cell_size_px) as f32,
                    (y * cell_size_px) as f32,
                    ((x + 1) * cell_size_px) as f32,
                    ((y + 1) * cell_size_px) as f32,
                    Color {
                        r: 0.,
                        g: distances[y * maze_generator.grid.size.y + x],
                        b: 0.,
                        a: 1.,
                    },
                );
            }
        }

        draw_grid(&maze_generator.grid, cell_size_px, BOUNDARY_LINE_WIDTH);

        // if !paused {
        //     for _ in 0..15 {
        //         if !done_generating {
        //             match maze_generator.step() {
        //                 GenerationState::Unfinished => {}
        //                 GenerationState::Finished => {
        //                     done_generating = true;
        //                 }
        //             }
        //         }
        //     }
        // }

        next_frame().await
    }
}
