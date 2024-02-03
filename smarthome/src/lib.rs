// разрешим пока прототип
#![allow(dead_code)]
struct SmartTermometer {}
struct SmartSocket {}

impl SmartTermometer {
    const fn new() -> Self {
        Self {}
    }

    // TODO: f32 vs f64 vs isize vs (Nominator/Denominator)
    fn get_temperature(&self) -> f32 {
        todo!()
    }
}

impl SmartSocket {
    const fn new() -> Self {
        Self {}
    }

    fn description(&self) -> String {
        todo!()
    }

    fn turn_on(&self) {
        todo!()
    }

    fn turn_off(&self) {
        todo!()
    }

    // TODO: определиться с типом мощности
    fn get_current_power_usage(&self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    // #[test]
    // fn it_works() {
    //     //TODO: реализовать в дальнейшем тесты
    // }
}
