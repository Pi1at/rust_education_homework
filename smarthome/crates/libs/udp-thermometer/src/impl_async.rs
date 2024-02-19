use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use tokio::net::{ToSocketAddrs, UdpSocket};

use crate::Result;
use crate::{temperature::Temperature, UdpThermo};
pub struct AsyncVer;

impl UdpThermo<AsyncVer> {
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
                match temperature_clone.lock() {
                    Ok(mut temperature) => *temperature = res,
                    Err(e) => eprintln!("Failed to acquire lock: {e}"),
                }
            }
        });

        Ok(Self {
            temperature,
            finished,
            _sync: std::marker::PhantomData,
        })
    }
}
