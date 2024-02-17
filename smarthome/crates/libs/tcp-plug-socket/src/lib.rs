use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

pub use self::command::Command;
pub use self::responses::{OddResponse, Response};
use error::WrongResponse;
use smarthome::devices::{Gauge, SendCommand};

type Result<T> = core::result::Result<T, Error>;
type Error = Box<dyn std::error::Error>; // For early dev.

mod command;
mod error;
mod responses;

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

/// returns cached power usage
impl Gauge<Power> for TcpPlugOddSocket {
    type R = Power;
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
        Ok(self.convert_response(odd_resp))
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
            _ => return Err(WrongResponse::new(format!("{res}")).into()),
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
