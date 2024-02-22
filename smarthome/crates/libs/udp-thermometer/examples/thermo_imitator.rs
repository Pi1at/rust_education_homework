use std::{
    net::{SocketAddr, UdpSocket},
    thread,
    time::Duration,
};

use measure_units::generator::Generator;

fn main() {
    let receiver = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:4342".into())
        .parse::<SocketAddr>()
        .expect("valid socket address expected");

    let bind_addr = "127.0.0.1:4320";
    let socket = UdpSocket::bind(bind_addr).expect("can't bind socket");

    println!("Starting send temperature from {bind_addr} to {receiver}");
    for temperature in Generator::default() {
        let bytes = temperature.to_be_bytes();
        if let Err(err) = socket.send_to(&bytes, receiver) {
            eprintln!("can't send temperature: {err}");
        }
        thread::sleep(Duration::from_secs(1));
    }
}
