use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

use crate::pet::BPet;
use crate::pet::Pet;
use crate::food::Food;

pub struct StateTable {
    map: HashMap<Vec<Option<BPet>>, usize>,
}

impl StateTable {
    pub fn new(path: &str, buckets: (&Vec<Pet>, &Vec<Food>)) -> Self {
        let file = File::open(path).unwrap();
        let buf_reader = BufReader::new(file);

        let mut map = HashMap::new();

        for (i, line) in buf_reader.lines().enumerate() {
            let mut crew = Vec::new();
            for p in line.unwrap().split('|') {
                if p != "" { // TODO: gambi
                    let split = p.split(",").next().unwrap().parse().unwrap();
                    let ref_pet: Vec<Pet> = buckets.0.iter().filter(|x| x.id == split).cloned().collect();
                    crew.push(Some(BPet::new_from_state_table(p, &ref_pet[0])));
                } 
            }

            loop {
                if crew.len() != 5 {
                    crew.push(None);
                } else {
                    break;
                }
            }

            map.insert(crew, i);
            
        }

        let test = &map.clone().into_keys().collect::<Vec<Vec<Option<BPet>>>>()[10];
        for x in test {
            if x.is_some() {
                println!("{}", x.as_ref().unwrap());
            } else {
                println!("None");
            }
        }

        Self {
            map
        }
    }
}

fn gen_q_table() -> std::io::Result<()> {
    let mut file = File::create("./qtables/std/qtable.table")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}