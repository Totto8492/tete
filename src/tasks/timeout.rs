use embassy_time::{Duration, Timer};
use rtt_target::rprintln;

#[embassy_executor::task]
pub async fn task() {
    for i in (1..=5).rev() {
        rprintln!("{}...", i);
        Timer::after(Duration::from_millis(1000)).await;
    }
    rprintln!("reset_to_usb_boot");
    Timer::after(Duration::from_millis(1000)).await;
    embassy_rp::rom_data::reset_to_usb_boot(0, 0);
}
