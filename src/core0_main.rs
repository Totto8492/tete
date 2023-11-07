use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_time::Timer;

#[embassy_executor::task]
pub async fn task(p: AnyPin) {
    let mut led = Output::new(p, Level::High);
    loop {
        Timer::after_millis(200).await;
        led.toggle();
    }
}
