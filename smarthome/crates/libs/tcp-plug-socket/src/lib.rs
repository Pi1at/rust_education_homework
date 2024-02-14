use std::{
    error,
    fmt::{self, Display},
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

use smarthome::devices::{Gauge, SendCommand};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Default)]
pub enum Command {
    #[default]
    TurnOff,
    TurnOn,
    IsEnabled,
    GetCurrentPowerUsage,
    GetMaxPowerUsage,
    Reserved {
        command_id: u8,
    },
}

impl From<u8> for Command {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::TurnOff,
            1 => Self::TurnOn,
            2 => Self::IsEnabled,
            3 => Self::GetCurrentPowerUsage,
            4 => Self::GetMaxPowerUsage,
            command_id => Self::Reserved { command_id },
        }
    }
}

impl From<Command> for u8 {
    fn from(value: Command) -> Self {
        match value {
            Command::TurnOff => 0,
            Command::TurnOn => 1,
            Command::IsEnabled => 2,
            Command::GetCurrentPowerUsage => 3,
            Command::GetMaxPowerUsage => 4,
            Command::Reserved { command_id } => command_id,
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TurnOff => write!(f, "Turn off"),
            Self::TurnOn => write!(f, "Turn on"),
            Self::IsEnabled => write!(f, "Is enabled"),
            Self::GetCurrentPowerUsage => write!(f, "Get current power usage"),
            Self::GetMaxPowerUsage => write!(f, "Get max power usage"),
            Self::Reserved { command_id } => write!(f, "Reserved :[{command_id}]"),
        }
    }
}

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

/// Odd TCP Smart Plug socket client- connects to smart plug via `TcpStream`
/// let's pretend for some reason `HW` implementation can't handle floats
/// so workaround needed
pub struct TcpPlugOddSocket {
    stream: TcpStream,
    delimiter: f32,
    cached_pu: f32,
}

#[derive(Clone, Copy)]
struct Power(f32);

#[derive(Debug, Clone)]
struct WrongResponse;
impl fmt::Display for WrongResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WrongResponse")
    }
}
impl error::Error for WrongResponse {}

/// returns cached power usage
impl Gauge<Power> for TcpPlugOddSocket {
    fn get_measure(&self) -> Power {
        Power(self.cached_pu)
    }
}

impl SendCommand<Command> for TcpPlugOddSocket {
    type R = Result<Response>;
    #[must_use]
    fn send_command(&mut self, command: Command) -> Self::R {
        let odd_resp = self.send_command(u8::from(command))?;
        self.update_state(odd_resp);
        let x = self.convert_response(odd_resp);
        Ok(x)
    }
}

impl SendCommand<u8> for TcpPlugOddSocket {
    type R = Result<OddResponse>;
    fn send_command(&mut self, raw_command: u8) -> Self::R {
        self.stream.write_all(&[raw_command])?;
        // reading OddResponse
        let mut buffer = [0u8; 4];
        self.stream.read_exact(&mut buffer)?;
        // now we need convert it to Response type
        Ok((&buffer).into())
    }
}

impl TcpPlugOddSocket {
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if connection or calibration fails
    pub fn new(plug_addr: impl ToSocketAddrs) -> Result<Self> {
        let stream = TcpStream::connect(plug_addr)?;
        let calibrated = Self {
            stream,
            delimiter: 1.0,
            cached_pu: 0.0,
        }
        .calibrate()?;
        Ok(calibrated)
    }
    fn calibrate(mut self) -> Result<Self> {
        let raw: u8 = Command::Reserved { command_id: 42 }.into();
        let res = self.send_command(raw)?;
        match res {
            OddResponse::Reserved(buf) if buf[0] == 42u8 => {
                self.delimiter = u16::from_be_bytes(buf[1..].try_into().unwrap_or([0, 1])).into();
            }
            _ => return Err(WrongResponse.into()),
        };
        Ok(self)
    }
    fn scale_power(&self, power: u16) -> f32 {
        f32::from(power) / self.delimiter
    }
    fn update_state(&mut self, r: OddResponse) {
        match r {
            OddResponse::Power(power) => self.cached_pu = self.scale_power(power),
            OddResponse::Ok
            | OddResponse::Enabled
            | OddResponse::Disabled
            | OddResponse::Retry
            | OddResponse::MaxPower(_)
            | OddResponse::Reserved(_) => {}
        }
    }

    fn convert_response(&self, v: OddResponse) -> Response {
        match v {
            OddResponse::Ok => Response::Ok,
            OddResponse::Enabled => Response::Enabled,
            OddResponse::Disabled => Response::Disabled,
            OddResponse::Retry => Response::Reserved([0, 0, 0]),
            OddResponse::Power(power) => Response::Power(self.scale_power(power)),
            OddResponse::MaxPower(power) => Response::MaxPower(self.scale_power(power)),
            OddResponse::Reserved(v) => Response::Reserved(v),
        }
    }
}
