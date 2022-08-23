use std::fmt;

use rand::seq::SliceRandom; 

use crate::pet::{Pet, SPet};

#[derive(Debug)]
pub struct Store {
    pets: Vec<SPet> 
    // TODO: Implement food   
}

impl Store {
    pub fn new(bucket: Vec<Pet>) -> Self {
        let tier_bucket: Vec<Pet> = bucket.iter().filter(|x| x.tier <= 1).cloned().collect();
        Store {
            pets: Store::_internal_roll(tier_bucket, 3)
        }
    }

    pub fn tier_up_pet(&mut self, bucket: Vec<Pet>, tier: f32) {
        let tier_bucket: Vec<Pet> = bucket.iter().filter(|x| x.tier == tier as i8).cloned().collect();
        self.pets.push(Store::_internal_roll(tier_bucket, 1)[0].clone());
    }

    pub fn _roll(&mut self, bucket: Vec<Pet>, slots: u8, tier: f32) {
        let frozen: Vec<SPet> = self.pets.iter().filter(|x| x.frozen).cloned().collect();
        let tier_bucket: Vec<Pet> = bucket.iter().filter(|x| x.tier <= tier as i8).cloned().collect();
        self.pets = Store::_internal_roll(tier_bucket, slots);

        for i in 0..frozen.len() {
            self.pets[i] = frozen[i].to_owned();
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

    pub fn freeze_and_unfreeze_pet(&mut self, slot: usize) {
        self.pets[slot].frozen = self.pets[slot].frozen ^ true;
    }

    pub fn remove_pet(&mut self, slot: u8) -> Pet {
        self.pets.remove(slot.into()).into()
    } 
}

impl fmt::Display for Store {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pets = String::new();

        for p in &self.pets {
            //if p.pet.id != 0 {
                pets.push_str(&format!("F:{} [{}] {} = {}/{}\n", p.frozen, p.pet.tier, p.pet.name, p.pet.power, p.pet.health))
            //}
        }
        write!(f, "=====STORE=====\n{}", pets)
    }
}