//! Runtime initialized resources
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate lm3s6965;
extern crate panic_halt;
extern crate rtfm;

#[rtfm::app(device = lm3s6965)]
const APP: () = {
    static mut X: u32 = ();
    static Y: u32 = ();

    #[init]
    fn init(_: init::Context) -> init::LateResources {
        init::LateResources { X: 0, Y: 1 }
    }
};
