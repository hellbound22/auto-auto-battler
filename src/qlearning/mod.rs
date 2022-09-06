use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

use rand::prelude::*;

use crate::pet::BPet;
use crate::pet::Pet;
use crate::food::Food;


// Curr_friends -> Shop_friends -> Action -> calculate 
pub struct Brain {
    pub q_table: HashMap<usize, HashMap<(u8, u8, u8), f64>>,
    pub state_table: Vec<(Vec<Option<BPet>>, Vec<Option<BPet>>)>,
    pub action_map: Vec<(u8, u8, u8)>,
}

impl Brain {
    pub fn new() -> Self {

        let file = File::open("qtables/std/action.table").unwrap();
        let buf_reader = BufReader::new(file);

        let mut map: Vec<(u8, u8, u8)> = Vec::new();

        for line in buf_reader.lines() {
            let line = line.unwrap();
            let s = line.split(',').collect::<Vec<&str>>();

            map.push((s[0].parse().unwrap(), s[1].parse().unwrap(), s[2].parse().unwrap()));

        }

        Self {
            q_table: HashMap::new(),
            state_table: Vec::new(),
            action_map: map,
        }
    }

    pub fn get_action_map_random(&self) -> HashMap<(u8, u8, u8), f64> {
        let mut map = HashMap::new();

        let mut rng = rand::thread_rng();

        for pair in &self.action_map {
            map.insert(pair.clone(), rng.gen());
        }

        map
    }

    pub fn process(&mut self, game: &mut crate::game::Game) {

        let discount_factor = 1.0;
        let alpha = 0.6;
        let epsilon = 0.1;

        let actions = self.get_action_map_random();
  
        let mut max_reward = 0.;

        let mut num_actions = 0;
        let max_actions = 100;

        loop {
            let state = (game.get_state().to_owned(), game.get_shop().to_owned());

            let index = match self.state_table.binary_search(&state) {
                Ok(x) => { x },
                Err(x) => {
                    self.state_table.push(state.clone());
                    x
                }
            };
            
            let q = self.q_table.entry(index).or_insert(actions.clone());

            let mut max = 0.;
            let mut best_next_action = (0,0,0);
            //q.iter().for_each(|x| if x.1 > &max {max = *x.1; best_next_action = x.0.clone()});
            for (i, x) in q.iter().enumerate() {
                if x.1 > &max {max = *x.1; best_next_action = x.0.clone()}
                //dbg!(x.1);
                //dbg!(max);
            }
            

            let actual_q = q.get_mut(&(0,0,0)).unwrap();
            if best_next_action == (0,0,0) {
                let mut rng = rand::thread_rng();
                best_next_action = *self.action_map.choose(&mut rng).unwrap();
                println!("Best next action is doing nothing");
            }

            dbg!(best_next_action);
            
            if game.crew.gold == 0 {
                best_next_action = (99,0,0);    
            }

            if game.take_action(best_next_action).is_err() {
                
            } else {
                num_actions += 1;
            }

            println!("=================================================\n{}", game.crew);
            

            let reward = game.reward() as f64 / game.crew.turn as f64;
            println!("{}", reward);

            if reward > max_reward {max_reward = reward}

            let td_target = reward + discount_factor * max;
            let td_delta = td_target - *actual_q;

            *actual_q += alpha + td_delta;

            if num_actions == max_actions {
                break;
            }
        }
    }

    pub fn get_best_actions(&mut self, game: &mut crate::game::Game) -> ((u8, u8, u8), f64) {
        let state = (game.get_state().to_owned(), game.get_shop().to_owned());

        let index = match self.state_table.binary_search(&state) {
            Ok(x) => { x },
            Err(x) => {
                self.state_table.push(state.clone());
                x
            }
        };
        let actions = self.get_action_map_random();
        
        let q = self.q_table.entry(index).or_insert(actions.clone());

        let mut max = std::f64::MAX;
        let mut best_next_action = (0,0,0);
        q.iter().for_each(|x| if x.1 > &max {max = *x.1; best_next_action = x.0.clone()});

        (best_next_action, max)
    }

}
