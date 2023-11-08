#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(stable_features, unknown_lints, async_fn_in_trait)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use embassy_rp::config::Config;
use embassy_rp::dma::Channel;
use embassy_rp::gpio::Pin;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

mod core0_main;
mod core1_main;

use {{crate_name}}::core1::run_task;
use {{crate_name}}::task::run_main;
use {{crate_name}}::{clear_locks, set_default_clock};

#[cortex_m_rt::entry]
fn main() -> ! {
    clear_locks();
    set_default_clock();
    rtt_init_print!();

    let p = embassy_rp::init(Config::default());
    rprintln!("init");

    run_task(p.CORE1, |spawner| spawner.spawn(core1_main::task()).unwrap());

    run_main(|spawner| {
        spawner.spawn(core0_main::task(p.PIN_25.degrade())).unwrap();
    });
}
