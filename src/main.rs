extern crate game_of_life;

const WIDTH: i32 = 40;
const HEIGHT: i32 = 20;

use game_of_life::{World, Dimension};
use std::{thread, time};

fn simulate_game_of_life(num_iterations: usize) -> Vec<Box<World>> {
    let dimension = Dimension::new(WIDTH, HEIGHT);

    let mut worlds: Vec<Box<World>> =
        vec![Box::new(World::create_random_world(dimension))];

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
        world.print();
        println!("");
        let sleep_time = time::Duration::from_millis(100);
        thread::sleep(sleep_time);
    }

}
