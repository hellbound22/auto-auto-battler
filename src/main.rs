use std::fs::File;
use std::io::prelude::*;
use std::fmt;

use rand::prelude::*;
use rand::seq::SliceRandom; 

#[derive(Debug)]
struct Game {
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

        Game { crew: Crew::new(), store: Store::new(bucket.clone()), pack: bucket }
    }
}

#[derive(Debug)]
struct Store {
    pets: Vec<Pet> 
    // TODO: Implement food   
}

impl Store {
    pub fn new(bucket: Vec<Pet>) -> Self {
        Store {
            pets: Store::_internal_roll(bucket, 3)
        }
    }

    pub fn roll(&mut self, bucket: Vec<Pet>, slots: u8) {
        self.pets = Store::_internal_roll(bucket, slots);
    }

    fn _internal_roll(bucket: Vec<Pet>, slots: u8) -> Vec<Pet>{
        let mut new_pets: Vec<Pet> = Vec::new();
        for x in 0..slots {
            new_pets.push(bucket
                .choose(&mut rand::thread_rng())
                .cloned()
                .unwrap());
        }
        new_pets
    }
}

#[derive(Debug)]
struct Crew {
    gold: u8,
    lifes: u8,
    wins: u8,
    turn: u8,
    team: Vec<BPet>,
}

impl Crew {
    pub fn new() -> Self {
        Crew { gold: 10, lifes: 10, wins: 0, turn: 0, team: Vec::new()}
    }
}

impl fmt::Display for Crew {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Gold: {}\nLifes: {}\nWins: {}, Turn:{}", self.gold, self.lifes, self.wins, self.turn)
    }
}

#[derive(Debug)]
struct BPet {
    pet: Pet,
    tier: u8,
    xp: u8,
    // TODO: Implement food
}

#[derive(Clone, Debug)]
struct Pet {
    id: u8,
    tier: u8,
    name: String,
    power: i8,
    health: i8,
}

impl Pet {
    pub fn new(line: &str) -> Self {
        let mut split = line.split(",");
        let vec = split.collect::<Vec<&str>>();

        Pet { id: vec[0].parse().unwrap(), tier: vec[1].parse().unwrap(), name: vec[2].to_string(), power: vec[3].parse().unwrap(), health: vec[4].parse().unwrap() }
    }
}

fn main() {
    let mut game = Game::new("std");

    dbg!(game.store);
}
