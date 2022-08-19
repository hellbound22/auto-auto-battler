use std::fs::File;
use std::io::prelude::*;
use std::fmt;

use crate::store::Store;
use crate::pet::{BPet, Pet};
use crate::crew::Crew;

#[derive(Debug)]
pub struct Game {
    pub crew: Crew,
    store: Store,
    pack: Vec<Pet>,
}

impl Game {
    pub fn new(pack: &str) -> Self {
        let mut file = File::open(format!("./packs/{}.pets", pack)).unwrap();
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

    pub fn bot_random(&mut self) {
        for x in 0..3 {
            self.buy_pet(0, x);
        }
    }

    pub fn buy_pet(&mut self, slot: u8, team_slot: u8) {
        if team_slot > 5 {
            return
        }

        let mut b = BPet{
            pet: self.store.remove_pet(slot),
            xp: 0,
            level: 1,
        };
        self.crew.add_pet(b, team_slot);

        self.crew.pay_for_pet(3);
        
    }

    pub fn battle(&mut self, mut enemy_crew: Crew) {
        let mut my_crew = self.crew.clone();

        let mut my_index = 0;
        let mut enemy_index = 0;

        loop {
            if my_index >= 5 && enemy_index >= 5 {
                println!("EMPATE!!!!!!!!!!!!!");
                break
            } else if my_index < 5 && enemy_index >= 5 {
                println!("VOCÊ GANHOU!!!!!!!!!!!!!");
                break
            } else if my_index >= 5 && enemy_index < 5 {
                println!("VOCÊ PERDEU!!!!!!!!!!!!!");
                break
            } else {
                //println!("TURN");
            }
            
            let mut my_attacker = &mut my_crew.team[my_index];
            let mut enemy_attacker = &mut enemy_crew.team[enemy_index];

            if my_attacker.pet.id == 0 {
                my_index += 1;
                continue;
            }
            if enemy_attacker.pet.id == 0 {
                enemy_index += 1;
                continue;
            }

            my_attacker.pet.health = my_attacker.pet.health - enemy_attacker.pet.power;
            enemy_attacker.pet.health = enemy_attacker.pet.health - my_attacker.pet.power;

            //println!("MY: \n{}", my_crew);
            //println!("\n\nENEMY: \n{}", enemy_crew);

            if my_attacker.pet.health <= 0 {
                my_index += 1;
            }
            if enemy_attacker.pet.health <= 0 {
                enemy_index += 1;
            }
            //break;
        }
        println!("MY: \n{}", my_crew);
        println!("\n\nENEMY: \n{}", enemy_crew);

    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n\n{}", self.crew, self.store)
    }
}
