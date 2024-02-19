use std::fmt::Display;

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
            Self::Reserved { command_id } => {
                write!(f, "Reserved :[")?;
                command_id.fmt(f)?;
                write!(f, "]")
            }
        }
    }
}
