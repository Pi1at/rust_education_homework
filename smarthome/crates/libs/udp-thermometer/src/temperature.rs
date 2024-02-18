use std::{
    fmt::{self, Display, Formatter},
    time::Instant,
};

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Temperature(f32);

impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<f32> for Temperature {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl Temperature {
    #[must_use]
    pub fn to_be_bytes(self) -> [u8; 4] {
        self.0.to_be_bytes()
    }

    #[must_use]
    pub fn from_be_bytes(bytes: [u8; 4]) -> Self {
        Self(f32::from_be_bytes(bytes))
    }
}

#[allow(clippy::module_name_repetitions)] // t = ampl*sin(seconds*koeff) + mid
pub struct TemperatureGenerator {
    started: Instant,
    ampl: f32,
    coeff: f32,
    mid: f32,
}

impl Iterator for TemperatureGenerator {
    type Item = Temperature;

    fn next(&mut self) -> Option<Self::Item> {
        Some(
            self.ampl
                .mul_add(
                    (self.started.elapsed().as_secs_f32() * self.coeff).sin(),
                    self.mid,
                )
                .into(),
        )
    }
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
    #[must_use]
    pub fn generate(&self) -> Temperature {
        self.ampl
            .mul_add(
                (self.started.elapsed().as_secs_f32() * self.coeff).sin(),
                self.mid,
            )
            .into()
    }
}
