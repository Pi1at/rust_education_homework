use anyhow::{anyhow, Result};
use eframe::NativeOptions;
use smarthome::devices::SendCommandAsync;
use std::time::Duration;
use tcp_plug_socket::{impl_async::TcpPlugOddSocket, Command, Response};
use tokio::{
    sync::mpsc::{self},
    time::sleep,
};

use crate::app::TcpPlug;

mod app;
mod logger;

const MAX_RETRY_LIMIT: i32 = 30;

#[tokio::main]
async fn main() -> Result<()> {
    logger::setup();
    let default_address =
        std::env::var("TCP_PLUG_ADDRESS").unwrap_or_else(|_| "127.0.0.1:6969".into());
    let plug_addr = std::env::args().nth(1).unwrap_or(default_address);
    tracing::info!("Connecting to plug with address {plug_addr}");
    let mut test_socket = TcpPlugOddSocket::new(&plug_addr).await;
    let mut retry_count = 0;
    while test_socket.is_err() {
        if retry_count >= MAX_RETRY_LIMIT {
            return Err(anyhow!(
                "Failed to connect after {MAX_RETRY_LIMIT} attempts"
            ));
        }
        sleep(Duration::from_secs(1)).await;
        test_socket = TcpPlugOddSocket::new(&plug_addr).await;
        retry_count += 1;
        continue;
    }
    let mut odd_socket = test_socket.expect("no error expected there");

    let (command_sender, mut command_reciever) = mpsc::unbounded_channel::<Command>();
    let (response_sender, response_reciever) = mpsc::unbounded_channel::<Response>();

    // send command to the socket
    tokio::task::spawn(async move {
        loop {
            if let Some(cmd) = command_reciever.recv().await {
                let x = odd_socket
                    .send_command(cmd)
                    .await
                    .expect("probably tcp socket imitator crashed");
                tracing::debug!("Sending response: {x} to channel");
                let _ = response_sender.send(x);
            }
        }
    });

    // produce update commands
    let state_updater = command_sender.clone();
    tokio::task::spawn(async move {
        loop {
            let _ = state_updater.send(Command::GetCurrentPowerUsage);
            let _ = state_updater.send(Command::IsEnabled);
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });

    let app = TcpPlug::new(command_sender, response_reciever);
    let options = NativeOptions::default();
    eframe::run_native(
        "TCP Plug Socket",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(app)
        }),
    )
    .map_err(|e| anyhow!("Got error : {}", e))
}
