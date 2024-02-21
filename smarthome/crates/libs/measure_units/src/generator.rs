use std::time::Instant;

use crate::temperature::Temperature;

// t = ampl*sin(seconds*koeff) + mid
// TODO: rename to better match meaning or change implementation
pub struct Generator {
    started: Instant,
    ampl: f32,
    coeff: f32,
    mid: f32,
}
// TODO: Provide more generic implementation
impl Iterator for Generator {
    type Item = Temperature;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.generate())
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            started: Instant::now(),
            ampl: 3.0,
            coeff: 0.2,
            mid: 15.0,
        }
    }
}

impl Generator {
    #[must_use]
    pub fn generate<T: From<f32>>(&self) -> T {
        T::from(self.ampl.mul_add(
            (self.started.elapsed().as_secs_f32() * self.coeff).sin(),
            self.mid,
        ))
    }
}
