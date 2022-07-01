mod gui;
mod xmodem;

use eframe::egui;
use gui::*;
use serialport::SerialPort;
use std::fs::File;
use std::time::Duration;
use xmodem::XModem;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "651R2/A Firmware Upgrade Application",
        options,
        Box::new(|cc| Box::new(Terminal::new(cc))),
    );
}

struct Terminal {
    selected_comport: String,
    comports: Vec<String>,
    buadrates: Vec<u32>,
    console_text: String,
    serial_settings_flag: bool,
    serial_port: Option<Box<dyn SerialPort>>,
    port_connected: bool,
    port_settings: SerialPortSettings,
}

impl Terminal {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let serial_ports = serialport::available_ports().unwrap();

        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self {
            selected_comport: "".to_owned(),
            comports: serial_ports
                .iter()
                .map(|port| port.port_name.clone())
                .collect(),
            buadrates: vec![
                110, 300, 600, 1200, 2400, 4800, 9600, 14400, 19200, 38400, 57600, 115200, 230400,
                460800, 921600,
            ],
            console_text: "".to_owned(),
            serial_settings_flag: false,
            serial_port: None,
            port_connected: false,
            port_settings: SerialPortSettings::default(),
        }
    }
}

impl eframe::App for Terminal {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("Menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Transfer", |ui| {
                    if ui.button("xModem Send").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            let picked_path = path.display().to_string();
                            let port = self.serial_port.as_mut().unwrap();
                            let stream = File::open(picked_path).unwrap();
                            match XModem::new().send(port, Box::new(stream)) {
                                Ok(()) => println!("File Send success"),
                                Err(err) => println!("Error: {err}"),
                            }
                        }
                    }
                    if ui.button("xModem Receive").clicked() {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            let picked_path = path.display().to_string();
                            let port = self.serial_port.as_mut().unwrap();
                            let stream = File::create(picked_path).unwrap();
                            match XModem::new().receive(port, Box::new(stream), false) {
                                Ok(bytes) => println!("File Receive success, Bytes: {bytes} read."),
                                Err(err) => println!("Error: {err}"),
                            }
                        }
                    }
                });
                ui.menu_button("Sessions", |ui| if ui.button("New session").clicked() {});
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                comport_setting_combo_box(ui, &mut self.selected_comport, &self.comports);
                if ui.button("Refresh Ports").clicked() {
                    let serial_ports = serialport::available_ports().unwrap();
                    self.comports = serial_ports
                        .iter()
                        .map(|port| port.port_name.clone())
                        .collect();
                    println!("Serial Ports {:?}", self.comports);
                }
                buadrate_setting_combo_box(ui, &mut self.port_settings.baud_rate, &self.buadrates);
                if self.port_connected {
                    if ui.button("Disconnect").clicked() {
                        self.serial_port = None;
                        self.port_connected = false;
                        println!("Disconnected Port");
                    }
                } else {
                    if ui.button("Connect").clicked() {
                        if self.selected_comport.len() > 0 {
                            if let Ok(port) = serialport::new(
                                self.selected_comport.clone(),
                                self.port_settings.baud_rate,
                            )
                            .data_bits(self.port_settings.data_bits)
                            .flow_control(self.port_settings.flow_control)
                            .parity(self.port_settings.parity)
                            .stop_bits(self.port_settings.stop_bits)
                            .timeout(Duration::from_millis(self.port_settings.timeout))
                            .open()
                            {
                                self.serial_port = Some(port);
                                self.port_connected = true;
                                println!("Opened the Serial Port!");
                            } else {
                                println!("Can't open port");
                            }
                        }
                    }
                }
                if ui.button("Settings").clicked() {
                    self.serial_settings_flag = !self.serial_settings_flag;
                }
            });
            ui.separator();
            match self.serial_port.as_mut() {
                Some(serial_port) => {
                    terminal(ui, &mut self.console_text, serial_port);
                }
                None => (),
            }
            ui.separator();
        });
        serial_settings_window(
            ctx,
            &mut self.selected_comport,
            &self.comports,
            &self.buadrates,
            &mut self.port_settings,
            &mut self.serial_settings_flag,
        );
        ctx.request_repaint();
    }
}
