//! examples/generics.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_semihosting::{debug, hprintln};
use lm3s6965::Interrupt;
use rtfm::Mutex;

#[rtfm::app(device = lm3s6965)]
const APP: () = {
    static mut SHARED: u32 = 0;

    #[init]
    fn init(_: init::Context) {
        rtfm::pend(Interrupt::UART0);
        rtfm::pend(Interrupt::UART1);
    }

    #[interrupt(resources = [SHARED])]
    fn UART0(c: UART0::Context) {
        static mut STATE: u32 = 0;

        hprintln!("UART0(STATE = {})", *STATE).unwrap();

        advance(STATE, c.resources.SHARED);

        rtfm::pend(Interrupt::UART1);

        debug::exit(debug::EXIT_SUCCESS);
    }

    #[interrupt(priority = 2, resources = [SHARED])]
    fn UART1(mut c: UART1::Context) {
        static mut STATE: u32 = 0;

        hprintln!("UART1(STATE = {})", *STATE).unwrap();

        // just to show that `SHARED` can be accessed directly and ..
        *c.resources.SHARED += 0;
        // .. also through a (no-op) `lock`
        c.resources.SHARED.lock(|shared| *shared += 0);

        advance(STATE, c.resources.SHARED);
    }
};

fn advance(state: &mut u32, mut shared: impl Mutex<T = u32>) {
    *state += 1;

    let (old, new) = shared.lock(|shared| {
        let old = *shared;
        *shared += *state;
        (old, *shared)
    });

    hprintln!("SHARED: {} -> {}", old, new).unwrap();
}
