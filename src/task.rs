use embassy_executor::Executor;
use static_cell::StaticCell;

static EXECUTOR0: StaticCell<Executor> = StaticCell::new();

pub fn run_main(token: impl FnOnce(embassy_executor::Spawner) + Send + 'static) -> ! {
    let executor0 = EXECUTOR0.init(Executor::new());
    executor0.run(token);
}
