use std::{thread, time::Duration};

use smarthome::devices::Gauge;
use udp_thermometer::UdpThermo;

fn main() {
    let receiver_address = "127.0.0.1:4342";
    let thermo = UdpThermo::new(receiver_address).unwrap();
    for i in 0..120 {
        thread::sleep(Duration::from_secs(1));
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
