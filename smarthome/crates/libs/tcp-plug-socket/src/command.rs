use derive_more::Display;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Default, Display)]
pub enum Command {
    #[default]
    #[display(fmt = "Turn off")]
    TurnOff,
    #[display(fmt = "Turn on")]
    TurnOn,
    #[display(fmt = "Is enabled")]
    IsEnabled,
    #[display(fmt = "Get current power usage")]
    GetCurrentPowerUsage,
    #[display(fmt = "Get max power usage")]
    GetMaxPowerUsage,
    #[display(fmt = "Reserved: [{command_id}]")]
    Reserved { command_id: u8 },
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
