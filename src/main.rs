#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use embassy_rp::config::Config;
use embassy_rp::gpio::{Level, Output};
use rtt_target::{rprintln, rtt_init_print};
use {{crate_name}}::{run_preemptive_task, run_task, run_task_on, Priority};
use {panic_rtt_target as _, rtt_target as _};

mod tasks;
use tasks::{core0, core1, timeout};

fn clear_locks() {
    // https://github.com/rp-rs/rp-hal/blob/main/rp2040-hal-macros/src/lib.rs
    for i in 0..32 {
        embassy_rp::pac::SIO.spinlock(i).write_value(1);
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    clear_locks();
    rtt_init_print!();
    rprintln!("--- RESET ---");
    let p = embassy_rp::init(Config::default());

    let led = Output::new(p.PIN_25, Level::Low);

    run_preemptive_task(Priority::P2, |spawner| {
        spawner.spawn(core0::run_med()).unwrap();
    });
    run_preemptive_task(Priority::P3, |spawner| {
        spawner.spawn(core0::run_high()).unwrap();
    });
    run_task_on(p.CORE1, |spawner| {
        spawner.spawn(core1::task(led)).unwrap();
    });
    run_task(|spawner| {
        spawner.spawn(core0::task()).unwrap();
        spawner.spawn(timeout::task()).unwrap();
    });
}
