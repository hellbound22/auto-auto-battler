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


fn main() {
    
    //let states = qlearning::StateTable::new("./qtables/std/meta_state.table", "./qtables/std/state.table", game.get_buckets());
    let mut brain = Brain::new();

    for _x in 0..100 {
        let mut game = Game::new("std");
        
        //game.bot_random();
        //game.roll_shop(1);

        brain.process(&mut game);

        //println!("{}", game);
        
    }
    
    /* 
    std::process::exit(0);

    
    let mut bot = Game::new("std");

    game.bot_random();
    bot.bot_random();
    
    game.game_loop(bot.crew.clone());
    */
}
