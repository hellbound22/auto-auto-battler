use std::fmt;

use rand::seq::SliceRandom; 

use crate::pet::{Pet};

#[derive(Debug)]
pub struct Store {
    pets: Vec<Pet> 
    // TODO: Implement food   
}

impl Store {
    pub fn new(bucket: Vec<Pet>) -> Self {
        Store {
            pets: Store::_internal_roll(bucket, 3, 1.)
        }
    }

    pub fn _roll(&mut self, bucket: Vec<Pet>, slots: u8, tier: f32) {
        self.pets = Store::_internal_roll(bucket, slots, tier);
    }

    fn _internal_roll(bucket: Vec<Pet>, slots: u8, tier: f32) -> Vec<Pet>{
        let tier_bucket: Vec<&Pet> = bucket.iter().filter(|x| x.tier <= tier as i8).collect();

        let mut new_pets: Vec<Pet> = Vec::new();
        for _x in 0..slots {
            new_pets.push(tier_bucket
                .choose(&mut rand::thread_rng())
                .cloned()
                .cloned()
                .unwrap());
        }
        new_pets
    }

    pub fn remove_pet(&mut self, slot: u8) -> Pet {
        self.pets.remove(slot.into())
    } 
}

impl fmt::Display for Store {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pets = String::new();

        for p in &self.pets {
            //if p.pet.id != 0 {
                pets.push_str(&format!("[{}] {} = {}/{}\n", p.tier, p.name, p.power, p.health))
            //}
        }
        write!(f, "=====STORE=====\n{}", pets)
    }
}