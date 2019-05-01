//! examples/static.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_semihosting::{debug, hprintln};
use lm3s6965::Interrupt;

#[rtfm::app(device = lm3s6965)]
const APP: () = {
    static KEY: u32 = ();

    #[init]
    fn init(_: init::Context) -> init::LateResources {
        rtfm::pend(Interrupt::UART0);
        rtfm::pend(Interrupt::UART1);

        init::LateResources { KEY: 0xdeadbeef }
    }

    #[interrupt(resources = [KEY])]
    fn UART0(c: UART0::Context) {
        hprintln!("UART0(KEY = {:#x})", c.resources.KEY).unwrap();

        debug::exit(debug::EXIT_SUCCESS);
    }

    #[interrupt(priority = 2, resources = [KEY])]
    fn UART1(c: UART1::Context) {
        hprintln!("UART1(KEY = {:#x})", c.resources.KEY).unwrap();
    }
};
