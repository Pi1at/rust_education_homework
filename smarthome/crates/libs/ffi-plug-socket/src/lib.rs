#![allow(clippy::missing_panics_doc, clippy::not_unsafe_ptr_arg_deref)]
use std::convert::Infallible;
use std::os::raw::c_char;

use smarthome::devices::{Gauge, SendCommand};

#[no_mangle]
pub extern "C" fn smartsocket_new(name: *const c_char) -> *mut SmartSocket {
    Box::into_raw(Box::new(SmartSocket::new(name)))
}

#[no_mangle]
pub extern "C" fn smartsocket_dealloc(socket: *mut SmartSocket) {
    if socket.is_null() {
        return;
    }
    drop(unsafe { Box::from_raw(socket) });
}

#[no_mangle]
pub extern "C" fn smartsocket_get_current_power_usage(socket: *const SmartSocket) -> f32 {
    let socket = unsafe { socket.as_ref().unwrap() };
    socket.get_current_power_usage()
}

#[no_mangle]
pub extern "C" fn smartsocket_is_enabled(socket: *const SmartSocket) -> bool {
    let socket = unsafe { socket.as_ref().unwrap() };
    socket.is_enabled()
}

#[no_mangle]
pub extern "C" fn smartsocket_turn_on(socket: *mut SmartSocket) {
    let socket = unsafe { socket.as_mut().unwrap() };
    socket.turn_on();
}

#[no_mangle]
pub extern "C" fn smartsocket_turn_off(socket: *mut SmartSocket) {
    let socket = unsafe { socket.as_mut().unwrap() };
    socket.turn_off();
}

type Watt = f32;

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SmartSocketState {
    On,
    Off,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct SmartSocket {
    name: *const c_char,
    power: Watt,
    state: SmartSocketState,
}

impl Gauge<Watt> for SmartSocket {
    type R = Watt;

    fn get_measure(&self) -> Watt {
        match self.state {
            SmartSocketState::On => self.power,
            SmartSocketState::Off => 0.0,
        }
    }
}

struct TurnOn;
struct TurnOff;

impl SendCommand<TurnOn> for SmartSocket {
    type R = Result<(), Infallible>;
    fn send_command(&mut self, _: TurnOn) -> Self::R {
        self.state = SmartSocketState::On;
        Ok(())
    }
}

impl SendCommand<TurnOff> for SmartSocket {
    type R = Result<(), Infallible>;
    fn send_command(&mut self, _: TurnOff) -> Self::R {
        self.state = SmartSocketState::Off;
        Ok(())
    }
}

impl Default for SmartSocket {
    fn default() -> Self {
        Self {
            name: "Socket".as_ptr().cast::<c_char>(),
            power: 1234.0,
            state: SmartSocketState::On,
        }
    }
}

impl SmartSocket {
    #[must_use]
    pub fn new(name: *const c_char) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    pub fn turn_on(&mut self) {
        let _ = self.send_command(TurnOn);
        self.power = 1234.0;
    }

    pub fn turn_off(&mut self) {
        let _ = self.send_command(TurnOff);
        self.power = 0.0;
    }

    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.state == SmartSocketState::On
    }

    #[must_use]
    pub fn get_current_power_usage(&self) -> Watt {
        self.get_measure()
    }
}
