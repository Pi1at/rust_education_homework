use std::{
    io::{Read, Write},
    net::TcpListener,
};
use tcp_plug_socket::{Command, OddResponse};

/// Imitate odd TCP Smart plug socket, with no floats avaiable
fn main() {
    let default_address =
        std::env::var("TCP_PLUG_ADDRESS").unwrap_or_else(|_| "127.0.0.1:6969".into());
    let server_address = std::env::args().nth(1).unwrap_or(default_address);

    let listener = TcpListener::bind(server_address).expect("can't bind tcp listener");

    let mut smart_socket = SmartSocket::default();

    while let Some(connection) = listener.incoming().next() {
        let mut stream = match connection {
            Ok(conn) => conn,
            Err(err) => {
                eprintln!("can't receive connection: {err}");
                continue;
            }
        };

        let peer = stream
            .peer_addr()
            .map_or("unknown".into(), |pa| pa.to_string());

        eprintln!("Peer '{peer}' connected");

        let mut in_buffer = [0u8];
        while stream.read_exact(&mut in_buffer).is_ok() {
            let response = &smart_socket.process_command(in_buffer[0].into());
            let response_buf: [u8; 4] = response.into();
            if stream.write_all(&response_buf).is_err() {
                break;
            };
        }

        println!("Connection with {peer} lost. Waiting for new connections...");
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
