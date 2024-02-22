use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs};

use crate::{error::Error, Command, OddResponse, Response};
use smarthome::devices::{Gauge, SendCommandAsync};

type Result<T> = core::result::Result<T, Error>;

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

impl SendCommandAsync<Command> for TcpPlugOddSocket {
    type R = Result<Response>;
    #[must_use]
    async fn send_command(&mut self, command: Command) -> Self::R {
        self.send_command(u8::from(command)).await.map(|resp| {
            self.update_state(resp);
            self.convert_response(resp)
        })
    }
}

impl SendCommandAsync<u8> for TcpPlugOddSocket {
    type R = Result<OddResponse>;
    async fn send_command(&mut self, raw_command: u8) -> Self::R {
        self.stream.write_all(&[raw_command]).await?;
        // reading OddResponse
        let mut buffer = [0u8; 4];
        self.stream.read_exact(&mut buffer).await?;
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
    pub async fn new<T: ToSocketAddrs + Send>(plug_addr: T) -> Result<Self> {
        let stream = TcpStream::connect(plug_addr).await?;
        Self {
            stream,
            delimiter: 1.0,
            cached_pu: 0.0,
        }
        .calibrate()
        .await
    }
    async fn calibrate(mut self) -> Result<Self> {
        const CALIBRATE_CMD: Command = Command::Reserved { command_id: 42 };
        let raw: u8 = CALIBRATE_CMD.into();
        let res = self.send_command(raw).await?;
        match res {
            OddResponse::Reserved(buf) if buf[0] == 42u8 => {
                self.delimiter = u16::from_be_bytes(buf[1..].try_into().unwrap_or([0, 1])).into();
            }
            _ => {
                return Err(Error::WrongResponse {
                    cmd: CALIBRATE_CMD,
                    resp: res,
                })
            }
        };
        Ok(self)
    }
    fn scale_power(&self, power: u16) -> f32 {
        f32::from(power) / self.delimiter
    }
    fn update_state(&mut self, r: OddResponse) {
        if let OddResponse::Power(power) = r {
            self.cached_pu = self.scale_power(power);
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
