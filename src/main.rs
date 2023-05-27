#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_rp::gpio::{Level, Output};
use rtt_target::{rprintln, rtt_init_print};
use {{crate_name}}::{core0, core1, run_task, run_task_with, timeout};
use {panic_rtt_target as _, rtt_target as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("--- RESET ---");
    let p = embassy_rp::init(Default::default());

    let led = Output::new(p.PIN_25, Level::Low);

    run_task_with(p.CORE1, |spawner| (spawner.spawn(core1::task(led))).unwrap());
    run_task(|spawner| {
        spawner.spawn(core0::task()).unwrap();
        spawner.spawn(timeout::task()).unwrap();
    });
}
