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
    pub q_table: HashMap<usize, HashMap<u8, (u8, u8)>>,
    pub state_table: Vec<(Vec<Option<BPet>>, Vec<Option<BPet>>)>,
    pub action_map: Vec<(u8, u8)>,
}

impl Brain {
    pub fn new() -> Self {
        Self {
            q_table: HashMap::new(),
            state_table: Vec::new(),
            action_map: Vec::new(),
        }
    }

    pub fn process(&mut self, game: crate::game::Game) {
        let state = (game.get_state().to_owned(), game.get_shop().to_owned());

        let index = match self.state_table.binary_search(&state) {
            Ok(x) => { x },
            Err(x) => {
                self.state_table.push(state.clone());
                x
            }
        };

        let mut rng = rand::thread_rng();
        
        
        let file = File::open("qtables/std/action.table").unwrap();
        let buf_reader = BufReader::new(file);

        let mut map = HashMap::new();

        for line in buf_reader.lines() {
            let line = line.unwrap();
            let s = line.split(',').collect::<Vec<&str>>();

            map.insert(rng.gen(), (s[0].parse().unwrap(), s[1].parse().unwrap()));

        }

        let q = self.q_table.entry(index).or_insert(map);

        dbg!(q);

    }

}