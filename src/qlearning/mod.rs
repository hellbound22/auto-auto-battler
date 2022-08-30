use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

use crate::pet::BPet;
use crate::pet::Pet;
use crate::food::Food;

pub struct StateTable {
    map: HashMap<Option<BPet>, usize>,
    pet_bucket: HashMap<u8, Pet>,
    buffer: String,
}

impl StateTable {
    pub fn new(meta_path: &str, state_path: &str, buckets: (HashMap<u8, Pet>, &Vec<Food>)) -> Self {
        let file = File::open(meta_path).unwrap();
        let buf_reader = BufReader::new(file);

        let mut map: HashMap<Option<BPet>, usize> = HashMap::new();

        for line in buf_reader.lines() {
            let line = line.unwrap();
            let s = line.split(',').collect::<Vec<&str>>();
            
            let pet_id: u8 = s[0].parse().unwrap();

            let ref_pet: Option<BPet>;
            if pet_id == 0 {
                ref_pet = None;
            } else {
                ref_pet = Some(buckets.0.get(&pet_id).cloned().unwrap().into());
            } 
            
            map.insert(ref_pet, s[1].parse().unwrap());
        }

        Self {
            map,
            pet_bucket: buckets.0,
            buffer: state_path.to_owned(),
        }
    }

    fn get_state_table_index_range(&self, pet: Option<BPet>) -> (usize, usize) {
        let p = *self.map.get(&pet).unwrap(); // TODO: If pet is none use default
        dbg!(p);
        let next = self.map.get(&self.pet_bucket.get(&((pet.unwrap().pet.id) as u8)).map(|x| x.clone().into())).unwrap();
        (p, *next)
    }

    pub fn get_state(&self, crew: Vec<Option<BPet>>) {
        dbg!(&crew);
        let index_range = self.get_state_table_index_range(crew[0].as_ref().map(|x| x.to_owned()));

        let file = File::open(self.buffer.clone()).unwrap();
        let buf_reader = BufReader::new(file);
        let line_iter = buf_reader.lines().nth(index_range.0);
        
        dbg!(line_iter);
    }

}