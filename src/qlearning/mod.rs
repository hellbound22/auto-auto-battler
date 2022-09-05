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


        let state = (game.get_state().to_owned(), game.get_shop().to_owned());

        let index = match self.state_table.binary_search(&state) {
            Ok(x) => { x },
            Err(x) => {
                self.state_table.push(state.clone());
                x
            }
        };
        
        let actions = self.get_action_map_random();

        let mut num_actions = 0;
        let mut max_reward = 0.;

        for _ in 0..100 {
            let q = self.q_table.entry(index).or_insert(actions.clone());

            let best_next_action = q.values().fold(std::f64::MIN, |a,b| a.max(*b));
            //let worse_next_action = q.values().fold(std::f64::MAX, |a,b| a.min(*b));

            //dbg!(best_next_action);
            //dbg!(worse_next_action);

            let actual_q = q.get_mut(&(0,0,0)).unwrap();

            let mut rng = rand::thread_rng();
            let random_action = self.action_map.choose(&mut rng).unwrap();
            
            if game.crew.gold == 0 {
                game.take_action((99,0,0)).unwrap();    
            }

            if game.take_action(*random_action).is_err() {
                continue;
            }

            let reward = game.reward() as f64 / game.crew.turn as f64;

            if reward > max_reward {max_reward = reward}

            let td_target = reward + discount_factor * best_next_action;
            let td_delta = td_target - *actual_q;

            *actual_q += alpha + td_delta;

            //dbg!(reward);
            num_actions += 1;
        }

        dbg!(max_reward);
    }

}