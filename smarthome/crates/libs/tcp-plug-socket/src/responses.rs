use derive_more::Display;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Default, Display)]
pub enum OddResponse {
    #[default]
    Ok,
    Enabled,
    Disabled,
    #[display("Retry later")]
    Retry,
    #[display("Power: {_0}")]
    Power(u16),
    #[display("Max power: {_0}")]
    MaxPower(u16),
    #[display("Reserved {_0:?}")]
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

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default, Display)]
pub enum Response {
    #[default]
    Ok,
    Enabled,
    Disabled,
    #[display("Power: {_0}")]
    Power(f32),
    #[display("Max power: {_0}")]
    MaxPower(f32),
    #[display("Reserved: {_0:?}")]
    Reserved([u8; 3]),
}
