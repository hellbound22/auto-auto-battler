use std::fmt;

use crate::pet::BPet;

#[derive(Clone, Debug)]
pub struct Crew {
    gold: u8,
    lifes: u8,
    wins: u8,
    turn: u8,
    pub team: Vec<Option<BPet>>,
}

impl Crew {
    pub fn new() -> Self {
        Crew {
            gold: 10,
            lifes: 10,
            wins: 0,
            turn: 0,
            team: vec![None; 5],
        }
    }

    pub fn pay_for_pet(&mut self, price: u8) {
        self.gold = self.gold - price;
    }

    pub fn add_pet(&mut self, pet: BPet, slot: u8) {
        let curr_pet = &mut self.team[slot as usize];
        if curr_pet.is_none() {
            *curr_pet = Some(pet);
        }
    }

    pub fn d_team(&self) -> String {
        let mut team = String::new();
        for p in &self.team {
            if p.is_some() {
                team.push_str(&format!(
                    "[{}] {} ({}|{}) = {}/{}\n",
                    p.as_ref().unwrap().pet.tier,
                    p.as_ref().unwrap().pet.name,
                    p.as_ref().unwrap().level,
                    p.as_ref().unwrap().xp,
                    p.as_ref().unwrap().pet.power,
                    p.as_ref().unwrap().pet.health
                ))
            }
        }
        team
    }
}

impl fmt::Display for Crew {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Gold: {}\nLifes: {}\nWins: {}\nTurn:{}\n=====TEAM=====\n{}",
            self.gold, self.lifes, self.wins, self.turn, self.d_team()
        )
    }
}
