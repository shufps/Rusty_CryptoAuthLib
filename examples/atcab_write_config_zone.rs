#![no_main]
#![no_std]
// #![allow(warnings)]

extern crate nrf52840_hal as hal;
extern crate panic_halt;
extern crate nrf52840_mdk;
use cortex_m_rt::{entry, exception};

use hal::gpio::{p0, p1};
use hal::target::Peripherals;
use hal::timer::Timer;
use hal::twim::{self, Twim};
// use cortex_m_semihosting::hprintln;
use Rusty_CryptoAuthLib::ATECC608A;
use nrf52840_mdk::Pins;

#[derive(Copy, Clone, Debug)]
pub enum TestEnum {
    OutofBoxConfig,
    AteccTflxTlsConfig,
}

impl<'a> TestEnum {
    pub fn get_value(self) -> &'a [u8] {
        match self {
            TestEnum::OutofBoxConfig => &[
                0x01, 0x23, 0x45, 0xE1, 0x00, 0x00, 0x60, 0x02, 0x23, 0xB1, 0xBD, 0x5B, 0xEE, 0x01,
                0x55, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x83, 0x20, 0x87, 0x20, 0x8F, 0x20, 0xC4, 0x8F,
                0x8F, 0x8F, 0x8F, 0x8F, 0x9F, 0x8F, 0xAF, 0x8F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xAF, 0x8F, 0xFF, 0xFF, 0xFF, 0xFF,
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x55, 0x55, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x33, 0x00,
                0x33, 0x00, 0x33, 0x00, 0x1C, 0x00, 0x1C, 0x00, 0x1C, 0x00, 0x1C, 0x00, 0x1C, 0x00,
                0x3C, 0x00, 0x3C, 0x00, 0x3C, 0x00, 0x3C, 0x00, 0x3C, 0x00, 0x3C, 0x00, 0x3C, 0x00,
                0x1C, 0x00,
            ],
            TestEnum::AteccTflxTlsConfig => &[
                0x01, 0x23, 0x45, 0xE1, 0x00, 0x00, 0x60, 0x02, 0x23, 0xB1, 0xBD, 0x5B, 0xEE, 0x01,
                0x55, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x85, 0x00, 0x82, 0x00, 0x85, 0x20, 0x85, 0x20,
                0x85, 0x20, 0x8F, 0x46, 0x8F, 0x0F, 0x9F, 0x8F, 0x0F, 0x0F, 0x8F, 0x0F, 0x0F, 0x0F,
                0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0D, 0x1F, 0x0F, 0x0F, 0xFF, 0xFF, 0xFF, 0xFF,
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x03, 0xF7, 0x00, 0x69, 0x76, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x55, 0x55, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x53, 0x00,
                0x53, 0x00, 0x73, 0x00, 0x73, 0x00, 0x73, 0x00, 0x38, 0x00, 0x7C, 0x00, 0x1C, 0x00,
                0x3C, 0x00, 0xA1, 0x00, 0x3C, 0x00, 0x30, 0x00, 0x3C, 0x00, 0x30, 0x00, 0x12, 0x00,
                0x30, 0x00,
            ],
        }
    }
}


#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let pins = Pins::new(p0::Parts::new(p.P0), p1::Parts::new(p.P1));
    let scl = pins.p27.into_floating_input().degrade();
    let sda = pins.p26.into_floating_input().degrade();

    let i2c_pins = twim::Pins { scl, sda };

    let i2c = Twim::new(p.TWIM1, i2c_pins, twim::Frequency::K100);
    let delay = Timer::new(p.TIMER0);
    let timer = Timer::new(p.TIMER1);
    let mut atecc608a = ATECC608A::new(i2c, delay, timer).unwrap();

    // WRITE COMMAND EXAMPLE 

    // Check to see if the device's config zone is locked before any write. (Config zone ID- 0x00  )
    if !(atecc608a.atcab_is_locked(0x00)) { 
        let selection = TestEnum::AteccTflxTlsConfig;
        let write_response = atecc608a.atcab_write_config_zone(selection.get_value());
        assert_eq!([[0, 0x03, 0x40]; 15], &write_response[..])
    }

    loop {}
}

#[exception]
fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}