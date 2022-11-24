pub mod dog;
pub mod food;

/// A pet trait with different stats.
///
/// Also defines some basic convenience methods.
pub trait Pet {
    /// The threshold for the pet’s boredom; beyond this point, they start to get hungrier.
    const BOREDOM_LIMIT: u8;

    /// The threshold for the pet’s hunger; beyond this point, the pet starts to lose health from starvation.
    const HUNGER_LIMIT: u8;

    /// Gets the pet’s boredom level. When it gets too high, the pet gets bored, it also gets hungry faster.
    fn get_boredom(&self) -> u8;

    /// Gets the pet’s level of hunger. When it gets too high, the pet’s health starts to go down.
    fn get_hunger(&self) -> u8;

    /// Gets the pet’s health, from 0 to 100.
    fn get_health(&self) -> u8;

    /// Gets the list of sounds the pet knows.
    fn get_sounds(&self) -> &[String];

    /// Checks if the pet is dead. This has a default implemenation which simply checks if the health is 0.
    fn is_dead(&self) -> bool {
        // Note that this uses the implementation of `self.get_health` provided by the concrete type.
        self.get_health() == 0
    }

    /// Returns the rate that the pet's boredom increases each tick.
    fn boredom_rate(&self) -> u8 {
        if self.get_boredom() > Self::BOREDOM_LIMIT {
            8
        } else {
            4
        }
    }

    // /// Returns the rate that the pet's hunger increases each tick.
    fn hunger_rate(&self) -> u8 {
        let initial = if self.get_health() <= 25 { 2 } else { 4 };

        if self.get_boredom() > Self::BOREDOM_LIMIT {
            initial * 2
        } else if self.get_boredom() >= 90 {
            initial * 4
        } else {
            initial
        }
    }

    /// Returns three strings (health, hunger, boredom) which indicate the pet's well-being.
    fn mood(&self) -> (&str, &str, &str) {
        let health = match self.get_health() {
            _ if self.is_dead() => "dead",
            1..=20 => "sick",
            80..=100 => "fighting fit",
            _ => "ok",
        };

        let hunger = match self.get_hunger() {
            0..50 => "full",
            h if (50..Self::HUNGER_LIMIT).contains(&h) => "hungry", // This uses a constant defined by the concrete type. The implementation is terrible :P
            _ => "starving",
        };

        let boredom = match self.get_hunger() {
            b if (0..Self::BOREDOM_LIMIT).contains(&b) => "happy",
            90.. => "angry",
            _ => "bored",
        };

        (health, hunger, boredom)
    }

    /// Feed the [`Pet`] with any kind of [`PetFood`].
    ///
    /// This method will decrease the `Pet`'s hunger.
    fn feed<F: PetFood>(&mut self, food: &F);

    /// 'Trains' the [`Pet`] by adding some sound to its knowledge.
    ///
    /// This method will decrease the `Pet`'s boredom. Returns whether the pet was trained successfully.
    fn train(&mut self, sound: String) -> bool;

    /// Ticks the pet and decreases it's different stats
    fn tick(&mut self);
}

/// Something that a [`Pet`] can eat.
pub trait PetFood {
    /// Returns the amount that the [`Pet`]'s hunger will decrease by.
    fn get_nutrition(&self) -> u8;

    /// Returns the amount that the [`Pet`]'s health will increase by.
    fn get_healing(&self) -> u8;

    /// Returns the amount that the food will cost.
    fn get_cost(&self) -> usize;
}
