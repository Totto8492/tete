#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::Config;
use embassy_time::Timer;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    rtt_init_print!();

    let p = embassy_stm32::init(Config::default());
    let mut led = Output::new(p.PA11, Level::Low, Speed::Low);

    rprintln!("Hello from STM32");
    loop {
        led.set_high();
        Timer::after_millis(500).await;
        led.set_low();
        Timer::after_millis(500).await;
    }
}
