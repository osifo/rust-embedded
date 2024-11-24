use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use microbit::hal::gpiote::Gpiote;
use super::utils::Turn;

static GPIO: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));
static TURN: Mutex<RefCell<Turn>> = Mutex::new(RefCell::new(Turn::None));

use cortex_m::interrupt::free;
use microbit::{
    pac::{ self, GPIOTE },
    board::Buttons
};

use microbit::pac::interrupt;

pub (crate) fn init_buttons(board_gpiote: GPIOTE, buttons: Buttons) {
    let gpiote = Gpiote::new(board_gpiote);

    let channel0 = gpiote.channel0();
    let channel1 = gpiote.channel1();

    channel0
        .input_pin(&buttons.button_a.degrade())
        .hi_to_lo() // a falling edge
        .enable_interrupt();
    channel0.reset_events();

    channel1
        .input_pin(&buttons.button_b.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel1.reset_events();

    free(move |critical_sec| {
        *GPIO.borrow(critical_sec).borrow_mut() = Some(gpiote)
    });

    unsafe {
        pac::NVIC::unmask(pac::interrupt::GPIOTE);
    }
    pac::NVIC::unpend(pac::interrupt::GPIOTE);
}
#[interrupt]
fn GPIOTE() {
    free(|critical_section| {
        if let Some(gpiote) = GPIO.borrow(critical_section).borrow().as_ref() {
            let a_pressed = gpiote.channel0().is_event_triggered();
            let b_pressed = gpiote.channel1().is_event_triggered();

            let turn_value = match (a_pressed, b_pressed) {
                (true, false) => Turn::Left,
                (false, true) => Turn::Right,
                _             => Turn::None
            };

            gpiote.channel0().reset_events();
            gpiote.channel1().reset_events();

            *TURN.borrow(critical_section).borrow_mut() = turn_value;
        }
    });
}

pub fn get_turn(reset: bool) -> Turn {
    free(|critical_section| {
        let turn = *TURN.borrow(critical_section).borrow(); // gets the value from the TURN mutex.

        if reset {
            *TURN.borrow(critical_section).borrow_mut() = Turn::None
        }

        turn
    })
}


