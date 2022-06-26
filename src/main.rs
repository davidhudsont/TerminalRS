mod gui;
mod xmodem;

use eframe::{
    egui::{self, Event, Key},
    emath::Align,
    epaint::vec2,
};
use gui::*;
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};
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

struct SerialPortSettings {
    /// The baud rate in symbols-per-second
    baud_rate: u32,
    /// Number of bits used to represent a character sent on the line
    data_bits: DataBits,
    /// The type of signalling to use for controlling data transfer
    flow_control: FlowControl,
    /// The type of parity to use for error checking
    parity: Parity,
    /// Number of bits to use to signal the end of a character
    stop_bits: StopBits,
    /// Amount of time to wait to receive data before timing out
    timeout: u64,
}

impl Default for SerialPortSettings {
    fn default() -> Self {
        Self {
            baud_rate: 115200,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: 10,
        }
    }
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

fn read_byte(port: &mut Box<dyn SerialPort>) -> String {
    let mut string: Vec<u8> = vec![];
    let mut read_buffer: Vec<u8> = vec![0; 1];
    loop {
        match port.read(&mut read_buffer[..]) {
            Err(_) => break,
            Ok(_) => {
                let byte = read_buffer[0];
                string.push(byte);
            }
        }
    }
    std::str::from_utf8(&string).unwrap().to_string()
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

impl Terminal {
    fn serial_settings_window(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new("Serial Settings")
            .open(open)
            .default_size(vec2(200.0, 200.0))
            .collapsible(true)
            .show(ctx, |ui| {
                ui.group(|ui| {
                    ui.label("Serial Parameters");
                    comport_setting_combo_box(ui, &mut self.selected_comport, &self.comports);
                    buadrate_setting_combo_box(
                        ui,
                        &mut self.port_settings.baud_rate,
                        &self.buadrates,
                    );
                    databits_setting_combo_box(ui, &mut self.port_settings.data_bits);
                    flowcontrol_setting_combo_box(ui, &mut self.port_settings.flow_control);
                    parity_setting_combo_box(ui, &mut self.port_settings.parity);
                    stopbits_setting_combo_box(ui, &mut self.port_settings.stop_bits);
                    timeout_setting_text_integer(ui, &mut self.port_settings.timeout);
                });
                // This line allows for freely resizable windows
                ui.allocate_space(ui.available_size());
            });
    }
}

impl eframe::App for Terminal {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("Menu").show(ctx, |ui| {
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
            egui::ScrollArea::vertical().show(ui, |ui| {
                if selectable_text(ui, &mut self.console_text).has_focus() {
                    let events = ui.input().events.clone(); // avoid dead-lock by cloning. TODO: optimize
                    for event in &events {
                        match event {
                            Event::Text(text) => {
                                // Newlines are handled by `Key::Enter`.
                                if !text.is_empty() && text != "\n" && text != "\r" {
                                    match self.serial_port.as_mut() {
                                        Some(port) => {
                                            port.write(text.as_bytes()).unwrap();
                                        }
                                        None => (),
                                    }
                                }
                            }
                            Event::Key {
                                key: Key::Enter,
                                pressed: true,
                                ..
                            } => match self.serial_port.as_mut() {
                                Some(port) => {
                                    port.write("\r\n".as_bytes()).unwrap();
                                }
                                None => (),
                            },
                            _ => (),
                        };
                    }
                    match self.serial_port.as_mut() {
                        Some(port) => {
                            let result = read_byte(port);
                            if result.len() > 0 {
                                self.console_text.push_str(&result);
                                ui.scroll_to_cursor(Some(Align::BOTTOM));
                            }
                        }
                        None => (),
                    }
                }
            });
            ui.separator();
        });
        let mut open = self.serial_settings_flag;
        self.serial_settings_window(ctx, &mut open);
        self.serial_settings_flag = open;
        ctx.request_repaint();
    }
}
