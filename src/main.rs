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

        Game { crew: Crew::new(), store: Store::new(bucket[1..].to_vec()), pack: bucket }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n\n{}", self.crew, self.store)
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
        Crew { gold: 10, lifes: 10, wins: 0, turn: 0, team: vec![BPet::default(); 5]}
    }
}

impl fmt::Display for Crew {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut team = String::new();

        for p in &self.team {
            //if p.pet.id != 0 {
                team.push_str(&format!("[{}] {} ({}|{}) = {}/{}\n", p.pet.tier, p.pet.name, p.level, p.xp, p.pet.power, p.pet.health))
            //}
        }
        write!(f, "Gold: {}\nLifes: {}\nWins: {}\nTurn:{}\n=====TEAM=====\n{}", 
            self.gold, self.lifes, self.wins, self.turn, team)
    }
}

#[derive(Clone, Debug)]
struct BPet {
    pet: Pet,
    level: u8,
    xp: u8,
    // TODO: Implement food
}

impl Default for BPet {
    fn default() -> Self {
        Self {
            pet: Pet::default(),
            level: 1,
            xp: 0,
        }
    }
}

#[derive(Clone, Debug)]
struct Pet {
    id: i8,
    tier: i8,
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

impl Default for Pet {
    fn default() -> Self {
        Self {
            id: 0,
            tier: 0,
            name: "Empty".to_owned(),
            power: 0,
            health: 0,
        }
    }
}

fn main() {
    let mut game = Game::new("std");

    println!("{}", game);
    //dbg!(game.crew);
}
