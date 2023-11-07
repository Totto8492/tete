use embassy_time::Timer;

#[embassy_executor::task]
pub async fn task() {
    loop {
        Timer::after_millis(500).await;
    }
}
