use std::fmt;
use std::fs::File;
use std::io::prelude::*;

use crate::crew::Crew;
use crate::pet::{BPet, Pet};
use crate::store::Store;

#[derive(Debug)]
pub struct Game {
    pub crew: Crew,
    store: Store,
    _pack: Vec<Pet>,
}

impl Game {
    pub fn new(pack: &str) -> Self {
        let mut file = File::open(format!("./packs/{}.pets", pack)).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let split = contents.split("\n");
        let lines = split.collect::<Vec<&str>>();

        let mut bucket: Vec<Pet> = Vec::new();

        for line in lines {
            bucket.push(Pet::new(line));
        }

        Game {
            crew: Crew::new(),
            store: Store::new(bucket[1..].to_vec()),
            _pack: bucket,
        }
    }

    pub fn bot_random(&mut self) {
        for x in 0..3 {
            self.buy_pet(0, x);
        }
    }

    pub fn buy_pet(&mut self, slot: u8, team_slot: u8) {
        if team_slot > 5 {
            return;
        }

        let b = BPet {
            pet: self.store.remove_pet(slot),
            xp: 0,
            level: 1,
            alive: true,
        };
        self.crew.add_pet(b, team_slot);

        self.crew.pay_for_pet(3);
    }

    pub fn battle(&mut self, mut enemy_crew: Crew) {
        let mut my_crew = self.crew.clone();

        let mut my_index = 0;
        let mut enemy_index = 0;

        loop {
            println!("MY:\n{}", my_crew.d_team());
            println!("ENEMY:\n{}", enemy_crew.d_team());

            let mut line = String::new();
            let _b1 = std::io::stdin().read_line(&mut line).unwrap();
            
            // check for win condition
            let mut my_alive = 0;
            let mut enemy_alive = 0;
            for pet in &my_crew.team {
                if pet.is_none() {
                    continue;
                }
                if pet.as_ref().unwrap().pet.health > 0 {
                    my_alive += 1;
                }
            }

            for pet in &enemy_crew.team {
                if pet.is_none() {
                    continue;
                }
                if pet.as_ref().unwrap().pet.health > 0 {
                    enemy_alive += 1;
                }
            }

            if my_alive == 0 && enemy_alive == 0 {
                println!("EMPATE!!!!!!!!!!!!!");
                break;
            } else if my_alive > 0 && enemy_alive == 0 {
                println!("VOCÊ GANHOU!!!!!!!!!!!!!");
                break;
            } else if my_alive == 0 && enemy_alive > 0  {
                println!("VOCÊ PERDEU!!!!!!!!!!!!!");
                break;
            } else {
                //println!("TURN");
            }
            
            let my_attacker = &mut my_crew.team[my_index];
            let enemy_attacker = &mut enemy_crew.team[enemy_index];

            // BUG
            if my_attacker.is_none() {
                my_index += 1;
                continue;
            }
            if enemy_attacker.is_none() {
                enemy_index += 1;
                continue;
            }

            //println!("{} - ({}/{})", my_attacker.as_ref().unwrap().pet.name, my_attacker.as_ref().unwrap().pet.power, my_attacker.as_ref().unwrap().pet.health);
            //println!("{} - ({}/{})", enemy_attacker.as_ref().unwrap().pet.name, enemy_attacker.as_ref().unwrap().pet.power, enemy_attacker.as_ref().unwrap().pet.health);

            my_attacker.as_mut().unwrap().pet.health = my_attacker.as_ref().unwrap().pet.health
                - enemy_attacker.as_ref().unwrap().pet.power;
            enemy_attacker.as_mut().unwrap().pet.health =
                enemy_attacker.as_ref().unwrap().pet.health
                    - my_attacker.as_ref().unwrap().pet.power;

            if my_attacker.as_ref().unwrap().pet.health <= 0 {
                my_index += 1;
            }
            if enemy_attacker.as_ref().unwrap().pet.health <= 0 {
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
