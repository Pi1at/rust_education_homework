use egui::Color32;
use std::time::Duration;
use tcp_plug_socket::{Command, Response};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum PowerState {
    Enabled,
    Disabled,
}

impl From<PowerState> for Color32 {
    fn from(val: PowerState) -> Self {
        match val {
            PowerState::Enabled => Self::LIGHT_GREEN,
            PowerState::Disabled => Self::RED,
        }
    }
}

pub struct TcpPlug {
    command_sender: UnboundedSender<Command>,
    response_receiver: UnboundedReceiver<Response>,
    power: f32,
    state: PowerState,
}

impl TcpPlug {
    pub const fn new(
        sender: UnboundedSender<Command>,
        receiver: UnboundedReceiver<Response>,
    ) -> Self {
        Self {
            command_sender: sender,
            response_receiver: receiver,
            power: 0.0,
            state: PowerState::Disabled,
        }
    }
}

impl eframe::App for TcpPlug {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // repaint at most every 0.25s
            ctx.request_repaint_after(Duration::from_millis(16));
            // try to read all msgs and update state
            while let Ok(resp) = self.response_receiver.try_recv() {
                match resp {
                    Response::Enabled => self.state = PowerState::Enabled,
                    Response::Disabled => self.state = PowerState::Disabled,
                    Response::Power(p) => self.power = p,
                    Response::MaxPower(_) | Response::Ok | Response::Reserved(_) => {}
                }
            }

            // HDPI Window Scaling
            ctx.set_pixels_per_point(1.5);

            ui.vertical_centered(|ui| {
                ui.heading("TCP Plug Socket");
                ui.add_space(30.0);
                let enabled_label = match self.state {
                    PowerState::Disabled => "Power Off",
                    PowerState::Enabled => "Power On",
                };
                ui.colored_label(egui::Color32::from_rgb(255, 255, 255), enabled_label);
                ui.add_space(10.0);
                ui.colored_label(
                    egui::Color32::from_rgb(255, 255, 255),
                    format!("Current usage: {:.2} Watt", self.power),
                );
                ui.add_space(25.0);
                let power_button =
                    egui::ImageButton::new(egui::include_image!("../images/pwr-btn-128.png"))
                        .tint(self.state)
                        .frame(false);
                if ui.add_sized(ui.available_size(), power_button).clicked() {
                    let _ = match self.state {
                        PowerState::Enabled => self.command_sender.send(Command::TurnOff),
                        PowerState::Disabled => self.command_sender.send(Command::TurnOn),
                    };
                };
            });
        });
    }
}
