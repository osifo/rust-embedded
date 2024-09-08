#![no_main]
#![no_std]

mod game;

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use crate::game::Game;
use game::utils::{GameStatus, Turn};

use microbit::{
    Board,
    hal::{prelude::*, Rng, Timer},
    display::blocking::Display
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut rng = Rng::new(board.RNG);
    let mut game = Game::new(rng.random_u32());
    let mut display = Display::new(board.display_pins);

    loop {
        // general application loop
        loop {
            //this is the game loop
            let image = game.game_matrix(9, 9, 9);

            //displays the current state of the game (via the game matrix)
            display.show(&mut timer, image, game.calc_step_interval());

            match game.status {
                GameStatus::Ongoing => game.step(Turn::None),
                _ => {
                    //handles won or lost scenarios
                    for _ in 0..3 {
                        display.clear();
                        timer.delay_ms(200u32); //waits for 200ms
                        display.show(&mut timer, image, 200)
                    }

                    display.clear();
                    display.show(&mut timer, game.score_matrix(), 1000); //displays the score got 1 sec
                    break // ends the game loop
                }
            }
        }
        // once the game loop ends, reset and restart the game
        game.reset();
    }
}