use std::fmt;

use rand::seq::SliceRandom; 

use crate::actors::pet::{Pet, SPet};
use crate::actors::food::Food;

#[derive(Debug, Default)]
pub struct Store {
    pub pets: Vec<Option<SPet>>,
    pub pet_bucket: Vec<Pet>,
    pet_slots: u8,
    foods: Vec<Option<Food>>, 
    pub food_bucket: Vec<Food>, 
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
                .unwrap()); // BUG
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

    pub fn freeze_and_unfreeze_pet(&mut self, slot: usize) -> Result<(), ()> {
        let target_pet = if let Some(pet) = self.pets.get_mut(slot) {
            pet
        } else {
            return Err(());
        };

        let target_pet = if let Some(pet) = target_pet {
            pet
        } else {
            return Err(());
        };

        target_pet.frozen = target_pet.frozen ^ true;
        Ok(())
    }

    pub fn freeze_and_unfreeze_food(&mut self, slot: usize) -> Result<(), ()> {
        let target_food = if let Some(food) = self.foods.get_mut(slot) {
            food
        } else {
            return Err(());
        };

        let target_food = if let Some(food) = target_food {
            food
        } else {
            return Err(());
        };

        target_food.frozen = target_food.frozen ^ true;
        Ok(())
    }

    pub fn remove_pet(&mut self, slot: u8) -> Result<Pet, ()> {
        if self.pets.len() <= slot as usize {
            return Err(());
        }
        match self.pets.remove(slot.into()) {
            Some(pet) => { Ok(pet.into()) },
            None => { Err(()) },
        }
    } 

    pub fn remove_food(&mut self, slot: u8) -> Result<Food, ()> {
        if self.foods.len() <= slot as usize {
            return Err(());
        }
        match self.foods.remove(slot.into()) {
            Some(food) => { Ok(food.into()) },
            None => { Err(()) },
        }
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