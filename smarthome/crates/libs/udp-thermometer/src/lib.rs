use std::marker::PhantomData;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use smarthome::devices::Gauge;
use temperature::Temperature;

pub mod impl_async;
pub mod impl_sync;
pub mod temperature;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

pub struct UdpThermo<S> {
    temperature: Arc<Mutex<Option<Temperature>>>,
    finished: Arc<AtomicBool>,
    _sync: PhantomData<S>,
}

impl<S> Gauge<Temperature> for UdpThermo<S> {
    type R = Option<Temperature>;
    fn get_measure(&self) -> Self::R {
        *self.temperature.lock().unwrap()
    }
}

impl<S> Drop for UdpThermo<S> {
    fn drop(&mut self) {
        self.finished.store(true, Ordering::SeqCst);
    }
}
