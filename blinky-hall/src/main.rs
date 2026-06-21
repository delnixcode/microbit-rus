#![no_std]
#![no_main]

use embedded_hal::{delay::DelayNs, digital::OutputPin};
use nrf52833_hal::{self as hal,pac::Peripherals,timer::Timer, gpio::Level};

use cortex_m_rt::entry;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {

    let peripherals = Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);
    let _col1 = port0.p0_28.into_push_pull_output(Level::Low);
    let mut row1 = port0.p0_21.into_push_pull_output(Level::High);
    let mut timer0 = Timer::new(peripherals.TIMER0);

    loop{
        timer0.delay_ms(5000);
        row1.set_high().unwrap();

        timer0.delay_ms(5000);
        row1.set_low().unwrap();
    }



  
}
