#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(stable_features, unknown_lints, async_fn_in_trait)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use embassy_rp::config::Config;
use rtt_target::rtt_init_print;
use {{crate_name}}::run_task;

fn clear_locks() {
    // https://github.com/rp-rs/rp-hal/blob/main/rp2040-hal-macros/src/lib.rs
    for i in 0..32 {
        embassy_rp::pac::SIO.spinlock(i).write_value(1);
    }
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    cortex_m::asm::udf();
}

#[cortex_m_rt::entry]
fn main() -> ! {
    clear_locks();
    rtt_init_print!();

    let p = embassy_rp::init(Config::default());

    run_task(|spawner| {
        spawner.spawn(sample::task(p)).unwrap();
    });
}

mod sample {
    use embassy_rp::gpio::{Level, Output};
    use embassy_rp::Peripherals;
    use embassy_time::Timer;

    #[embassy_executor::task]
    pub async fn task(p: Peripherals) {
        let mut led = Output::new(p.PIN_25, Level::High);
        loop {
            Timer::after_millis(200).await;
            led.toggle();
        }
    }
}
