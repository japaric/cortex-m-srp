#![no_main]
#![no_std]

extern crate lm3s6965;
extern crate panic_halt;
extern crate rtfm;

use rtfm::app;

#[app(device = lm3s6965)]
const APP: () = {
    #[cfg(never)]
    static mut O1: u32 = 0; // init
    #[cfg(never)]
    static mut O2: u32 = 0; // idle
    #[cfg(never)]
    static mut O3: u32 = 0; // EXTI0
    #[cfg(never)]
    static O4: u32 = 0; // idle
    #[cfg(never)]
    static O5: u32 = 0; // EXTI1
    #[cfg(never)]
    static O6: u32 = 0; // init

    #[cfg(never)]
    static mut S1: u32 = 0; // idle & EXTI0
    #[cfg(never)]
    static mut S2: u32 = 0; // EXTI0 & EXTI1
    #[cfg(never)]
    static S3: u32 = 0;

    #[init(resources = [O1, O4, O5, O6, S3])]
    fn init(c: init::Context) {
        c.resources.O1; //~ ERROR no field `O1`
        c.resources.O4; //~ ERROR no field `O4`
        c.resources.O5; //~ ERROR no field `O5`
        c.resources.O6; //~ ERROR no field `O6`
        c.resources.S3; //~ ERROR no field `S3`
    }

    #[idle(resources = [O2, O4, S1, S3])]
    fn idle(c: idle::Context) -> ! {
        c.resources.O2; //~ ERROR no field `O2`
        c.resources.O4; //~ ERROR no field `O4`
        c.resources.S1; //~ ERROR no field `S1`
        c.resources.S3; //~ ERROR no field `S3`

        loop {}
    }

    #[interrupt(resources = [O3, S1, S2, S3])]
    fn UART0(c: UART0::Context) {
        c.resources.O3; //~ ERROR no field `O3`
        c.resources.S1; //~ ERROR no field `S1`
        c.resources.S2; //~ ERROR no field `S2`
        c.resources.S3; //~ ERROR no field `S3`
    }

    #[interrupt(resources = [S2, O5])]
    fn UART1(c: UART1::Context) {
        c.resources.S2; //~ ERROR no field `S2`
        c.resources.O5; //~ ERROR no field `O5`
    }
};
