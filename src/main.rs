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
    
    // println!("{}", game);
    // game.roll_shop(1);
    // println!("{}", game);

    //game.swap_pet(0, 4);
    /* 
    
    println!("======================BATTLE====================");
    

    
    */
    //println!("{}", game);
    game.game_loop(bot.crew.clone());
}
