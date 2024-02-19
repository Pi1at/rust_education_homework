use std::time::Duration;
use tokio::time::sleep;

use smarthome::devices::Gauge;
use udp_thermometer::{impl_async::AsyncVer, UdpThermo};

#[tokio::main]
async fn main() {
    let receiver_address = "127.0.0.1:4342";
    let thermo = match UdpThermo::<AsyncVer>::new(receiver_address).await {
        Ok(thermo) => thermo,
        Err(e) => {
            eprintln!("Failed to create UdpThermo: {e}");
            return;
        }
    };
    for i in 0..120 {
        sleep(Duration::from_secs(1)).await;
        print!("Measure #{i} - ");
        thermo.get_measure().map_or_else(
            || {
                println!("got nothing");
            },
            |t| {
                println!("the temperature is {t:.2}");
            },
        );
    }
}
