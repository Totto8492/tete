use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;

pub static CHANNEL: Channel<CriticalSectionRawMutex, Led, 1> = Channel::new();

pub enum Led {
    On,
    Off,
}
