//! Two tasks running at the *same* priority with access to the same resource
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate cortex_m_rt_macros;
extern crate cortex_m_rtfm as rtfm;
extern crate panic_halt;
extern crate stm32f103xx;

use rtfm::{app, Threshold};

app! {
    device: stm32f103xx,

    resources: {
        static COUNTER: u64 = 0;
    },

    // Both SysTick and TIM2 have access to the `COUNTER` data
    tasks: {
        SysTick: {
            path: sys_tick,
            resources: [COUNTER],
        },

        TIM2: {
            path: tim2,
            resources: [COUNTER],
        },
    },
}

fn init(_p: init::Peripherals, _r: init::Resources) {
    // ..
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

// As both tasks are running at the same priority one can't preempt the other.
// Thus both tasks have direct access to the resource
fn sys_tick(_t: &mut Threshold, mut r: SysTick::Resources) {
    // ..

    *r.COUNTER += 1;

    // ..
}

fn tim2(_t: &mut Threshold, mut r: TIM2::Resources) {
    // ..

    *r.COUNTER += 1;

    // ..
}
