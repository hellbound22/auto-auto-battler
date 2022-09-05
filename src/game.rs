use std::fmt;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::collections::HashMap;

use crate::crew::Crew;
use crate::food::Food;
use crate::pet::{BPet, Pet};
use crate::store::Store;
use crate::{battle::*, util};

#[derive(Debug)]
pub struct Game {
    pub crew: Crew,
    store: Store,
}

impl Game {
    pub fn new(pack: &str) -> Self {
        let mut file = File::open(format!("./packs/{}.pets", pack)).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let split = contents.split("\n");
        let lines = split.collect::<Vec<&str>>();

        let pet_lines = lines.iter().filter(|x| x.chars().last() == Some('p'));
        let food_lines = lines.iter().filter(|x| x.chars().last() == Some('f'));

        let mut pet_bucket: Vec<Pet> = Vec::new();
        for line in pet_lines {
            pet_bucket.push(Pet::new(line));
        }

        let mut food_bucket: Vec<Food> = Vec::new();
        for line in food_lines {
            food_bucket.push(Food::new(line));
        }

        Game {
            crew: Crew::new(),
            store: Store::new(pet_bucket.clone(), food_bucket.clone()),
        }
    }

    pub fn get_buckets(&self) -> (HashMap<u8, Pet>, &Vec<Food>) {
        let mut hm = HashMap::new();

        for pet in &self.store.pet_bucket {
            hm.insert(pet.id.clone() as u8, pet.clone());
        }
        (hm, &self.store.food_bucket)
    }

    pub fn get_state(&self) -> &Vec<Option<BPet>> {
        &self.crew.friends
    }

    pub fn bot_random(&mut self) {
        self.roll_shop(0);
        for x in 0..3 {
            self.buy_pet(0, x);
        }
    }

    pub fn buy_pet(&mut self, slot: u8, team_slot: u8) -> Result<(), ()>{
        if team_slot > 5 {
            return Err(());
        }

        let pet = match self.store.remove_pet(slot) { Ok(pet) => pet, Err(x) => return Err(x)};

        let b = BPet {
            pet,
            xp: 0,
            level: 1,
            food: None, // WARN: Some pets come with foods
        };

        match self.crew.add_pet(b, team_slot) {
            Ok(0) => {}
            Ok(1) => {
                let mut tier = (self.crew.turn as f32 / 2.).ceil();
                if tier < 6. {
                    tier += 1.;
                }

                self.store.tier_up_pet(tier);
            }
            Ok(_) => {}
            Err(_) => {}
        }

        self.crew.pay(3)
    }

    pub fn buy_food(&mut self, shop_slot: u8, pet_slot: u8) -> Result<(), ()> {
        if pet_slot as usize >= self.crew.friends.len() {
            return Err(());
        }
        // WARN some foods have team and store wide effects
        let food = self.store.remove_food(shop_slot)?;

        match food.type_effect {
            0 => {
                let pet = self.crew.friends[pet_slot as usize].as_mut();
                if let Some(pet) = pet {
                    pet.switch_food(food);
                    self.crew.pay(3)
                } else {
                    Err(())
                }
            }
            _ => {
                Err(())
            }
        }
        
    }

    pub fn battle(&mut self, mut enemy_crew: Crew) -> u8 {
        let mut my_crew = self.crew.clone();

        let mut my_index = 0;
        let mut enemy_index = 0;

        loop {
            //println!("MY:\n{}", my_crew.d_team());
            //println!("ENEMY:\n{}", enemy_crew.d_team());

            //let mut line = String::new();
            //let _b1 = std::io::stdin().read_line(&mut line).unwrap();

            match check_win_condition(&my_crew, &enemy_crew) {
                3 => {}
                x => return x,
            }

            // TODO: change how the attacker is chosen
            let my_attacker = &mut my_crew.friends[my_index];
            let enemy_attacker = &mut enemy_crew.friends[enemy_index];

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

    pub fn swap_pet(&mut self, pet_one: usize, pet_two: usize) -> Result<(), ()> {
        self.crew._reorder(pet_one, pet_two)
    }

    pub fn take_action(&mut self, action: (u8, u8, u8)) -> Result<(), ()> {
        match action.0 {
            // buy mode
            1 => {
                self.buy_pet(action.1, action.2)
            }
            // swap mode
            2 => {
                self.swap_pet(action.1 as usize, action.2 as usize)
            }
            // Roll shop
            3 => {
                self.roll_shop(1)
            }
            // Sell pet
            4 => {
                self.crew.sell_pet(action.1 as usize) // TODO: Check for error
            }
            // Freeze pet
            5 => {
                self.store.freeze_and_unfreeze_pet(action.2 as usize)
            }
            // buy food
            6 => {
                self.buy_food(action.1, action.2)
            }
            // freeze food
            7 => {
                self.store.freeze_and_unfreeze_food(action.1 as usize)
            }
            // end turn mode
            99 => { 
                self.crew.gold = 10;
                self.crew.turn += 1;
                Ok(()) 
            },
            _ => { Err(()) }
        }
    }

    pub fn reward(&self) -> i8 {
        let mut x = 0;
        self.crew.friends.iter().for_each(|e| x += e.clone().unwrap_or_default().pet.health + e.clone().unwrap_or_default().pet.power);
        x
    }

    pub fn get_shop(&self) -> Vec<Option<BPet>> {
        self.store.pets.iter().map(|x| Some(x.clone().unwrap_or_default().into())).collect()
    }

    pub fn roll_shop(&mut self, price: u8) -> Result<(), ()> {
        let tier = (self.crew.turn as f32 / 2.).ceil();
        self.store.roll(self.crew.turn, tier);
        self.crew.pay(price)
    }

    pub fn game_loop(&mut self, bot: Crew) {
        // bot prob has to be reference
        loop {
            // TODO: control usage of this block with a arg
            loop {
                println!("{}", self);
                println!("=====Options=====\n(1) Buy mode\n(2) Swap mode\n(3) Roll shop\n(4) Sell pet\n(5) Freeze/unfreeze pet\n(6) Buy food\n(7) Freeze/unfreeze food\n(99) End turn\n");
                let x = util::wait_for_input();

                match x {
                    // buy mode
                    1 => {
                        println!("What shop pet do you want?");
                        let shop_pet: u8 = util::wait_for_input() - 1;
                        println!("What team slot do you want to put that pet?");
                        let team_slot: u8 = util::wait_for_input() - 1;
                        self.buy_pet(shop_pet, team_slot);
                    }
                    // swap mode
                    2 => {
                        println!("Insert position of pet one");
                        let pet_one: u8 = util::wait_for_input() - 1;
                        println!("Insert position of pet two");
                        let pet_two: u8 = util::wait_for_input() - 1;
                        self.swap_pet(pet_one as usize, pet_two as usize);
                    }
                    // Roll shop
                    3 => {
                        self.roll_shop(1);
                    }
                    // Sell pet
                    4 => {
                        println!("What pet do you want to sell?");
                        let pet = util::wait_for_input() - 1;
                        self.crew.sell_pet(pet as usize);
                    }
                    // Freeze pet
                    5 => {
                        println!("What pet do you want to freeze/unfreeze?");
                        let pet = util::wait_for_input() - 1;
                        self.store.freeze_and_unfreeze_pet(pet as usize);
                    }
                    // buy food
                    6 => {
                        println!("What food do you want?");
                        let shop_pet: u8 = util::wait_for_input() - 1;
                        println!("What team slot do you want to put that food?");
                        let team_slot: u8 = util::wait_for_input() - 1;
                        self.buy_food(shop_pet, team_slot);
                    }
                    // freeze food
                    7 => {
                        println!("What food do you want to freeze/unfreeze?");
                        let food = util::wait_for_input() - 1;
                        self.store.freeze_and_unfreeze_food(food as usize);
                    }
                    // end turn mode
                    99 => break,
                    _ => {}
                }
            }

            match self.battle(bot.clone()) {
                //Note: we dont need to clone here, just for debuging
                0 => {
                    println!("DRAW!!!!")
                }
                1 => {
                    self.crew.wins += 1;
                    println!("WIN!!!!")
                }
                2 => {
                    // TODO: check if lives are 0
                    self.crew.lifes -= 1;
                    println!("LOST!!!!")
                }
                _ => {
                    println!("ERROR")
                }
            }

            self.crew.gold = 10;
            self.crew.turn += 1;
            self.roll_shop(0);
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n\n{}", self.crew, self.store)
    }
}
