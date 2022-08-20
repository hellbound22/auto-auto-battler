use std::fmt;

use crate::pet::BPet;

#[derive(Clone, Debug)]
pub struct Crew {
    pub gold: u8,
    pub lifes: u8,
    pub wins: u8,
    pub turn: u8,
    pub team: Vec<Option<BPet>>,
}

impl Crew {
    pub fn new() -> Self {
        Crew {
            gold: 10,
            lifes: 4,
            wins: 0,
            turn: 0,
            team: vec![None; 5],
        }
    }

    pub fn pay_for_shop_roll(&mut self, price: u8) {
        self.gold -= price;
    }

    pub fn pay_for_pet(&mut self, price: u8) {
        self.gold = self.gold - price;
    }

    pub fn add_pet(&mut self, pet: BPet, slot: u8) -> Result<(), ()> {
        let curr_pet = &mut self.team[slot as usize];
        match curr_pet {
            Some(x) => {
                if x.pet.id == pet.pet.id {
                    let curr_pet = curr_pet.as_mut().unwrap();
                    curr_pet.xp += 1;
                    if curr_pet.xp == 2 && curr_pet.level == 1 {
                        curr_pet.xp = 0;
                        curr_pet.level = 2;
                        Ok(())
                    } else if curr_pet.xp == 3 && curr_pet.level == 2 {
                        curr_pet.xp = 0;
                        curr_pet.level = 3;
                        Ok(())
                    } else {
                        Err(())
                    }
                } else {
                    Err(())
                }
            },
            None => { *curr_pet = Some(pet); Ok(()) }
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
            } else {
                team.push_str("None\n");
            }
        }
        team
    }

    pub fn _reorder(&mut self, pet_one: usize, pet_two: usize) {
        let swap_aux = self.team[pet_one].clone();
        self.team[pet_one] = self.team[pet_two].to_owned();
        self.team[pet_two] = swap_aux;
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
