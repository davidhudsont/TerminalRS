mod xmodem;

use eframe::{
    egui::{self, Event, Key, Response},
    emath::Align,
};
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};
use std::fs::File;
use std::time::Duration;
use xmodem::XModem;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(Terminal::default()), options);
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
    timeout: Duration,
}

impl SerialPortSettings {
    fn default() -> Self {
        Self {
            baud_rate: 115200,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_millis(100),
        }
    }
}

struct Terminal {
    name: String,
    selected_comport: String,
    comports: Vec<String>,
    buadrates: Vec<u32>,
    console_text: String,
    serial_settings_flag: bool,
    serial_port: Option<Box<dyn SerialPort>>,
    port_connected: bool,
    port_settings: SerialPortSettings,
}

impl Default for Terminal {
    fn default() -> Self {
        Self {
            name: "651R2/A Firmware Upgrade Application".to_owned(),
            selected_comport: "".to_owned(),
            comports: vec![],
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

fn selectable_text(ui: &mut egui::Ui, mut text: &str) -> Response {
    ui.add(
        egui::TextEdit::multiline(&mut text)
            .desired_width(1000.0)
            .desired_rows(25),
    )
}

impl Terminal {
    fn serial_settings_window(&mut self, ctx: &egui::Context) {
        let window = egui::Window::new("Serial Settings")
            .open(&mut self.serial_settings_flag)
            .resizable(true)
            .min_width(800.0)
            .collapsible(true)
            .enabled(true);

        window.show(ctx, |ui| {
            ui.group(|ui| {
                ui.label("Serial Parameters");
                ui.horizontal(|ui| {
                    ui.label("Data:");
                    egui::ComboBox::from_id_source("DataBit")
                        .selected_text(format!("{:?}", self.port_settings.data_bits))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.port_settings.data_bits,
                                DataBits::Five,
                                "Five",
                            );
                            ui.selectable_value(
                                &mut self.port_settings.data_bits,
                                DataBits::Six,
                                "Six",
                            );
                            ui.selectable_value(
                                &mut self.port_settings.data_bits,
                                DataBits::Seven,
                                "Seven",
                            );
                            ui.selectable_value(
                                &mut self.port_settings.data_bits,
                                DataBits::Eight,
                                "Eight",
                            );
                        });
                });
                ui.horizontal(|ui| {
                    ui.label("Flow Control:");
                    egui::ComboBox::from_id_source("FlowControl")
                        .selected_text(format!("{:?}", self.port_settings.flow_control))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.port_settings.flow_control,
                                FlowControl::None,
                                "None",
                            );
                            ui.selectable_value(
                                &mut self.port_settings.flow_control,
                                FlowControl::Software,
                                "Software",
                            );
                            ui.selectable_value(
                                &mut self.port_settings.flow_control,
                                FlowControl::Hardware,
                                "Hardware",
                            );
                        })
                });
                ui.horizontal(|ui| {
                    ui.label("Parity:");
                    egui::ComboBox::from_id_source("Parity")
                        .selected_text(format!("{:?}", self.port_settings.parity))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.port_settings.parity,
                                Parity::None,
                                "None",
                            );
                            ui.selectable_value(&mut self.port_settings.parity, Parity::Odd, "Odd");
                            ui.selectable_value(
                                &mut self.port_settings.parity,
                                Parity::Even,
                                "Even",
                            );
                        })
                });

                ui.horizontal(|ui| {
                    ui.label("Stop Bits:");
                    egui::ComboBox::from_id_source("StopBits")
                        .selected_text(format!("{:?}", self.port_settings.stop_bits))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.port_settings.stop_bits,
                                StopBits::One,
                                "One",
                            );
                            ui.selectable_value(
                                &mut self.port_settings.stop_bits,
                                StopBits::Two,
                                "Two",
                            );
                        })
                });
            });
        });
    }
}

impl eframe::epi::App for Terminal {
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &eframe::epi::Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        let serial_ports = serialport::available_ports().unwrap();
        self.comports = serial_ports
            .iter()
            .map(|port| port.port_name.clone())
            .collect();
        println!("Serial Ports {:?}", self.comports);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &eframe::epi::Frame) {
        if self.serial_settings_flag {
            self.serial_settings_window(ctx);
        }

        egui::TopBottomPanel::top("Menu").show(ctx, |ui| {
            ui.menu_button("Transfer", |ui| {
                if ui.button("xModem Send").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        let picked_path = Some(path.display().to_string());
                        println!("Picked path: {:?}", picked_path);

                        let mut xmodem = XModem::new();
                        let stream = File::open(picked_path.unwrap()).unwrap();
                        let port = self.serial_port.as_mut().unwrap();

                        match xmodem.send(port, Box::new(stream)) {
                            Ok(()) => println!("File Send success"),
                            Err(err) => println!("Error: {err}"),
                        }
                    }
                }
                if ui.button("xModem Receive").clicked() {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        let picked_path = Some(path.display().to_string());
                        println!("Picked path: {:?}", picked_path);

                        let mut xmodem = XModem::new();
                        let stream = File::create(picked_path.unwrap()).unwrap();
                        let port = self.serial_port.as_mut().unwrap();

                        match xmodem.receive(port, Box::new(stream), false) {
                            Ok(bytes) => println!("File Receive success, Bytes: {bytes} read."),
                            Err(err) => println!("Error: {err}"),
                        }
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("COMM:");
                egui::ComboBox::from_id_source("COMPORT")
                    .selected_text(format!("{}", self.selected_comport))
                    .show_ui(ui, |ui| {
                        for comport in &self.comports {
                            ui.selectable_value(
                                &mut self.selected_comport,
                                comport.to_string(),
                                comport,
                            );
                        }
                    });
                if ui.button("Refresh Ports").clicked() {
                    let serial_ports = serialport::available_ports().unwrap();
                    self.comports = serial_ports
                        .iter()
                        .map(|port| port.port_name.clone())
                        .collect();
                    println!("Serial Ports {:?}", self.comports);
                }
                ui.label("BAUD:");
                egui::ComboBox::from_id_source("BAUD")
                    .selected_text(format!("{}", self.port_settings.baud_rate))
                    .show_ui(ui, |ui| {
                        for baudrate in &self.buadrates {
                            ui.selectable_value(
                                &mut self.port_settings.baud_rate,
                                *baudrate,
                                baudrate.to_string(),
                            );
                        }
                    });
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
                            .timeout(self.port_settings.timeout)
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
                            Event::Text(text_to_insert) => {
                                // Newlines are handled by `Key::Enter`.
                                if !text_to_insert.is_empty()
                                    && text_to_insert != "\n"
                                    && text_to_insert != "\r"
                                {
                                    match self.serial_port.as_mut() {
                                        Some(port) => {
                                            port.write(text_to_insert.as_bytes()).unwrap();
                                        }
                                        None => (),
                                    }
                                    ui.scroll_to_cursor(Some(Align::BOTTOM));
                                }
                            }
                            Event::Key {
                                key: Key::Enter,
                                pressed: true,
                                ..
                            } => {
                                match self.serial_port.as_mut() {
                                    Some(port) => {
                                        port.write("\r\n".as_bytes()).unwrap();
                                    }
                                    None => (),
                                }
                                ui.scroll_to_cursor(Some(Align::BOTTOM));
                            }
                            _ => (),
                        };
                    }
                    let mut read_buffer: Vec<u8> = vec![0; 1];
                    match self.serial_port.as_mut() {
                        Some(port) => match port.read(&mut read_buffer[..]) {
                            Err(_) => (),
                            Ok(bytes) => {
                                println!("Bytes read: {bytes}");
                                if bytes > 0 {
                                    self.console_text
                                        .push_str(std::str::from_utf8(&read_buffer).unwrap());
                                }
                            }
                        },
                        None => (),
                    }
                }
            });
            ui.separator();
        });
        ctx.request_repaint();
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}
