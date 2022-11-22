use crate::utils::ClampedArithmetic;

use super::{Pet, PetFood};

pub struct Dog {
    boredom: u8,
    hunger: u8,
    health: u8,
    sounds: Vec<String>,
}

impl Pet for Dog {
    const BOREDOM_LIMIT: u8 = 75;
    const HUNGER_LIMIT: u8 = 80;

    fn get_boredom(&self) -> u8 {
        self.boredom
    }

    fn get_hunger(&self) -> u8 {
        self.hunger
    }

    fn get_health(&self) -> u8 {
        self.health
    }

    fn get_sounds(&self) -> &[String] {
        self.sounds.as_slice()
    }

    fn feed<F: PetFood>(&mut self, food: &F) {
        self.hunger = self.hunger.clamped_sub(food.get_nutrition(), 0..=100);
    }

    fn train(&mut self, sound: String) -> bool {
        if self.sounds.len() >= 5 {
            false
        } else {
            self.sounds.push(sound);

            self.boredom = self.boredom.clamped_sub(50, 0..=100);
            self.hunger = self.hunger.clamped_add(25, 0..=100);

            true
        }
    }

    fn tick(&mut self) {
        todo!()
    }
}
