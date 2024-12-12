use std::io;
use rand::Rng;
use colored::*;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
}
//ok
fn random_num_gen() -> u32 {
    let randomNum: u32 = rand::thread_rng().gen_range(0..10);
    return randomNum;
}
//working
fn game_logic() {
    
}
