extern crate rand;

use rand::Rng;
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum CellState {
    ALIVE,
    DEAD,
}

#[derive(Copy, Clone)]
pub struct Cell {
    state: CellState,
}

#[derive(Copy, Clone, Eq, Hash)]
pub struct Position {
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

#[derive(Clone)]
pub struct Dimension {
    width: i32,
    height: i32,
}

impl Dimension {
    pub fn new(width: i32, height: i32) -> Dimension {
        Dimension {width: width, height: height}
    }
}

#[derive(Clone)]
pub struct GameRule {
    // if any cell has exactly these number of neighbors and is alive the cell survives
    surviving_cell_rule: Vec<i32>,
    // if any cell has exactly these number of neighbors and is dead the cell is born
    born_cell_rule: Vec<i32>,
    // in any other case the cell dies or stays dead
}


impl GameRule {
    pub fn new(surviving_cell_rule: Vec<i32>, born_cell_rule: Vec<i32>) -> GameRule {
        GameRule { surviving_cell_rule: surviving_cell_rule, born_cell_rule: born_cell_rule }
    }

    pub fn default_rule() -> GameRule {
        GameRule::new( vec![2,3], vec![3])
    }
}

pub struct World {
    cells: HashMap<Position, Cell>,
    dimension: Dimension,
    rule: GameRule,
}

impl World {
    pub fn create_initial_world(dimension: Dimension, state_generator: &Fn(&Dimension, &Position) -> CellState) -> World {
        let mut world = World { cells: HashMap::new(), dimension: dimension, rule: GameRule::default_rule() };
        for y in 0..world.dimension.height {
            for x in 0..world.dimension.width {
                let position = Position { x: x, y: y };
                world.cells.insert(position,
                    Cell { state: state_generator(&world.dimension, &position) }
                );
            }
        }

        world
    }

    pub fn set_rule (&mut self, rule: GameRule) {
        self.rule = rule;
    }

    pub fn create_random_world(dimension: Dimension) -> World {
        World::create_initial_world(dimension, &World::random_world_state_generator)
    }

    pub fn create_dead_world(dimension: Dimension) -> World {
        World::create_initial_world(dimension, &World::dead_world_state_generator)
    }

    pub fn create_glider_world(dimension: Dimension) -> World {
        World::create_initial_world(dimension, &World::glider_world_state_generator)
    }

    pub fn create_spaceship_world(dimension: Dimension) -> World {
        World::create_initial_world(dimension, &World::spaceship_world_state_generator)
    }

    pub fn create_big_crunch_world(dimension: Dimension) -> World {
        World::create_initial_world(dimension, &World::big_crunch_world_state_generator)
    }

    fn dead_world_state_generator(_dimension: &Dimension, _grid_position: &Position) -> CellState {
        CellState::DEAD
    }

    fn random_world_state_generator(_dimension: &Dimension, _grid_position: &Position) -> CellState {
        let mut rng = rand::thread_rng();
        if rng.gen() {
            CellState::ALIVE
        } else {
            CellState::DEAD
        }
    }

    fn glider_world_state_generator(_dimension: &Dimension, grid_position: &Position) -> CellState {
        let glider = vec![
            Position { x: 0, y: 1 },
            Position { x: 1, y: 2 },
            Position { x: 2, y: 0 },
            Position { x: 2, y: 1 },
            Position { x: 2, y: 2 },
        ];

        let offsets = vec![Position {x: 0, y: 8}, Position {x: 0, y: 2}, 
                           Position {x: 8, y: 8}, Position {x: 8, y: 2},
                           Position {x: 15, y: 8}, Position {x: 15, y: 2},
                           Position {x: 22, y: 8}, Position {x: 22, y: 2},
                           ];
        let all_gliders = World::apply_offsets_to_shape (&glider, &offsets);

        World::get_cell_state_in_vector(&all_gliders, grid_position)
    }

    fn spaceship_world_state_generator(_dimension: &Dimension, grid_position: &Position) -> CellState {
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

        World::get_cell_state_in_vector(&all_spaceships, grid_position)
    }

    fn big_crunch_world_state_generator(dimension: &Dimension, grid_position: &Position) -> CellState {
        let big_crunch_initial_state = vec![
            Position { x: 0, y: 0 },
            Position { x: 1, y: 0 },
            Position { x: 2, y: 0 },
            Position { x: 0, y: 1 },
            Position { x: 0, y: 2 },
            Position { x: 2, y: 1 },
            Position { x: 2, y: 2 },

            Position { x: 0, y: 6 },
            Position { x: 1, y: 6 },
            Position { x: 2, y: 6 },
            Position { x: 0, y: 4 },
            Position { x: 0, y: 5 },
            Position { x: 2, y: 4 },
            Position { x: 2, y: 5 },
        ];

        let offsets = vec![Position {x: (dimension.width/2)-2, y: dimension.height/2-3}];
        let big_crunch_initial_state = World::apply_offsets_to_shape (&big_crunch_initial_state, &offsets);

        World::get_cell_state_in_vector(&big_crunch_initial_state, grid_position)
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

    fn get_cell_in_world(&self, position: &Position) -> Option<&Cell> {
        self.cells.get(position)
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
                match self.rule.surviving_cell_rule.iter().find(|&&x| num_neigbors == x) {
                    Some (_x) => CellState::ALIVE,
                    None      => CellState::DEAD,
                }
            }
            CellState::DEAD => {
                match self.rule.born_cell_rule.iter().find(|&&x| num_neigbors == x) {
                    Some (_x) => CellState::ALIVE,
                    None      => CellState::DEAD,
                }
            }
        }
    }

    pub fn populate(&self) -> World {
        let mut new_cells: HashMap<Position, Cell> = HashMap::new();
        for (position, cell) in self.cells.iter() {
            let new_cell = Cell { state: self.get_new_cell_state(cell, &position) };
            new_cells.insert(position.clone(), new_cell);
        }

        return World { cells: new_cells, dimension: self.dimension.clone(), rule: self.rule.clone() };
    }

    pub fn print(&self) {
        fn print_bar(width: i32) {
            print!("|");
            for _i in 0..width {
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

        print_bar(self.dimension.width);

        for y in 0..self.dimension.height {
            for x in 0..self.dimension.width {
                let position = Position { x: x, y: y };
                let cell = self.get_cell_in_world(&position).unwrap();

                if position.x == 0 {
                    print!("|");
                }

                print_cell(cell);

                if position.x == self.dimension.width - 1 {
                    println!("|");
                }
            }
        }

        print_bar(self.dimension.width);
    }
}
