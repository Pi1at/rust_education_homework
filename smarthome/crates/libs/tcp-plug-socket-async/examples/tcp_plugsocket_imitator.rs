use std::sync::{Arc, Mutex};

use tcp_plug_socket_async::{Command, OddResponse};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

/// Imitate odd TCP Smart plug socket, with no floats avaiable
#[tokio::main]
async fn main() {
    let default_address =
        std::env::var("TCP_PLUG_ADDRESS").unwrap_or_else(|_| "127.0.0.1:6969".into());
    let server_address = std::env::args().nth(1).unwrap_or(default_address);

    let listener = TcpListener::bind(server_address)
        .await
        .expect("can't bind tcp listener");

    let smart_socket = Arc::new(Mutex::new(SmartSocket::default()));

    while let Ok((mut stream, peer_addr)) = listener.accept().await {
        eprintln!("Connection with peer {peer_addr} accepted");
        let smart_socket = smart_socket.clone();
        tokio::spawn(async move {
            let mut in_buffer = [0u8];
            while stream.read_exact(&mut in_buffer).await.is_ok() {
                // std-mutex in async code is perfectly fine, as long as you don't hold it when awaiting
                let response = &smart_socket
                    .lock()
                    .expect("shoudn't be locked by current thread")
                    .process_command(in_buffer[0].into());
                let response_buf: [u8; 4] = response.into();
                if stream.write_all(&response_buf).await.is_err() {
                    break;
                };
            }
            println!("Connection with {peer_addr} lost. Waiting for new connections...");
        });
    }
}

#[derive(Default)]
struct SmartSocket {
    enabled: bool,
}

impl SmartSocket {
    fn process_command(&mut self, cmd: Command) -> OddResponse {
        match cmd {
            Command::TurnOn => {
                self.enabled = true;
                OddResponse::Ok
            }
            Command::TurnOff => {
                self.enabled = false;
                OddResponse::Ok
            }
            Command::IsEnabled => {
                if self.enabled {
                    OddResponse::Enabled
                } else {
                    OddResponse::Disabled
                }
            }
            Command::GetCurrentPowerUsage => {
                if self.enabled {
                    OddResponse::Power(12345)
                } else {
                    OddResponse::Power(0)
                }
            }
            Command::GetMaxPowerUsage => OddResponse::MaxPower(240 * 16 * 10),
            Command::Reserved { command_id: 42 } => {
                eprintln!("raw command accepted");
                let buf = [42, 0, 10];
                OddResponse::Reserved(buf)
            }
            Command::Reserved { .. } => {
                eprintln!("Unknown command received");
                OddResponse::Ok
            }
        }
    }
}
