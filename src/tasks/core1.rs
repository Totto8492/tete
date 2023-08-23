use embassy_rp::gpio::Output;
use embassy_rp::peripherals::PIN_25;
use rtt_target::rprintln;

use crate::tasks::state::{Led, CHANNEL};

#[embassy_executor::task]
pub async fn task(mut led: Output<'static, PIN_25>) {
    rprintln!("Hello from core 1");
    loop {
        match CHANNEL.receive().await {
            Led::On => led.set_high(),
            Led::Off => led.set_low(),
        }
    }
}
