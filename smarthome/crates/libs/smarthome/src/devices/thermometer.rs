use crate::location::DeviceName;

pub use super::Construct;
use super::Gauge;

type Temperature = f32;

#[derive(Clone, Debug)]
pub struct SmartThermometer {
    pub name: DeviceName,
}

impl Construct for SmartThermometer {
    #[must_use]
    fn new(name: DeviceName) -> Self {
        Self { name }
    }
}
impl Gauge<Temperature> for SmartThermometer {
    type R = Temperature;
    #[must_use]
    fn get_measure(&self) -> Temperature {
        10.0
    }
}

impl SmartThermometer {
    #[must_use]
    pub fn get_temperature(&self) -> Temperature {
        self.get_measure()
    }
}
