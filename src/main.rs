mod actors;
mod game;
mod battle;
mod util;
mod qlearning;

use game::Game;
use qlearning::Brain;

use crossterm::{
    ExecutableCommand,
    cursor::{MoveUp}
};
use std::io::{stdout};


fn main() {
    let mut brain = Brain::new();

    let mut acc = 0.;

    let range = 10000;
    //std::process::exit(0);
    for x in 0..range {
        let pct = x as f64 / range as f64 * 100.;
        println!("Processing {}: {:.2}%", x, pct);
        let mut game = Game::new("std");

        acc += brain.process(&mut game);

        println!("Average game reward at the end {:.4}\r", acc / range as f64);
        
        stdout()
            .execute(MoveUp(2)).unwrap();

    }
}
