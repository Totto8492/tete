use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;

pub(crate) static CHANNEL: Channel<CriticalSectionRawMutex, LedState, 1> = Channel::new();

pub(crate) enum LedState {
    On,
    Off,
}
