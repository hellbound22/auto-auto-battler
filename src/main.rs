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
    let mut last = ((0, 0, 0), 0.);

    let range = 1;
    for x in 0..range {
        let pct = x as f64 / range as f64 * 100.;
        print!("Processing {}: {:.2}%\r", x, pct);
        let mut game = Game::new("std");
        
        //game.bot_random();
        //game.roll_shop(1);

        brain.process(&mut game);

        //println!("{}", game);
        last = brain.get_best_actions(&mut game);
        
    }

    dbg!(last);
    
    /* 
    std::process::exit(0);

    
    let mut bot = Game::new("std");

    game.bot_random();
    bot.bot_random();
    
    game.game_loop(bot.crew.clone());
    */
}
