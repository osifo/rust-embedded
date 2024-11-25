use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex};
use microbit::display::nonblocking::Display;
use microbit::gpio::DisplayPins;
use microbit::pac;
use microbit::pac::TIMER1;
use microbit::pac::interrupt;
use tiny_led_matrix::Render;

static DISPLAY: Mutex<RefCell<Option<Display<TIMER1>>>> = Mutex::new(RefCell::new(None));

pub (crate) fn init_display(board_timer: TIMER1, board_display: DisplayPins) {
    let display = Display::new(board_timer, board_display);

    free(|critical_section| {
        *DISPLAY.borrow(critical_section).borrow_mut() = Some(display);
    });

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::TIMER1);
    }
}

// diplays the snake representiation as an image on matrix
pub (crate) fn display_image(image: &impl Render) {
    free(|critical_section| {
        if let Some(display) = DISPLAY.borrow(critical_section).borrow_mut().as_mut() {
            display.show(image)
        }
    })
}

// turn all leds off
pub (crate) fn clear_display() {
    free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.clear();
        }
    })
}

#[interrupt]
fn TIMER1() {
    free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.handle_display_event();
        }
    });
}