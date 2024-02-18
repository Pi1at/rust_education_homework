use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};

use tokio::{net::UdpSocket, time::sleep};
use udp_thermpmeter_async::Temperature;

#[tokio::main]
async fn main() {
    let receiver = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:4342".into())
        .parse::<SocketAddr>()
        .expect("valid socket address expected");

    let bind_addr = "127.0.0.1:4320";
    let socket = UdpSocket::bind(bind_addr).await.expect("can't bind socket");
    let temperature_generator = TemperatureGenerator::default();

    println!("Starting send temperature from {bind_addr} to {receiver}");
    loop {
        let bytes = temperature_generator.generate().to_be_bytes();
        if let Err(err) = socket.send_to(&bytes, receiver).await {
            eprintln!("can't send temperature: {err}");
        }
        sleep(Duration::from_secs(1)).await;
    }
}

// t = ampl*sin(seconds*koeff) + mid
struct TemperatureGenerator {
    started: Instant,
    ampl: f32,
    coeff: f32,
    mid: f32,
}

impl Default for TemperatureGenerator {
    fn default() -> Self {
        Self {
            started: Instant::now(),
            ampl: 3.0,
            coeff: 0.2,
            mid: 15.0,
        }
    }
}

impl TemperatureGenerator {
    pub fn generate(&self) -> Temperature {
        self.ampl
            .mul_add(
                (self.started.elapsed().as_secs_f32() * self.coeff).sin(),
                self.mid,
            )
            .into()
    }
}
