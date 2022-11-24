use core::panic;
use std::io::stdin;
use virtual_pet::pet::{dog::Dog, food::*, Pet, PetFood};

fn main() {
    println!("`virtual-pet` in Rust!");
}

pub struct Simulation {
    pet: Dog,
    pet_name: String,
    tick_count: usize,
    wallet: usize,
}

impl Simulation {
    pub fn new(pet_name: String) -> Self {
        Self {
            pet: Dog::new(),
            pet_name,
            tick_count: 0,
            wallet: 0,
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        self.pet.tick();
        let mood = self.pet.mood();
        println!(
            "\"{}\"'s mood: {} {} {}",
            self.pet_name, mood.0, mood.1, mood.2
        );

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut input = input.trim().split_ascii_whitespace();

        if let Some(command) = input.next() {
            match command {
                "feed" => {
                    let food: Box<dyn PetFood> = match input.next().expect("no food type provided")
                    {
                        "cake" => Box::new(Cake),
                        "berry" => Box::new(Berry),
                        "banana" => Box::new(Banana),
                        "peach" => Box::new(Peach),
                        "pea" => Box::new(Pea),
                        "bean" => Box::new(Bean),
                        "pod" => Box::new(Pod),
                        _ => panic!("Unknown food"),
                    };

                    if self.wallet.checked_sub(food.get_cost()).is_none() {
                        panic!("No money for food :(");
                    } else {
                        self.wallet -= food.get_cost();
                    }
                }
                "teach" => {
                    let success = self.pet.train(
                        input
                            .next()
                            .expect("Expected argument for sound")
                            .to_owned(),
                    );

                    println!(
                        "{}",
                        if success {
                            "Successfully trained pet"
                        } else {
                            "Failed to train pet"
                        }
                    );
                }
                "sounds" => {
                    println!("{:?}", self.pet.get_sounds());
                }
                _ => panic!("Unknown command"),
            }
        }
    }
}
