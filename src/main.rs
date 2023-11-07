#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(stable_features, unknown_lints, async_fn_in_trait)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use {{crate_name}}::core1::run_task;
use {{crate_name}}::task::run_main;
use embassy_rp::config::Config;
use embassy_rp::gpio::Pin;
use rtt_target::rtt_init_print;

mod core0_main;
mod core1_main;

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

    let p = embassy_rp::init(Config::default());
    run_task(p.CORE1, |spawner| spawner.spawn(core1_main::task()).unwrap());

    run_main(|spawner| {
        spawner.spawn(core0_main::task(p.PIN_25.degrade())).unwrap();
    });
}
