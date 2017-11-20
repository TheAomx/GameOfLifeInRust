extern crate rand;

use rand::Rng;
use std::{thread, time};

const WIDTH: i32 = 40;
const HEIGHT: i32 = 15;

#[derive(Copy, Clone)]
enum CellState {
    ALIVE,
    DEAD,
}

#[derive(Copy, Clone)]
struct Cell {
    state: CellState,
}

#[derive(Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn shift (&self, factor: &Position) -> Position {
        Position { x: self.x + factor.x, y: self.y + factor.y}
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Copy, Clone)]
struct Dimension {
    width: i32,
    height: i32,
}

#[derive(Clone)]
struct World {
    cells: Vec<(Position, Cell)>,
}

impl World {
    fn create_initial_world(dimension: &Dimension, state_generator: &Fn(Position) -> CellState) -> World {
        let mut world = World { cells: Vec::new() };
        for y in 0..dimension.height {
            for x in 0..dimension.width {
                let position = Position { x: x, y: y };
                world.cells.push((
                    position,
                    Cell { state: state_generator(position) },
                ));
            }
        }

        world
    }

    fn create_random_world(dimension: &Dimension) -> World {
        World::create_initial_world(dimension, &World::random_world_state_generator)
    }

    fn create_dead_world(dimension: &Dimension) -> World {
        World::create_initial_world(dimension, &World::dead_world_state_generator)
    }

    fn create_glider_world(dimension: &Dimension) -> World {
        World::create_initial_world(dimension, &World::glider_world_state_generator)
    }

    fn create_spaceship_world(dimension: &Dimension) -> World {
        World::create_initial_world(dimension, &World::spaceship_world_state_generator)
    }

    fn dead_world_state_generator(_grid_position: Position) -> CellState {
        CellState::DEAD
    }

    fn random_world_state_generator(_grid_position: Position) -> CellState {
        let mut rng = rand::thread_rng();
        if rng.gen() {
            CellState::ALIVE
        } else {
            CellState::DEAD
        }
    }

    fn glider_world_state_generator(grid_position: Position) -> CellState {
        let glider = vec![
            Position { x: 0, y: 1 },
            Position { x: 1, y: 2 },
            Position { x: 2, y: 0 },
            Position { x: 2, y: 1 },
            Position { x: 2, y: 2 },
        ];

        let offsets = vec![Position {x: 0, y: 8}, Position {x: 0, y: 2}, 
                           Position {x: 8, y: 8}, Position {x: 8, y: 2},
                           Position {x: 15, y: 8}, Position {x: 15, y: 2},];
        let all_gliders = World::apply_offsets_to_shape (&glider, &offsets);

        World::get_cell_state_in_vector(&all_gliders, &grid_position)
    }

    fn spaceship_world_state_generator(grid_position: Position) -> CellState {
        let spaceship = vec![
            Position { x: 1, y: 0 },
            Position { x: 2, y: 0 },
            Position { x: 3, y: 0 },
            Position { x: 4, y: 0 },
            Position { x: 0, y: 1 },
            Position { x: 4, y: 1 },
            Position { x: 4, y: 2 },
            Position { x: 0, y: 3 },
            Position { x: 3, y: 3 },
        ];

        let offsets = vec![Position {x: 0, y: 8}, Position {x: 0, y: 2}, 
                           Position {x: 8, y: 8}, Position {x: 8, y: 2}];
        let all_spaceships = World::apply_offsets_to_shape (&spaceship, &offsets);

        World::get_cell_state_in_vector(&all_spaceships, &grid_position)
    }

    fn get_cell_state_in_vector(vector: &Vec<Position>, position: &Position) -> CellState {
        let some_cell = vector.iter().find(|&cell| position == cell);
        match some_cell {
            Some(_cell) => CellState::ALIVE,
            None => CellState::DEAD,
        }
    }

    fn apply_offsets_to_shape (shape: &Vec<Position>, offsets: &Vec<Position>) -> Vec<Position> {
        let mut positions = vec![];

        for offset in offsets {
             let shape_with_offset : Vec<Position> = shape.iter().map(|&position| position.shift(&offset)).collect();
             positions.extend(shape_with_offset);
        }

        positions
    }

    fn get_cell_in_world(&self, position: &Position) -> Option<Cell> {
        let callback = |entry: &&(Position, Cell)| {
            let ref cell_position = entry.0;
            position.x == cell_position.x && position.y == cell_position.y
        };

        match self.cells.iter().find(callback) {
            Some(entry) => Some(entry.1),
            None => None,
        }
    }

    fn get_neighbor_count(&self, position: &Position) -> i32 {
        let found_cell = self.get_cell_in_world(position);
        match found_cell {
            Some(cell) => {
                match cell.state {
                    CellState::ALIVE => 1,
                    CellState::DEAD => 0,
                }
            }
            None => 0,
        }
    }

    fn get_num_neigbors_for_cell(&self, position: &Position) -> i32 {
        let start_x = position.x - 1;
        let end_x = position.x + 2;
        let start_y = position.y - 1;
        let end_y = position.y + 2;
        let mut num_neigbors = 0;

        for x in start_x..end_x {
            for y in start_y..end_y {
                if x == position.x && y == position.y {
                } else {
                    let neighbor_position = Position { x: x, y: y };
                    num_neigbors += self.get_neighbor_count(&neighbor_position);
                }
            }
        }

        num_neigbors
    }

    fn get_new_cell_state(&self, cell: &Cell, position: &Position) -> CellState {
        let num_neigbors = self.get_num_neigbors_for_cell(position);
        match cell.state {
            CellState::ALIVE => {
                if num_neigbors == 2 || num_neigbors == 3 {
                    CellState::ALIVE
                } else {
                    CellState::DEAD
                }
            }
            CellState::DEAD => {
                if num_neigbors == 3 {
                    CellState::ALIVE
                } else {
                    CellState::DEAD
                }
            }
        }
    }

    fn populate(&self) -> World {
        let mut new_cells: Vec<(Position, Cell)> = vec![];
        for &(position, ref cell) in self.cells.iter() {
            let new_cell = Cell { state: self.get_new_cell_state(cell, &position) };
            new_cells.push((position, new_cell));
        }

        return World { cells: new_cells };
    }

    fn print(&self) {
        fn print_bar() {
            print!("|");
            for _i in 0..WIDTH {
                print!("-");
            }
            print!("|\n");
        }

        fn print_cell(cell: &Cell) {
            match cell.state {
                CellState::ALIVE => print!("x"),
                CellState::DEAD => print!(" "),
            }
        }

        print_bar();
        for &(ref position, ref cell) in self.cells.iter() {
            if position.x == 0 {
                print!("|");
            }

            print_cell(cell);

            if position.x == WIDTH - 1 {
                println!("|");
            }
        }

        print_bar();
    }
}

fn simulate_game_of_life(num_iterations: usize) -> Vec<Box<World>> {
    let dimension = Dimension {
        width: WIDTH,
        height: HEIGHT,
    };
    let mut worlds: Vec<Box<World>> =
        vec![Box::new(World::create_random_world(&dimension)); num_iterations];
    for x in 1..num_iterations {
        worlds[x] = Box::new(worlds[x - 1].populate());
    }

    worlds
}

fn main() {
    let num_iterations: usize = 100;
    let worlds = simulate_game_of_life(num_iterations);

    for world in worlds.iter() {
        world.print();
        println!("");
        let sleep_time = time::Duration::from_millis(100);
        thread::sleep(sleep_time);
    }

}
