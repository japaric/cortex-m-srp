//! examples/task.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_semihosting::{debug, hprintln};

#[rtfm::app(device = lm3s6965)]
const APP: () = {
    #[init(spawn = [foo])]
    fn init(c: init::Context) {
        c.spawn.foo().unwrap();
    }

    #[task(spawn = [bar, baz])]
    fn foo(c: foo::Context) {
        hprintln!("foo").unwrap();

        // spawns `bar` onto the task scheduler
        // `foo` and `bar` have the same priority so `bar` will not run until
        // after `foo` terminates
        c.spawn.bar().unwrap();

        // spawns `baz` onto the task scheduler
        // `baz` has higher priority than `foo` so it immediately preempts `foo`
        c.spawn.baz().unwrap();
    }

    #[task]
    fn bar(_: bar::Context) {
        hprintln!("bar").unwrap();

        debug::exit(debug::EXIT_SUCCESS);
    }

    #[task(priority = 2)]
    fn baz(_: baz::Context) {
        hprintln!("baz").unwrap();
    }

    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn UART0();
        fn UART1();
    }
};
