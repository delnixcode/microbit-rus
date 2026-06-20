#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display::blocking::Display, hal::timer::Timer};
use cortex_m_rt::entry;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let button_a = board.buttons.button_a;
    let button_b = board.buttons.button_b;

    let happy_face = [
        [0, 0, 0, 0, 0],
        [0, 1, 0, 1, 0],
        [0, 0, 0, 0, 0],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
    ];

    let sad_face = [
        [0, 0, 0, 0, 0],
        [0, 1, 0, 1, 0],
        [0, 0, 0, 0, 0],
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
    ];


    loop {
        let a_pressed = button_a.is 
    }
}