use std::{
    fmt::{self, Display, Formatter},
    net::{ToSocketAddrs, UdpSocket},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use smarthome::devices::Gauge;

pub type Result<T> = core::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error>; // For early dev.

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Temperature(f32);

impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
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
    pub fn new(address: impl ToSocketAddrs) -> Result<Self> {
        let socket = UdpSocket::bind(address)?;
        // by default timeout is None
        socket.set_read_timeout(Some(Duration::from_secs(1)))?;

        let finished = Arc::new(AtomicBool::new(false));
        let temperature = Arc::new(Mutex::from(None));

        let temperature_clone = temperature.clone();
        let finished_clone = finished.clone();
        thread::spawn(move || loop {
            if finished_clone.load(Ordering::SeqCst) {
                return;
            }
            // TODO: handle as errors maybe
            let mut buf = [0; 4];
            let res = match socket.recv_from(&mut buf) {
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
