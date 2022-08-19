use std::fs::File;
use std::io::prelude::*;
use std::fmt;

use crate::store::Store;
use crate::pet::{BPet, Pet};
use crate::crew::Crew;

#[derive(Debug)]
pub struct Game {
    crew: Crew,
    store: Store,
    pack: Vec<Pet>,
}

impl Game {
    pub fn new(pack: &str) -> Self {
        let mut file = File::open(format!("./{}.pets", pack)).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        
    
        let mut split = contents.split("\n");
        let lines = split.collect::<Vec<&str>>();
    
        let mut bucket: Vec<Pet> = Vec::new();
    
        for line in lines {
            bucket.push(Pet::new(line));
        }

        Game { crew: Crew::new(), store: Store::new(bucket[1..].to_vec()), pack: bucket }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n\n{}", self.crew, self.store)
    }
}
