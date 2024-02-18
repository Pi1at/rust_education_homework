use std::time::Duration;

use smarthome::devices::Gauge;
use tokio::time::sleep;
use udp_thermpmeter_async::UdpThermo;

#[tokio::main]
async fn main() {
    let receiver_address = "127.0.0.1:4342";
    let thermo = UdpThermo::new(receiver_address).await.unwrap();
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
