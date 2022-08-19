use std::fs::File;
use std::io::prelude::*;
use std::fmt;

mod store;
mod pet;

use store::Store;
use pet::{BPet, Pet};

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



fn main() {
    let mut game = Game::new("std");

    println!("{}", game);
    //dbg!(game.crew);
}
