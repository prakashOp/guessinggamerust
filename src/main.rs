use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("1 -> Play Game");
    println!("2 -> Exit");

    let mut user_choice: String = String::new();

    print!("Choice -> ");
    io::stdin()
        .read_line(&mut user_choice)
        .expect("Failed to Read user choise");

    let user_choice: u8 = user_choice.parse().unwrap();

    match user_choice {
        1 => {
            //calling game_logic()
        },
        2 => {
            print!("Exiting Program");
            std::process::exit(0);
        },
        _ => {
            println!("Enter Wrong Choice");
            main();
        }
    }
}
//ok
fn random_num_gen() -> u32 {
    let randomNum: u32 = rand::thread_rng().gen_range(0..10);
    return randomNum;
}
//working
fn game_logic() {}
