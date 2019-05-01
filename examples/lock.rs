//! examples/lock.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_semihosting::{debug, hprintln};
use lm3s6965::Interrupt;

#[rtfm::app(device = lm3s6965)]
const APP: () = {
    static mut SHARED: u32 = 0;

    #[init]
    fn init(_: init::Context) {
        rtfm::pend(Interrupt::GPIOA);
    }

    // when omitted priority is assumed to be `1`
    #[interrupt(resources = [SHARED])]
    fn GPIOA(mut c: GPIOA::Context) {
        hprintln!("A").unwrap();

        // the lower priority task requires a critical section to access the data
        c.resources.SHARED.lock(|shared| {
            // data can only be modified within this critical section (closure)
            *shared += 1;

            // GPIOB will *not* run right now due to the critical section
            rtfm::pend(Interrupt::GPIOB);

            hprintln!("B - SHARED = {}", *shared).unwrap();

            // GPIOC does not contend for `SHARED` so it's allowed to run now
            rtfm::pend(Interrupt::GPIOC);
        });

        // critical section is over: GPIOB can now start

        hprintln!("E").unwrap();

        debug::exit(debug::EXIT_SUCCESS);
    }

    #[interrupt(priority = 2, resources = [SHARED])]
    fn GPIOB(mut c: GPIOB::Context) {
        // the higher priority task does *not* need a critical section
        *c.resources.SHARED += 1;

        hprintln!("D - SHARED = {}", *c.resources.SHARED).unwrap();
    }

    #[interrupt(priority = 3)]
    fn GPIOC(_: GPIOC::Context) {
        hprintln!("C").unwrap();
    }
};
