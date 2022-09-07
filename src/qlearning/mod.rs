use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

use rand::prelude::*;

use crate::pet::BPet;


// Curr_friends -> Shop_friends -> Action -> calculate 
pub struct Brain {
    // Index of state_table, (q_table for a certain state (action, q_value), average q_value of a certain state)
    pub q_table: HashMap<usize, (HashMap<(u8, u8, u8), f64>, f64)>,
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
            let mut q = rng.gen();
            if pair == &(0,0,0) {
                q = std::f64::MIN;
            }
            map.insert(pair.clone(), std::f64::MIN);
        }

        map
    }

    pub fn process(&mut self, game: &mut crate::game::Game) -> f64 {
        // Plans for optimization
        // Clear self.q_table of entries that don't meet the average
        let discount_factor = 1.0;
        let alpha = 0.6;
        let _epsilon = 0.1;

        let actions = self.get_action_map_random();
  
        let mut max_reward;

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
            
            let q = self.q_table.entry(index).or_insert((actions.clone(), 0.));

            let mut max = 0.;
            let mut best_next_action = (0,0,0);
            for (_i, x) in q.0.iter().enumerate() {
                if x.1 > &max {max = *x.1; best_next_action = x.0.clone()}
            }
            

            let actual_q = q.0.get_mut(&(0,0,0)).unwrap();
            if best_next_action == (0,0,0) {
                let mut rng = rand::thread_rng();
                best_next_action = *self.action_map.choose(&mut rng).unwrap();
            }
            
            if game.crew.gold == 0 {
                best_next_action = (99,0,0);    
            }

            if game.take_action(best_next_action).is_err() {
                
            } else {
                num_actions += 1;
            }

            let reward = game.reward() as f64 / game.crew.turn as f64;

            max_reward = reward;

            let td_target = reward + discount_factor * max;
            let td_delta = td_target - *actual_q;

            *actual_q += alpha + td_delta;

            if num_actions == max_actions {
                break;
            }
        }
        max_reward
    }

}
