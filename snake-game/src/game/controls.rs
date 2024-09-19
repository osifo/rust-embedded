use cortex_m::cell::RefCell;
use cortex_m::interrupt::Mutex;
use microbit::hal::gpiote::Gpiote;
use game::utils::Turn;


static GPIO: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));
static TURN: Mutex<Refcell<Turn>> = Mutex::new(RefCell::new(None));

use cortex_m::nterrupt::free;
use microbit::{
    pac::{ self, GPIOTE },
    board::Buttons
};

pub (crate) fn init_buttons(board_gpiote: GPIOTE, buttons: Buttons) {
    let gpiote = Gpiote.new(board_gpiote);

    let channel0 = gpiote.channel0();
    let channel1 = gpiote.channel1();

    channel0
        .input_pin(&buttons.button_a.degrade())
        .hi_to_lo()
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


