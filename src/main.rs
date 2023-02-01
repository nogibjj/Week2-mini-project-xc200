use std::io;

fn main() {
    println!("Welcome to the Rust Adventure Game!");
    println!("You find yourself in a dark forest. You can either go North or South.");

    loop {
        println!("Which direction do you want to go? (N/S)");

        let mut direction = String::new();
        io::stdin().read_line(&mut direction)
            .expect("Failed to read line");

        direction = direction.trim().to_uppercase();

        match direction.as_str() {
            "N" => {
                println!("You went North and found a castle.");
                println!("Do you want to go inside? (Y/N)");

                let mut choice = String::new();
                io::stdin().read_line(&mut choice)
                    .expect("Failed to read line");

                choice = choice.trim().to_uppercase();

                match choice.as_str() {
                    "Y" => {
                        println!("You went inside the castle and won the game!");
                        break;
                    },
                    "N" => {
                        println!("You decided to stay outside. You can either go North or South.");
                        continue;
                    },
                    _ => {
                        println!("Invalid input. Please enter Y or N.");
                        continue;
                    }
                }
            },
            "S" => {
                println!("You went South and found a river.");
                println!("Do you want to cross the river? (Y/N)");

                let mut choice = String::new();
                io::stdin().read_line(&mut choice)
                    .expect("Failed to read line");

                choice = choice.trim().to_uppercase();

                match choice.as_str() {
                    "Y" => {
                        println!("You crossed the river and lost the game!");
                        break;
                    },
                    "N" => {
                        println!("You decided to stay at the river. You can either go North or South.");
                        continue;
                    },
                    _ => {
                        println!("Invalid input. Please enter Y or N.");
                        continue;
                    }
                }
            },
            _ => {
                println!("Invalid input. Please enter N or S.");
                continue;
            }
        }
    }
}
