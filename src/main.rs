use serialport::{DataBits, StopBits};

mod xmodem;

use xmodem::XModem;

use std::fs::File;

use std::time::Duration;

use eframe::egui::{self, Response, Event, Key};
use serialport::SerialPort;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        Box::new(MyApp::default()),
        options,
    );
}

// fn main() {
//     let builder = serialport::new("COM11", 115200)
//         .data_bits(DataBits::Eight)
//         .stop_bits(StopBits::One);

//     let mut port = builder.open().expect("Failed to open port");
//     port.set_timeout(Duration::new(1, 0)).unwrap();

//     let mut xmodem: XModem = XModem::new();

//     let stream = File::open("example.txt").unwrap();
//     xmodem.send(&mut port, Box::new(stream)).unwrap();

//     // let stream = File::create("example2.txt").unwrap();
//     // xmodem.receive(Box::new(stream), true).unwrap();

// }


#[derive(Debug, PartialEq)]
enum ComPort
{
    None,
    COMPORT(String),
}

#[derive(Debug, PartialEq)]
enum BuadRates {
    None,
    BAUD(i32)
}

struct MyApp {
    name: String,
    selected_comport: ComPort,
    comports: Vec<ComPort>,
    selected_buadrate: BuadRates,
    buadrates: Vec<BuadRates>,
    console_text: String,
    serial_settings_flag: bool,
    serial_port: Option<Box<dyn SerialPort>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "651R2/A Firmware Upgrade Application".to_owned(),
            selected_comport: ComPort::None,
            comports: vec![],
            selected_buadrate: BuadRates::None,
            buadrates: vec![BuadRates::BAUD(9600), BuadRates::BAUD(115200)],
            console_text: "".to_owned(),
            serial_settings_flag: false,
            serial_port: None,
        }
    }
}

fn selectable_text(ui: &mut egui::Ui, mut text: &str) -> Response {
    ui.add(egui::TextEdit::multiline(&mut text).desired_width(1000.0).desired_rows(25))
}

impl eframe::epi::App for MyApp {
    fn setup(&mut self, _ctx: &egui::Context, _frame: &eframe::epi::Frame, _storage: Option<&dyn eframe::epi::Storage>) {
        let serial_ports = serialport::available_ports().unwrap();
        self.comports = serial_ports.iter().map(|port| ComPort::COMPORT(port.port_name.clone())).collect();
        println!("Serial Ports {:?}", self.comports);
    }


    fn update(&mut self, ctx: &egui::Context, _frame: & eframe::epi::Frame) {
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
                .selected_text(format!("{:?}", self.selected_comport))
                .show_ui(ui, |ui| {
                        for comport in &self.comports {
                            match comport {
                                ComPort::COMPORT(port) => ui.selectable_value(&mut self.selected_comport, ComPort::COMPORT(port.to_string()), port.to_string()),
                                ComPort::None => ui.selectable_value(&mut self.selected_comport, ComPort::None, ""),
                            };
                        }
                    }
                );
                if ui.button("Refresh Ports").clicked() {
                    let serial_ports = serialport::available_ports().unwrap();
                    self.comports = serial_ports.iter().map(|port| ComPort::COMPORT(port.port_name.clone())).collect();
                    println!("Serial Ports {:?}", self.comports);

                }
                ui.label("BAUD:");
                egui::ComboBox::from_id_source("BAUD")
                .selected_text(format!("{:?}", self.selected_buadrate))
                .show_ui(ui, |ui| {
                        for baudrate in &self.buadrates {
                            match baudrate {
                                BuadRates::BAUD(rate) => ui.selectable_value(&mut self.selected_buadrate, BuadRates::BAUD(*rate), rate.to_string()),
                                BuadRates::None => ui.selectable_value(&mut self.selected_buadrate, BuadRates::None, ""),
                            };
                        }
                    }
                );
                if ui.button("Connect").clicked() {
                    match &self.selected_comport {
                        ComPort::None => println!("Select a valid Comport!!!!"),
                        ComPort::COMPORT(port_name) => {
                            if let Ok(port) = serialport::new(port_name, 115200).timeout(Duration::from_millis(100)).open() {
                                self.serial_port = Some(port);
                                println!("Opened the Serial Port!");
                            }
                            else {
                                println!("Can't open port");
                            }
                        },
                    }
                }
                if ui.button("Settings").clicked() {
                    self.serial_settings_flag = !self.serial_settings_flag;
                }
                if self.serial_settings_flag {
                    egui::Window::new("Serial Settings").collapsible(false).open(&mut self.serial_settings_flag).resizable(true).show(ctx, |ui| {
                        ui.label("Serial Parameters");
                        ui.horizontal(|ui| {
                            ui.label("Data:");
                            egui::ComboBox::from_id_source("BAUD")
                            .selected_text(format!("{:?}", self.selected_buadrate))
                            .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.selected_buadrate, BuadRates::None, "");
                                }
                            );
                        });
                    });
                }
            });
            if ui.button("Get ID").clicked() {
                let buf = "ID\n\r".as_bytes();
                match self.serial_port {
                    Some(_) => self.serial_port.as_mut().unwrap().write(buf).unwrap(),
                    None => 0,
                };

                let mut read_buffer: Vec<u8> = vec![0; 1000];
                match self.serial_port.as_mut().unwrap().read(&mut read_buffer[..]) {
                    Err(_) => println!("Got an IO Error"),
                    Ok(bytes) => println!("Bytes read: {bytes}"),
                }
                self.console_text = std::str::from_utf8(&read_buffer).unwrap().to_string();
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                if selectable_text(ui, &mut self.console_text).has_focus() {
                    let events = ui.input().events.clone(); // avoid dead-lock by cloning. TODO: optimize
                    for event in &events {
                        match event {
                            Event::Text(text_to_insert) => {
                                // Newlines are handled by `Key::Enter`.
                                if !text_to_insert.is_empty() && text_to_insert != "\n" && text_to_insert != "\r" {
                                    self.serial_port.as_mut().unwrap().write(text_to_insert.as_bytes()).unwrap();
                                    let mut read_buffer: Vec<u8> = vec![0; 1000];
                                    match self.serial_port.as_mut().unwrap().read(&mut read_buffer[..]) {
                                        Err(err) => println!("Got an IO Error: {err}"),
                                        Ok(bytes) => println!("Bytes read: {bytes}"),
                                    }
                                    self.console_text.push_str(std::str::from_utf8(&read_buffer).unwrap());
                                } else {
                                }
                            }
                            Event::Key {
                                key: Key::Enter,
                                pressed: true,
                                ..
                            } => {
                                self.serial_port.as_mut().unwrap().write("\r\n".as_bytes()).unwrap();
                                let mut read_buffer: Vec<u8> = vec![0; 1000];
                                match self.serial_port.as_mut().unwrap().read(&mut read_buffer[..]) {
                                    Err(err) => println!("Got an IO Error: {err}"),
                                    Ok(bytes) => println!("Bytes read: {bytes}"),
                                }
                                self.console_text.push_str(std::str::from_utf8(&read_buffer).unwrap());
                            }
                            _ => (),
                        };
                    }
                }
            });
        });
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}


