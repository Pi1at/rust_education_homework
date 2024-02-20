use std::fmt;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Default)]
pub enum OddResponse {
    #[default]
    Ok,
    Enabled,
    Disabled,
    Retry,
    Power(u16),
    MaxPower(u16),
    Reserved([u8; 3]),
}

impl From<&[u8; 4]> for OddResponse {
    fn from(&bytes: &[u8; 4]) -> Self {
        match bytes {
            [0, ..] => Self::Ok,
            [1, ..] => Self::Enabled,
            [2, ..] => Self::Disabled,
            [3, ..] => Self::Retry,
            [4, ..] => Self::Power(u16::from_be_bytes(
                bytes[2..].try_into().unwrap_or_default(),
            )),

            [5, ..] => Self::MaxPower(u16::from_be_bytes(
                bytes[2..].try_into().unwrap_or_default(),
            )),
            _ => {
                let mut v = [0u8; 3];
                v.clone_from_slice(&bytes[1..4]);
                Self::Reserved(v)
            }
        }
    }
}

impl From<&OddResponse> for [u8; 4] {
    fn from(value: &OddResponse) -> Self {
        let mut buf = [0u8; 4];
        match value {
            OddResponse::Ok => {}
            OddResponse::Enabled => buf[0] = 1,
            OddResponse::Disabled => buf[0] = 2,
            OddResponse::Retry => buf[0] = 3,

            OddResponse::Power(pw) => {
                buf[0] = 4;
                buf[2..].copy_from_slice(&pw.to_be_bytes());
            }
            OddResponse::MaxPower(mpw) => {
                buf[0] = 5;
                buf[2..].copy_from_slice(&mpw.to_be_bytes());
            }
            OddResponse::Reserved(b3) => {
                buf[0] = 6;
                buf[1..].copy_from_slice(b3);
            }
        };
        buf
    }
}

impl fmt::Display for OddResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ok => write!(f, "Ok"),
            Self::Enabled => write!(f, "Enabled"),
            Self::Disabled => write!(f, "Disabled"),
            Self::Power(power) => write!(f, "Power: {power}"),
            Self::MaxPower(power) => write!(f, "Max power: {power}"),
            Self::Reserved(v) => write!(f, "Reserved {v:?}"),
            Self::Retry => write!(f, "Retry later"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub enum Response {
    #[default]
    Ok,
    Enabled,
    Disabled,
    Power(f32),
    MaxPower(f32),
    Reserved([u8; 3]),
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ok => write!(f, "Ok"),
            Self::Enabled => write!(f, "Enabled"),
            Self::Disabled => write!(f, "Disabled"),
            Self::Power(power) => write!(f, "Power: {power}"),
            Self::MaxPower(power) => write!(f, "Max power: {power}"),
            Self::Reserved(v) => write!(f, "Reserved {v:?}"),
        }
    }
}