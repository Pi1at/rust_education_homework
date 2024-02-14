use anyhow::Result;
use smarthome::devices::SendCommand;
use std::{io, thread::sleep, time::Duration};
use tcp_plug_socket::{Command, TcpPlugOddSocket};
fn main() -> Result<()> {
    let plug_addr = "127.0.0.1:6969";
    eprintln!("Connecting to plug with address {plug_addr}");
    let mut test_socket = TcpPlugOddSocket::new(plug_addr);
    while test_socket.is_err() {
        eprint!(".");
        sleep(Duration::from_secs(1));
        test_socket = TcpPlugOddSocket::new(plug_addr);
        continue;
    }
    let mut odd_socket = test_socket.expect("no error expected there");
    show_menu();
    let mut input = io::stdin().lines().flatten();
    loop {
        eprint!("> ");
        if let Ok(cmd) = input
            .next()
            .expect("input end")
            .trim()
            .parse::<u8>()
            .map(Command::from)
        {
            let x = odd_socket
                .send_command(cmd)
                .expect("no error should be here");
            eprintln!("{x}");
        } else {
            // just exit
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
