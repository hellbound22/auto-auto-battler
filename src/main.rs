mod store;
mod pet;
mod crew;
mod game;
mod battle;

use game::Game;


fn main() {
    let mut game = Game::new("std");
    let mut bot = Game::new("std");

    game.bot_random();
    bot.bot_random();
    
    println!("======================BATTLE====================");
    match game.battle(bot.crew.clone()) {
        0 => { println!("DRAW!!!!") },
        1 => { println!("WIN!!!!") },
        2 => { println!("LOST!!!!") },
        _ => { println!("ERROR") }
    }

}
