#![no_main]
#![no_std]

mod game;

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use crate::game::Game;
use game::utils::GameStatus;
use game::controls::get_turn;
use crate::game::controls::init_buttons;
use crate::game::display::{init_display, display_image, clear_display};

use microbit::{
    Board,
    hal::{prelude::*, Rng, Timer},
    display::nonblocking::{BitImage, GreyscaleImage}
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut rng = Rng::new(board.RNG);
    let mut game = Game::new(rng.random_u32());

    init_buttons(board.GPIOTE, board.buttons);
    init_display(board.TIMER1, board.display_pins);

    loop {
        // general application loop
        loop {
            //this is the game loop
            let image = GreyscaleImage::new(&game.game_matrix(6, 3, 9));

            //displays the current state of the game as a image
            display_image(&image);
            timer.delay_ms(game.calc_step_interval());
            match game.status {
                GameStatus::Ongoing => game.step(get_turn(true)),
                _ => {
                    //handles won or lost scenarios
                    for _ in 0..3 {
                        clear_display();
                        timer.delay_ms(200u32); //waits for 200ms
                        display_image(&image);
                        timer.delay_ms(200u32); //waits for 200ms
                    }

                    clear_display();
                    display_image(&BitImage::new(&game.score_matrix())); //displays the score got 1 sec
                    timer.delay_ms(200u32);
                    break // ends the game loop
                }
            }
        }
        // once the game loop ends, reset and restart the game
        game.reset();
    }
}