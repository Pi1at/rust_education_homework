use crate::location::DeviceName;

type Tempreature = f32;

#[derive(Clone, Debug)]
pub struct SmartThermometer {
    pub name: DeviceName,
}

impl SmartThermometer {
    #[must_use]
    pub const fn new(name: DeviceName) -> Self {
        Self { name }
    }

    #[must_use]
    pub const fn get_temperature(&self) -> Tempreature {
        // TODO
        10.0
    }
}
