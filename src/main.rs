mod store;
mod pet;
mod crew;
mod game;
mod battle;
mod util;
mod food;
mod qlearning;

use game::Game;
use qlearning::Brain;

use crossterm::{
    ExecutableCommand, execute, Result,
    cursor::{DisableBlinking, EnableBlinking, MoveUp, RestorePosition, SavePosition}
};
use std::io::{stdout, Write};


fn main() {
    let mut brain = Brain::new();
    let mut _last = ((0, 0, 0), 0.);

    let mut acc = 0.;

    let range = 10000;
    for x in 0..range {
        let pct = x as f64 / range as f64 * 100.;
        println!("Processing {}: {:.2}%", x, pct);
        let mut game = Game::new("std");
        
        //game.bot_random();
        //game.roll_shop(1);

        acc += brain.process(&mut game);

        //println!("{}", game);
        //_last = brain.get_best_actions(&mut game);

        println!("Average game reward at the end {:.4}\r", acc / range as f64);
        
        stdout()
            .execute(MoveUp(2)).unwrap();

    }
}
