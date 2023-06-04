#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_rp::gpio::{Level, Output};
use {{crate_name}}::{run_preemptive_task, run_task, run_task_on, Priority};
use rtt_target::{rprintln, rtt_init_print};
use {panic_rtt_target as _, rtt_target as _};

mod tasks;
use tasks::*;

// https://github.com/rp-rs/rp-hal/blob/main/rp2040-hal-macros/src/lib.rs
unsafe fn clear_locks() {
    const SIO_BASE: u32 = 0xd0000000;
    const SPINLOCK0_PTR: *mut u32 = (SIO_BASE + 0x100) as *mut u32;
    const SPINLOCK_COUNT: usize = 32;
    for i in 0..SPINLOCK_COUNT {
        SPINLOCK0_PTR.wrapping_add(i).write_volatile(1);
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    unsafe {
        clear_locks();
    }
    rtt_init_print!();
    rprintln!("--- RESET ---");
    let p = embassy_rp::init(Default::default());

    let led = Output::new(p.PIN_25, Level::Low);

    run_preemptive_task(Priority::medium(), |spawner| spawner.spawn(core0::run_med()).unwrap());
    run_preemptive_task(Priority::high(), |spawner| spawner.spawn(core0::run_high()).unwrap());
    run_task_on(p.CORE1, |spawner| (spawner.spawn(core1::task(led))).unwrap());
    run_task(|spawner| {
        spawner.spawn(core0::task()).unwrap();
        spawner.spawn(timeout::task()).unwrap();
    });
}
