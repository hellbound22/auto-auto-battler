use std::fmt;

use crate::pet::BPet;

#[derive(Clone, Debug)]
pub struct Crew {
    pub gold: u8,
    pub lifes: u8,
    pub wins: u8,
    pub turn: u8,
    pub friends: Vec<Option<BPet>>,
}

impl Crew {
    pub fn new() -> Self {
        Crew {
            gold: 10,
            lifes: 4,
            wins: 0,
            turn: 1,
            friends: vec![None; 5],
        }
    }

    // TODO: error handle this
    pub fn pay(&mut self, price: u8) -> Result<(), ()> {
        if self.gold < price {
            return Err(())
        }
        self.gold -= price;
        Ok(())
    }

    pub fn sell_pet(&mut self, pet: usize) -> Result<(), ()> {
        if pet >= 5 {
            return Err(());
        }
        let pet = &mut self.friends[pet];

        if pet.is_some() {
            self.gold += pet.as_mut().unwrap().level;
            *pet = None;
        }
        Ok(())
    }

    pub fn add_pet(&mut self, pet: BPet, slot: u8) -> Result<u8, ()> {
        if slot as usize >= self.friends.len() {
            return Err(());
        }
        let curr_pet = &mut self.friends[slot as usize];
        match curr_pet {
            Some(x) => {
                if x.pet.id == pet.pet.id {
                    let curr_pet = curr_pet.as_mut().unwrap();
                    curr_pet.xp += 1;
                    if curr_pet.xp == 2 && curr_pet.level == 1 {
                        curr_pet.xp = 0;
                        curr_pet.level = 2;
                        Ok(1) // mark: when level up
                    } else if curr_pet.xp == 3 && curr_pet.level == 2 {
                        curr_pet.xp = 0;
                        curr_pet.level = 3;
                        Ok(1) // mark: when level up
                    } else {
                        Err(())
                    }
                } else {
                    Err(())
                }
            },
            None => { *curr_pet = Some(pet); Ok(0) }
        }
    }

    pub fn d_team(&self) -> String {
        let mut team = String::new();
        for p in &self.friends {
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

    pub fn _reorder(&mut self, pet_one: usize, pet_two: usize) -> Result<(), ()> {
        if pet_one >= 5 || pet_two >= 5 {
            return Err(());
        }
        let swap_aux = self.friends[pet_one].clone();
        self.friends[pet_one] = self.friends[pet_two].to_owned();
        self.friends[pet_two] = swap_aux;

        Ok(())
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
