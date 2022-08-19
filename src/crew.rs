use std::fmt;

use crate::pet::{BPet};

#[derive(Debug)]
pub struct Crew {
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
