// разрешим пока прототип
#![allow(dead_code)]
use smarthome::{
    devices::{socket::SmartSocket, thermometer::SmartThermometer},
    location::{home::SmartHome, room::Room},
    providers::DeviceInfoProvider,
};

struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_state(&self, _room: &str, device: &str) -> String {
        if self.socket.name == device {
            format!(
                "{} power: {} W",
                device,
                self.socket.get_current_power_usage()
            )
        } else {
            format!("ERROR: Device {device} not found")
        }
    }
}

struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_device_state(&self, _room: &str, device: &str) -> String {
        if self.socket.name == device {
            format!(
                "{} power: {} W",
                device,
                self.socket.get_current_power_usage()
            )
        } else if self.thermo.name == device {
            format!("{} {} °C", device, self.thermo.get_temperature())
        } else {
            format!("ERROR: Device {device} not found")
        }
    }
}

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket::new("Kitchen socket".into());
    let socket2 = SmartSocket::new("Outdoor socket".into());
    let thermo = SmartThermometer::new("Outdoor thermo".into());
    let kitchen = Room::new("Kitchen".into()).with_device(socket1.name.clone());
    let outdoor = Room::new("Outdoor".into())
        .with_devices([socket2.name.clone(), thermo.name.clone()].into_iter());
    // Инициализация дома
    let house = SmartHome::new("House").with_rooms([kitchen, outdoor].into_iter());

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let mut info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    info_provider_1.socket.turn_on();
    let report1 = house.create_report(&info_provider_1);
    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: \n{report1}");
    println!("Report #2: \n{report2}");
}
