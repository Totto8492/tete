use embassy_executor::SendSpawner;
use embassy_rp::gpio::{AnyPin, Level, Output, Pin};
use embassy_rp::Peripherals;
use embassy_time::{Instant, Timer};
use {{crate_name}}::task::run_on;

use crate::core1_main;

#[embassy_executor::task]
async fn led_task(led_pin: AnyPin) {
    let mut led = Output::new(led_pin, Level::High);
    loop {
        Timer::after_millis(200).await;
        led.toggle();
    }
}

#[embassy_executor::task]
pub async fn task(spawner: SendSpawner, p: Peripherals) {
    run_on(p.CORE1, |spawner| spawner.spawn(core1_main::task()).unwrap());

    spawner.must_spawn(led_task(p.PIN_25.degrade()));
    loop {
        Timer::at(Instant::MAX).await;
    }
}
