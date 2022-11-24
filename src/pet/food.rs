use super::PetFood;

macro_rules! petfood {
    ($name:ident, $hunger:literal, $health:literal, $cost:literal) => {
        pub struct $name;

        impl PetFood for $name {
            fn get_nutrition(&self) -> u8 {
                $hunger
            }

            fn get_healing(&self) -> u8 {
                $health
            }

            const COST: u32 = $cost;
        }
    };
}

petfood!(Cake, 2, 0, 0);
petfood!(Berry, 10, 5, 5);
petfood!(Banana, 15, 2, 10);
petfood!(Peach, 20, 0, 20);
petfood!(Pea, 5, 10, 10);
petfood!(Bean, 2, 15, 25);
petfood!(Pod, 0, 20, 40);
