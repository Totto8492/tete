#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(stable_features, unknown_lints, async_fn_in_trait)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use embassy_rp::config::Config;
use embassy_rp::gpio::Pin;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

mod core0_main;
mod core1_main;

use {{crate_name}}::core0_task;
use {{crate_name}}::core1_task;
use {{crate_name}}::{clear_locks, set_default_clock};

#[cortex_m_rt::entry]
fn main() -> ! {
    clear_locks();
    set_default_clock();
    rtt_init_print!();

    let p = embassy_rp::init(Config::default());
    rprintln!("init");

    core1_task::run(p.CORE1, |spawner| spawner.spawn(core1_main::task()).unwrap());

    core0_task::run(|spawner| {
        spawner.spawn(core0_main::task(p.PIN_25.degrade())).unwrap();
    });
}
