//! Prints "Hello, world!" on the OpenOCD console using semihosting
//!
//! ---

#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate stm32f40x;

use core::fmt::Write;

use cortex_m::asm;
use cortex_m_semihosting::hio;

//use stm32f40x::{gpiod, RCC};

fn main() {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Hello, Rust!").unwrap();

    let mut count = 0u32;
    loop {
        writeln!(stdout, "KoenigHack 2018 {}", count).unwrap();
        count += 1;
    }
    /*cortex_m::interrupt::free(|cs| {
        let gpioe = GPIOD.borrow(cs);
        let rcc = RCC.borrow(cs);

        rcc.ahbenr.modify(|_, w| w.iopden().enabled());
    });*/
    /*unsafe {
        let mut peripherals = stm32f40x::Peripherals::take().unwrap();
        peripherals.GPIOD.odr.write(|w| w.bits(14));
    }*/

    //peripherals.rcc.apb1enr.modify(|_, w| w.usart2en().enabled());
    //peripherals.rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
}

// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
