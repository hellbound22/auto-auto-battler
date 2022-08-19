use std::fmt;

use crate::pet::{BPet};

#[derive(Clone, Debug)]
pub struct Crew {
    gold: u8,
    lifes: u8,
    wins: u8,
    turn: u8,
    pub team: Vec<BPet>,
}

impl Crew {
    pub fn new() -> Self {
        Crew { gold: 10, lifes: 10, wins: 0, turn: 0, team: vec![BPet::default(); 5]}
    }

    pub fn pay_for_pet(&mut self, price:u8) {
        self.gold = self.gold - price;
    }

    pub fn add_pet(&mut self, pet: BPet, slot: u8) {
        let curr_pet = &mut self.team[slot as usize];
        if curr_pet.pet.id == 0 {
            *curr_pet = pet;
        }
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
