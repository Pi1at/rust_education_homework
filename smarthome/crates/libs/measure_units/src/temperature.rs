use derive_more::Display;
use derive_more::From;

/// No validation on purpose of generalization
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, From, Display)]
pub struct Temperature(f32);

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
