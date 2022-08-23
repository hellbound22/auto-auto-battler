use std::fmt;

use rand::seq::SliceRandom; 

use crate::pet::{Pet, SPet};
use crate::food::Food;

#[derive(Debug)]
pub struct Store {
    pets: Vec<SPet>,
    pet_bucket: Vec<Pet>,
    foods: Vec<Food>, 
    food_bucket: Vec<Food>, 
    // TODO: Implement food   
}

impl Store {
    pub fn new(pet_bucket: Vec<Pet>, food_bucket: Vec<Food>) -> Self {
        let pet_tier_bucket: Vec<Pet> = pet_bucket.iter().filter(|x| x.tier <= 1).cloned().collect();
        let food_tier_bucket: Vec<Food> = food_bucket.iter().filter(|x| x.tier <= 1).cloned().collect();
        Store {
            pets: Self::_internal_roll(pet_tier_bucket, 3),
            pet_bucket,
            foods: Self::_internal_food_roll(food_tier_bucket, 1),
            food_bucket,
        }
    }

    pub fn tier_up_pet(&mut self, tier: f32) {
        let tier_bucket: Vec<Pet> = self.pet_bucket.iter().filter(|x| x.tier == tier as i8).cloned().collect();
        self.pets.push(Store::_internal_roll(tier_bucket, 1)[0].clone());
    }

    pub fn roll(&mut self, pet_slots: u8, food_slots: u8, tier: f32) {
        let frozen_pets: Vec<SPet> = self.pets.iter().filter(|x| x.frozen).cloned().collect();
        let tier_bucket: Vec<Pet> = self.pet_bucket.iter().filter(|x| x.tier <= tier as i8).cloned().collect();
        self.pets = Store::_internal_roll(tier_bucket, pet_slots);

        for i in 0..frozen_pets.len() {
            self.pets[i] = frozen_pets[i].to_owned();
        }

        let frozen_food: Vec<Food> = self.foods.iter().filter(|x| x.frozen).cloned().collect();
        let tier_bucket: Vec<Food> = self.food_bucket.iter().filter(|x| x.tier <= tier as i8).cloned().collect();
        self.foods = Store::_internal_food_roll(tier_bucket, food_slots);

        for i in 0..frozen_food.len() {
            self.foods[i] = frozen_food[i].to_owned();
        }
    }

    fn _internal_roll(bucket: Vec<Pet>, slots: u8) -> Vec<SPet> {
        let mut new_pets: Vec<Pet> = Vec::new();
        for _x in 0..slots {
            new_pets.push(bucket
                .choose(&mut rand::thread_rng())
                .cloned()
                .unwrap());
        }
        new_pets.iter().map(|x| x.into()).collect()
    }
    // TODO: Join implementation of pet/food rolls and freezes
    fn _internal_food_roll(bucket: Vec<Food>, slots: u8) -> Vec<Food> {
        let mut new_food: Vec<Food> = Vec::new();
        for _x in 0..slots {
            new_food.push(bucket
                .choose(&mut rand::thread_rng())
                .cloned()
                .unwrap());
        }
        new_food
    }

    pub fn freeze_and_unfreeze_pet(&mut self, slot: usize) {
        self.pets[slot].frozen = self.pets[slot].frozen ^ true;
    }

    pub fn freeze_and_unfreeze_food(&mut self, slot: usize) {
        self.foods[slot].frozen = self.foods[slot].frozen ^ true;
    }

    pub fn remove_pet(&mut self, slot: u8) -> Pet {
        self.pets.remove(slot.into()).into()
    } 

    pub fn remove_food(&mut self, slot: u8) -> Food {
        self.foods.remove(slot.into()).into()
    } 
}

impl fmt::Display for Store {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pets = String::new();
        for p in &self.pets {
            pets.push_str(&format!("F:{} [{}] {} = {}/{}\n", p.frozen, p.pet.tier, p.pet.name, p.pet.power, p.pet.health))
        }

        let mut foods = String::new();
        for p in &self.foods {
            foods.push_str(&format!("F:{} [{}] {} \n", p.frozen, p.tier, p.name))
        }
        write!(f, "=====STORE=====\n{}\n\n{}", pets, foods)
    }
}