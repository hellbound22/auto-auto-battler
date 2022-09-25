use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

use rand::prelude::*;
use mongodb::{
    bson,
    bson::doc,
    sync::Client,
    sync::Collection,
    options::FindOneOptions,
};
use serde::{Serialize, Deserialize};

use crate::actors::pet::BPet;
use crate::util::parse_action_trio;
use crate::util::serialize_action_trio;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct QTable {
    id: u32,
    //map: HashMap<(u8, u8, u8), f64>,
    map: HashMap<String, f64>,
    _used: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct State {
    id: u32, 
    friends: Vec<Option<BPet>>,
    shop: Vec<Option<BPet>>,
}

pub struct Brain {
    // Index of state_table, (q_table for a certain state (action, q_value), average q_value of a certain state)
    //pub q_table: HashMap<usize, (HashMap<(u8, u8, u8), f64>, f64)>,
    pub q_table: Collection<QTable>,
    pub state_table: Collection<State>,
    //pub state_table: Vec<(Vec<Option<BPet>>, Vec<Option<BPet>>)>,
    pub action_map: Vec<(u8, u8, u8)>,
}

impl Brain {
    pub fn new() -> Self {
        let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
        let database = client.database("sap");
        let state_collection = database.collection::<State>("states");
        let qtable_collection = database.collection::<QTable>("qtable");

        // This drops both collections for debug purposes
        state_collection.drop(None).unwrap();
        qtable_collection.drop(None).unwrap();

        let file = File::open("qtables/std/action.table").unwrap();
        let buf_reader = BufReader::new(file);

        let mut map: Vec<(u8, u8, u8)> = Vec::new();

        for line in buf_reader.lines() {
            let line = line.unwrap();
            let s = line.split(',').collect::<Vec<&str>>();

            map.push((s[0].parse().unwrap(), s[1].parse().unwrap(), s[2].parse().unwrap()));
        }

        Self {
            q_table: qtable_collection,
            state_table: state_collection,
            action_map: map,
        }
    }

    pub fn get_action_map_random(&self) -> HashMap<String, f64> {
        let mut map = HashMap::new();

        let mut rng = rand::thread_rng();

        for pair in &self.action_map {
            let mut _q = rng.gen();
            if pair == &(0,0,0) {
                _q = std::f64::MIN;
            }
            map.insert(serialize_action_trio(*pair), std::f64::MIN);
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

            let last_doc_index = match self.state_table.find_one(doc!{}, Some(FindOneOptions::builder().sort(doc! {"_id": -1}).build())) {
                Ok(doc) => {doc.unwrap_or_default().id},
                Err(_) => {0},
            };
            //println!("F last_doc_index");

            let tst = State {
                id: 0,
                friends: state.0.clone(),
                shop: state.1.clone(),
            };

            #[derive(Clone, Debug, Serialize, Deserialize)]
            struct FindQuery {
                friends: Vec<Option<BPet>>,
                shop: Vec<Option<BPet>>,
            }

            let fq = FindQuery{friends: state.0, shop: state.1};

            let doc = bson::to_document(&fq).unwrap();
            let index = if let Some(state) = self.state_table.find_one(doc, None).unwrap() {
                        state.id     
                    } else {
                        self.state_table.insert_one(tst, None).unwrap(); // NOT RIGHT
                        last_doc_index
                    };
            
                    //println!("F index");

            let q_obj = if let Some(obj) = self.q_table.find_one(doc!{"id" : index}, None).unwrap() {
                    obj
            } else {
                    let k = QTable {
                        id: index,
                        map: actions.clone(),
                        _used: 0.,
                    };
                    let _id = self.q_table.insert_one(k, None);
                    self.q_table.find_one(doc!{"_id" : _id.unwrap().inserted_id}, None).unwrap().unwrap()
                };

                //println!("F q_obj");
            let mut q = q_obj.map.clone();
            //let q = self.q_table.entry(index).or_insert((actions.clone(), 0.));

            let mut max = 0.;
            let mut best_next_action = (0,0,0);
            for (_i, x) in q.iter().enumerate() {
                if x.1 > &max {max = *x.1; best_next_action = parse_action_trio(&x.0)}
            }
            

            let actual_q = q.get_mut(&serialize_action_trio((0,0,0))).unwrap();
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

            //println!("T Action");
            let reward = game.reward() as f64 / game.crew.turn as f64;

            max_reward = reward;

            let td_target = reward + discount_factor * max;
            let td_delta = td_target - *actual_q;

            *actual_q += alpha + td_delta;

            self.q_table.find_one_and_replace(doc!{"id" : index}, q_obj.clone(), None).unwrap();
            //println!("W q_table");

            if num_actions == max_actions {
                break;
            }
        }
        max_reward
    }

}
