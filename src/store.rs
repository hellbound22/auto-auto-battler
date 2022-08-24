use std::fmt;

use rand::seq::SliceRandom; 

use crate::pet::{Pet, SPet};
use crate::food::Food;

#[derive(Debug, Default)]
pub struct Store {
    pets: Vec<Option<SPet>>,
    pet_bucket: Vec<Pet>,
    pet_slots: u8,
    foods: Vec<Option<Food>>, 
    food_bucket: Vec<Food>, 
    food_slots: u8,
    // TODO: Implement food   
}

impl Store {
    pub fn new(pet_bucket: Vec<Pet>, food_bucket: Vec<Food>) -> Self {
        //let pet_tier_bucket: Vec<Pet> = pet_bucket.iter().filter(|x| x.tier <= 1).cloned().collect();
        //let food_tier_bucket: Vec<Food> = food_bucket.iter().filter(|x| x.tier <= 1).cloned().collect();
        Store {
            pets: vec![Default::default(); 3],
            pet_bucket,
            pet_slots: 3,
            foods: vec![Default::default(); 1],
            food_bucket,
            food_slots: 1,
        }
    }

    pub fn add_slot(&mut self, class: char) {
        match class {
            'p' => { self.pet_slots += 1 },
            'f' => { self.food_slots += 1 },
            _ => {} //TODO: handle this as error
        }
    }

    pub fn tier_up_pet(&mut self, tier: f32) {
        let tier_bucket: Vec<Pet> = self.pet_bucket.iter().filter(|x| x.tier == tier as i8).cloned().collect();
        self.pets.push(Some(self._internal_roll(tier_bucket)[0].clone()));
    }

    pub fn roll(&mut self, turn: u8, tier: f32) {
        match turn {
            3 => self.add_slot('f'),
            5 => self.add_slot('p'),
            9 => self.add_slot('p'),
            _ => {} //TODO: handle this as error
        }

        let frozen_pets: Vec<Option<SPet>> = self.pets.iter().filter(|x| x.is_some()).filter(|x| x.as_ref().unwrap().frozen).cloned().collect();
        let tier_bucket: Vec<Pet> = self.pet_bucket.iter().filter(|x| x.tier <= tier as i8).cloned().collect();
        self.pets = self._internal_roll(tier_bucket).iter().map(|p| Some(p.to_owned())).collect();

        for i in 0..frozen_pets.len() {
            self.pets[i] = frozen_pets[i].to_owned();
        }

        let frozen_food: Vec<Option<Food>> = self.foods.iter().filter(|x| x.is_some()).filter(|x| x.as_ref().unwrap().frozen).cloned().collect();
        let tier_bucket: Vec<Food> = self.food_bucket.iter().filter(|x| x.tier <= tier as i8).cloned().collect();
        self.foods = self._internal_food_roll(tier_bucket).iter().map(|p| Some(p.to_owned())).collect();

        for i in 0..frozen_food.len() {
            self.foods[i] = frozen_food[i].to_owned();
        }
    }

    fn _internal_roll(&self, bucket: Vec<Pet>) -> Vec<SPet> {
        let mut new_pets: Vec<Pet> = Vec::new();
        for _x in 0..self.pet_slots {
            new_pets.push(bucket
                .choose(&mut rand::thread_rng())
                .cloned()
                .unwrap());
        }
        new_pets.iter().map(|x| x.into()).collect()
    }
    // TODO: Join implementation of pet/food rolls and freezes
    fn _internal_food_roll(&self, bucket: Vec<Food>) -> Vec<Food> {
        let mut new_food: Vec<Food> = Vec::new();
        for _x in 0..self.food_slots {
            new_food.push(bucket
                .choose(&mut rand::thread_rng())
                .cloned()
                .unwrap());
        }
        new_food
    }

    pub fn freeze_and_unfreeze_pet(&mut self, slot: usize) {
        self.pets[slot].as_mut().unwrap().frozen = self.pets[slot].as_ref().unwrap().frozen ^ true;
    }

    pub fn freeze_and_unfreeze_food(&mut self, slot: usize) {
        self.foods[slot].as_mut().unwrap().frozen = self.foods[slot].as_ref().unwrap().frozen ^ true;
    }

    pub fn remove_pet(&mut self, slot: u8) -> Pet {
        self.pets.remove(slot.into()).unwrap().into()
    } 

    pub fn remove_food(&mut self, slot: u8) -> Food {
        self.foods.remove(slot.into()).unwrap().into()
    } 
}

impl fmt::Display for Store {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pets = String::new();
        for p in &self.pets {
            if let Some(p) = p {
                pets.push_str(&format!("F:{} [{}] {} = {}/{}\n", p.frozen, p.pet.tier, p.pet.name, p.pet.power, p.pet.health))    
            }
        }

        let mut foods = String::new();
        for p in &self.foods {
            if let Some(f) = p {
                foods.push_str(&format!("F:{} [{}] {} \n", f.frozen, f.tier, f.name))
            }
            
        }
        write!(f, "=====STORE=====\n{}\n\n{}", pets, foods)
    }
}