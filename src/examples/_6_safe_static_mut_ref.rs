//! Safe creation of `&'static mut` references
//!
//! ```
//! #![deny(unsafe_code)]
//! #![deny(warnings)]
//! #![no_std]
//! #![no_main]
//! 
//! extern crate cortex_m_rt_macros;
//! extern crate cortex_m_rtfm as rtfm;
//! extern crate panic_halt;
//! extern crate stm32f103xx;
//! 
//! use rtfm::app;
//! 
//! app! {
//!     device: stm32f103xx,
//! 
//!     resources: {
//!         static BUFFER: [u8; 16] = [0; 16];
//!     },
//! 
//!     init: {
//!         resources: [BUFFER],
//!     },
//! }
//! 
//! fn init(_p: init::Peripherals, r: init::Resources) {
//!     let _buf: &'static mut [u8; 16] = r.BUFFER;
//! }
//! 
//! fn idle() -> ! {
//!     loop {
//!         rtfm::wfi();
//!     }
//! }
//! ```
// Auto-generated. Do not modify.
