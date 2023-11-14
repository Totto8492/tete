use embassy_time::{Instant, Timer};

#[embassy_executor::task]
pub async fn task() {
    loop {
        Timer::at(Instant::MAX).await;
    }
}
