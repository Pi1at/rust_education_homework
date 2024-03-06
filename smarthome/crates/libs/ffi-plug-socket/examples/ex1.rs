#![allow(clippy::missing_panics_doc, clippy::not_unsafe_ptr_arg_deref)]
use std::{
    ffi::{c_char, CStr, CString},
    fmt::Display,
};

#[link(name = "plug_socket", kind = "dylib")]
extern "C" {
    fn smartsocket_new(name: *const c_char) -> *mut SmartSocket;
    fn smartsocket_get_current_power_usage(ptr: *const SmartSocket) -> Power;
    fn smartsocket_is_enabled(ptr: *const SmartSocket) -> bool;
    fn smartsocket_turn_on(ptr: *mut SmartSocket);
    fn smartsocket_turn_off(ptr: *mut SmartSocket);
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum SmartSocketState {
    On,
    Off,
}

impl Display for SmartSocketState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::On => write!(f, "On"),
            Self::Off => write!(f, "Off"),
        }
    }
}

pub type Power = f32;

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct SmartSocket {
    name: *const c_char,
    power: Power,
    state: SmartSocketState,
}

impl Default for SmartSocket {
    fn default() -> Self {
        Self::new("")
    }
}

impl SmartSocket {
    #[must_use]
    pub fn new(name: &str) -> Self {
        unsafe { *smartsocket_new(CString::new(name).unwrap().into_raw()) }
    }

    #[must_use]
    pub fn description(&self) -> String {
        format!(
            r#"
       Name:  {name}
       Power: {power:.2} Wh
       State: {state}
            "#,
            name = unsafe { CStr::from_ptr(self.name).to_str().unwrap() },
            power = self.power,
            state = self.state
        )
    }

    #[must_use]
    pub fn power(&self) -> Power {
        unsafe { smartsocket_get_current_power_usage(self) }
    }

    #[must_use]
    pub fn is_enabled(&self) -> bool {
        unsafe { smartsocket_is_enabled(self) }
    }

    pub fn turn_off(&mut self) {
        unsafe { smartsocket_turn_off(self) }
    }

    pub fn turn_on(&mut self) {
        unsafe { smartsocket_turn_on(self) }
    }
}

fn main() {
    println!("Creating new socket");
    let mut smart_socket = SmartSocket::new("Socket");

    assert!(smart_socket.is_enabled());
    assert!((smart_socket.power() - 1234.0).abs() < f32::EPSILON);
    println!("{}", smart_socket.description());

    print!("Turning off");
    smart_socket.turn_off();

    assert!(!smart_socket.is_enabled());
    assert!(smart_socket.power() < f32::EPSILON);

    println!("{}", smart_socket.description());
}
