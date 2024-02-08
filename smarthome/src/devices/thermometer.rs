type Tempreature = f32;

#[derive(Clone, Debug)]
pub struct SmartThermometer {
    pub name: String,
}

impl SmartThermometer {
    #[must_use]
    pub const fn new(name: String) -> Self {
        Self { name }
    }

    #[must_use]
    pub const fn get_temperature(&self) -> Tempreature {
        // TODO
        10.0
    }
}
