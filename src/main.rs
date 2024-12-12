use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("1 -> Play Game");
    println!("2 -> Exit");
    println!("Choice -> ");
    let mut user_choice: String = String::new();
    
    io::stdin()
        .read_line(&mut user_choice)
        .expect("Failed to Read user choise");

    // Handle potential parsing errors
    let user_choice: u8 = match user_choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return; // Exit main if parsing fails
        }
    };

    match user_choice {
        1 => {
            game_logic();
        }
        2 => {
            print!("Exiting Program");
            std::process::exit(0);
        }
        _ => {
            println!("Enter Wrong Choice");
            main(); // You might want to avoid recursion here
        }
    }
}

fn game_logic() {
    // Generate a random number between 1 and 10 (inclusive)
    let random_num: u32 = rand::thread_rng().gen_range(1..=10); 

    loop {
        println!("Input -> ");
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user_input");

        // Parse user input to u32
        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match user_input.cmp(&random_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}