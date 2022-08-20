use std::fmt;
use std::fs::File;
use std::io::prelude::*;

use crate::crew::Crew;
use crate::pet::{BPet, Pet};
use crate::store::Store;
use crate::battle::*;

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
            store: Store::new(bucket.clone()),
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

    pub fn battle(&mut self, mut enemy_crew: Crew) -> u8 {
        let mut my_crew = self.crew.clone();

        let mut my_index = 0;
        let mut enemy_index = 0;

        loop {
            println!("MY:\n{}", my_crew.d_team());
            println!("ENEMY:\n{}", enemy_crew.d_team());

            //let mut line = String::new();
            //let _b1 = std::io::stdin().read_line(&mut line).unwrap();
            
            match check_win_condition(&my_crew, &enemy_crew) {
                3 => {},
                x => { return x },
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

            headon_attack(my_attacker, enemy_attacker);
            headon_attack(enemy_attacker, my_attacker);

            if check_life(my_attacker) {
                my_index += 1;
            }
            if check_life(enemy_attacker) {
                enemy_index += 1;
            }
            //break;
        }

        
    }

    pub fn swap_pet(&mut self, pet_one: usize, pet_two: usize) {
        self.crew._reorder(pet_one, pet_two);
    }

    pub fn roll_shop(&mut self, price: u8) {
        self.store._roll(self._pack.clone(), 3); // TODO: dynamic slots
        self.crew.pay_for_shop_roll(price);
    }

    pub fn game_loop(&mut self, bot: Crew) { // bot prob has to be reference
        match self.battle(bot) {
            0 => { println!("DRAW!!!!") },
            1 => { 
                self.crew.wins += 1;
                println!("WIN!!!!") 
            },
            2 => {
                self.crew.lifes -= 1;
                println!("LOST!!!!") 
            },
            _ => { println!("ERROR") }
        }
    
        self.crew.gold = 10;
        self.crew.turn += 1;
        self.roll_shop(0);

        //TODO: manage
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n\n{}", self.crew, self.store)
    }
}
