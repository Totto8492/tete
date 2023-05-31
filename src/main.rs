#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_rp::gpio::{Level, Output};
use {{crate_name}}::{run_preemptive_task, run_task, run_task_at, Priority};
use rtt_target::{rprintln, rtt_init_print};
use {panic_rtt_target as _, rtt_target as _};

mod tasks;
use tasks::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("--- RESET ---");
    let p = embassy_rp::init(Default::default());

    let led = Output::new(p.PIN_25, Level::Low);

    run_preemptive_task(Priority::medium(), |spawner| spawner.spawn(core0::run_med()).unwrap());
    run_preemptive_task(Priority::high(), |spawner| spawner.spawn(core0::run_high()).unwrap());
    run_task_at(p.CORE1, |spawner| (spawner.spawn(core1::task(led))).unwrap());
    run_task(|spawner| {
        spawner.spawn(core0::task()).unwrap();
        spawner.spawn(timeout::task()).unwrap();
    });
}
