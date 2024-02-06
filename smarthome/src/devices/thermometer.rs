type Tempreature = f32;

#[derive(Clone, Debug)]
pub struct SmartThermometer {
    pub name: String,
}

impl SmartThermometer {
    pub const fn new(name: String) -> Self {
        Self { name }
    }

    pub const fn get_temperature(&self) -> Tempreature {
        // TODO
        10.0
    }
}
