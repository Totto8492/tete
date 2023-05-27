use embassy_rp::gpio::Output;
use embassy_rp::peripherals::PIN_25;
use rtt_target::rprintln;

use crate::{LedState, CHANNEL};

#[embassy_executor::task]
pub async fn task(mut led: Output<'static, PIN_25>) {
    rprintln!("Hello from core 1");
    loop {
        match CHANNEL.recv().await {
            LedState::On => led.set_high(),
            LedState::Off => led.set_low(),
        }
    }
}
