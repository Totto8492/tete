#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::{Executor};
use embassy_rp::gpio::{Level, Output};
use embassy_rp::multicore::spawn_core1;
#[allow(unused)]
use rtt_target::rprintln as info;
#[allow(unused)]
use rtt_target::{rdbg, rprint, rprintln, rtt_init_print};
use {{project_name}}::{core0, core1, CORE1_STACK, EXECUTOR0, EXECUTOR1};
use {panic_rtt_target as _, rtt_target as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    info!("--- RESET ---");
    let p = embassy_rp::init(Default::default());

    let led = Output::new(p.PIN_5, Level::Low);

    spawn_core1(p.CORE1, unsafe { &mut CORE1_STACK }, move || {
        let executor1 = EXECUTOR1.init(Executor::new());
        executor1.run(|spawner| (spawner.spawn(core1::task(led))).unwrap());
    });

    let executor0 = EXECUTOR0.init(Executor::new());
    executor0.run(|spawner| (spawner.spawn(core0::task())).unwrap());
}
