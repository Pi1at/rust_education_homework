use anyhow::{anyhow, Result};
use std::io;
use std::time::Duration;
use tokio::time::sleep;

use smarthome::devices::SendCommandAsync;
use tcp_plug_socket::{impl_async::TcpPlugOddSocket, Command};

#[tokio::main]
async fn main() -> Result<()> {
    const MAX_RETRY_LIMIT: i32 = 30;
    let default_address =
        std::env::var("TCP_PLUG_ADDRESS").unwrap_or_else(|_| "127.0.0.1:6969".into());
    let plug_addr = std::env::args().nth(1).unwrap_or(default_address);
    eprint!("Connecting to plug with address {plug_addr}");
    let mut test_socket = TcpPlugOddSocket::new(&plug_addr).await;
    let mut retry_count = 0;
    while test_socket.is_err() {
        if retry_count >= MAX_RETRY_LIMIT {
            return Err(anyhow!(
                "Failed to connect after {MAX_RETRY_LIMIT} attempts"
            ));
        }
        eprint!(".");
        sleep(Duration::from_secs(1)).await;
        test_socket = TcpPlugOddSocket::new(&plug_addr).await;
        retry_count += 1;
        continue;
    }
    let mut odd_socket = test_socket.expect("no error expected there");
    show_menu();
    let mut input = io::stdin().lines().map_while(Result::ok);
    loop {
        eprint!("> ");
        if let Ok(cmd) = input
            .next()
            .ok_or_else(|| anyhow!("Input stream ended unexpectedly"))?
            .trim()
            .parse::<u8>()
            .map(Command::from)
        {
            let x = odd_socket
                .send_command(cmd)
                .await
                .expect("no error should be here");
            eprintln!("{x}");
        } else {
            // just exit
            eprintln!("Bye...");
            break Ok(());
        }
    }
}

fn show_menu() {
    eprintln!();
    eprintln!("Input command:");
    for id in 0..5 {
        eprintln!("{id}: {}", Command::from(id));
    }
    eprintln!("anything else to exit (almost :P)");
}
