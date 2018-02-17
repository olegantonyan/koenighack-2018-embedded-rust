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

//use stm32f40x::{USART6};

fn main() {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Hello, Rust!").unwrap();

    let mut count = 0u32;
    loop {
        writeln!(stdout, "iteration {}", count).unwrap();
        count += 1;
    }

    //let mut peripherals = stm32f40x::Peripherals::take().unwrap();
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
