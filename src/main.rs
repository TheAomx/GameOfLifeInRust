extern crate game_of_life;
extern crate termion;

use game_of_life::{World, Dimension, GameRule};
use std::{thread, time};

fn get_world_dimension () -> Dimension {
    const DEFAULT_WIDTH: i32 = 40;
    const DEFAULT_HEIGHT: i32 = 20;

    match termion::terminal_size() {
        Ok(result) => {
            let (cols, rows) = result;
            let width = (cols - 2) as i32;
            let height = (rows - 3) as i32;
            Dimension::new(width, height)
        },
        Err(_err)  => Dimension::new(DEFAULT_WIDTH, DEFAULT_HEIGHT),
    }
}

fn simulate_game_of_life(num_iterations: usize) -> Vec<Box<World>> {
    let dimension = get_world_dimension();
    let mut initial_world = World::create_random_world(dimension);
    initial_world.set_rule(GameRule::default_rule());
    let mut worlds: Vec<Box<World>> = Vec::with_capacity(num_iterations);
    worlds.push(Box::new(initial_world));

    for x in 1..num_iterations {
        let populated_world = worlds[x-1].populate();
        worlds.push(Box::new(populated_world));
    }

    worlds
}

fn main() {
    let num_iterations: usize = 200;
    let worlds = simulate_game_of_life(num_iterations);

    for world in worlds.iter() {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        world.print();
        let sleep_time = time::Duration::from_millis(100);
        thread::sleep(sleep_time);
    }

}
