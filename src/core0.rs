use embassy_time::{Duration, Timer};
use rtt_target::rprintln as info;

use super::{LedState, CHANNEL};

#[embassy_executor::task]
pub async fn task() {
    info!("Hello from core 0");
    loop {
        CHANNEL.send(LedState::On).await;
        Timer::after(Duration::from_millis(100)).await;
        CHANNEL.send(LedState::Off).await;
        Timer::after(Duration::from_millis(400)).await;
    }
}
