use std::{
    fmt::{self, Display, Formatter},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

use smarthome::devices::Gauge;
use tokio::net::{ToSocketAddrs, UdpSocket};

pub type Result<T> = core::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error>; // For early dev.

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Temperature(f32);

impl From<f32> for Temperature {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Temperature {
    #[must_use]
    pub fn to_be_bytes(self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
}

pub struct UdpThermo {
    temperature: Arc<Mutex<Option<Temperature>>>,
    finished: Arc<AtomicBool>,
}

impl UdpThermo {
    /// # Panics
    /// may panic
    /// # Errors
    /// IO Errors expected
    pub async fn new<T: ToSocketAddrs + Send>(address: T) -> Result<Self> {
        let socket = UdpSocket::bind(address).await?;

        let finished = Arc::new(AtomicBool::new(false));
        let temperature = Arc::new(Mutex::from(None));

        let temperature_clone = temperature.clone();
        let finished_clone = finished.clone();
        tokio::spawn(async move {
            loop {
                if finished_clone.load(Ordering::SeqCst) {
                    return;
                }
                // TODO: handle as errors maybe
                let mut buf = [0; 4];
                let res = match socket.recv_from(&mut buf).await {
                    Ok((bytes_read, _)) if bytes_read == buf.len() => {
                        Some(Temperature(f32::from_be_bytes(buf)))
                    }
                    Ok((bytes_read, src_addr)) => {
                        eprintln!("only {bytes_read} bytes read from {src_addr}");
                        None
                    }
                    Err(err) => {
                        eprintln!("can't receive datagram: {err}");
                        None
                    }
                };
                *temperature_clone.lock().unwrap() = res;
            }
        });

        Ok(Self {
            temperature,
            finished,
        })
    }
}

impl Gauge<Temperature> for UdpThermo {
    type R = Option<Temperature>;
    fn get_measure(&self) -> Self::R {
        *self.temperature.lock().unwrap()
    }
}

impl Drop for UdpThermo {
    fn drop(&mut self) {
        self.finished.store(true, Ordering::SeqCst);
    }
}
