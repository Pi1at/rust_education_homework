use std::{
    marker::PhantomData,
    net::{ToSocketAddrs, UdpSocket},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};
pub struct SyncVer;

use crate::Result;
use crate::{temperature::Temperature, UdpThermo};

impl UdpThermo<SyncVer> {
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
                    Some(Temperature::from_be_bytes(buf))
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
            _sync: PhantomData,
        })
    }
}
