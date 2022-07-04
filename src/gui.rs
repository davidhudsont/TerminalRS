use std::time::Duration;

use eframe::{
    egui::{self, epaint::vec2, Event, Key, Response, Ui, WidgetText},
    emath::Align,
};
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};

pub struct SerialPortSettings {
    /// The baud rate in symbols-per-second
    pub baud_rate: u32,
    /// Number of bits used to represent a character sent on the line
    pub data_bits: DataBits,
    /// The type of signalling to use for controlling data transfer
    pub flow_control: FlowControl,
    /// The type of parity to use for error checking
    pub parity: Parity,
    /// Number of bits to use to signal the end of a character
    pub stop_bits: StopBits,
    /// Amount of time to wait to receive data before timing out
    pub timeout: u64,
}

pub struct Session {
    pub name: String,
    pub buffer: String,
    pub settings: SerialPortSettings,
    pub port: Option<Box<dyn SerialPort>>,
}

impl Session {
    fn new(name: String, settings: SerialPortSettings, port: Option<Box<dyn SerialPort>>) -> Self {
        Self {
            name: name,
            buffer: String::default(),
            settings: settings,
            port: port,
        }
    }
}

impl Default for Session {
    fn default() -> Self {
        Self {
            name: Default::default(),
            buffer: String::default(),
            settings: SerialPortSettings::default(),
            port: None,
        }
    }
}

pub fn read_byte(port: &mut Box<dyn SerialPort>) -> String {
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

pub fn selectable_text(ui: &mut egui::Ui, mut text: &str) -> Response {
    ui.add(
        egui::TextEdit::multiline(&mut text)
            .desired_width(1000.0)
            .desired_rows(25),
    )
}

pub fn comport_setting_combo_box(
    ui: &mut Ui,
    selected_comport: &mut String,
    comports: &Vec<String>,
) {
    ui.horizontal(|ui| {
        ui.label("COMM:");
        egui::ComboBox::from_id_source("COMPORT")
            .selected_text(format!("{}", selected_comport))
            .show_ui(ui, |ui| {
                for comport in comports {
                    ui.selectable_value(selected_comport, comport.to_string(), comport);
                }
            });
    });
}

pub fn buadrate_setting_combo_box(ui: &mut Ui, baud_rate: &mut u32) {
    let baud_rates: Vec<u32> = vec![
        110, 300, 600, 1200, 2400, 4800, 9600, 14400, 19200, 38400, 57600, 115200, 230400, 460800,
        921600,
    ];

    ui.horizontal(|ui| {
        ui.label("BAUD:");
        egui::ComboBox::from_id_source("BAUD")
            .selected_text(format!("{}", baud_rate))
            .show_ui(ui, |ui| {
                for rate in baud_rates {
                    ui.selectable_value(baud_rate, rate, rate.to_string());
                }
            });
    });
}

pub fn databits_setting_combo_box(ui: &mut Ui, data_bits: &mut DataBits) {
    ui.horizontal(|ui| {
        ui.label("DataBits:");
        egui::ComboBox::from_id_source("DataBit")
            .selected_text(format!("{:?}", data_bits))
            .show_ui(ui, |ui| {
                ui.selectable_value(data_bits, DataBits::Five, "Five");
                ui.selectable_value(data_bits, DataBits::Six, "Six");
                ui.selectable_value(data_bits, DataBits::Seven, "Seven");
                ui.selectable_value(data_bits, DataBits::Eight, "Eight");
            });
    });
}

pub fn flowcontrol_setting_combo_box(ui: &mut Ui, flow_control: &mut FlowControl) {
    ui.horizontal(|ui| {
        ui.label("Flow Control:");
        egui::ComboBox::from_id_source("FlowControl")
            .selected_text(format!("{:?}", flow_control))
            .show_ui(ui, |ui| {
                ui.selectable_value(flow_control, FlowControl::None, "None");
                ui.selectable_value(flow_control, FlowControl::Software, "Software");
                ui.selectable_value(flow_control, FlowControl::Hardware, "Hardware");
            })
    });
}

pub fn parity_setting_combo_box(ui: &mut Ui, parity: &mut Parity) {
    ui.horizontal(|ui| {
        ui.label("Parity:");
        egui::ComboBox::from_id_source("Parity")
            .selected_text(format!("{:?}", parity))
            .show_ui(ui, |ui| {
                ui.selectable_value(parity, Parity::None, "None");
                ui.selectable_value(parity, Parity::Odd, "Odd");
                ui.selectable_value(parity, Parity::Even, "Even");
            })
    });
}

pub fn stopbits_setting_combo_box(ui: &mut Ui, stop_bits: &mut StopBits) {
    ui.horizontal(|ui| {
        ui.label("Stop Bits:");
        egui::ComboBox::from_id_source("StopBits")
            .selected_text(format!("{:?}", stop_bits))
            .show_ui(ui, |ui| {
                ui.selectable_value(stop_bits, StopBits::One, "One");
                ui.selectable_value(stop_bits, StopBits::Two, "Two");
            })
    });
}

pub fn timeout_setting_text_integer(ui: &mut Ui, timeout: &mut u64) {
    ui.horizontal(|ui| {
        ui.label("Timeout:");
        ui.add(egui::DragValue::new(timeout));
    });
}

pub enum Action {
    Select,
    Delete,
    None,
}

pub fn tab(ui: &mut Ui, name: impl Into<WidgetText>, checked: bool) -> Action {
    let mut action = Action::None;
    ui.group(|ui| {
        if ui.selectable_label(checked, name).clicked() {
            action = Action::Select;
        } else if ui.button("x").clicked() {
            action = Action::Delete;
        }
    });
    action
}

pub fn serial_settings(
    ui: &mut Ui,
    selected_comport: &mut String,
    port_settings: &mut SerialPortSettings,
) {
    ui.group(|ui| {
        ui.label("Serial Parameters");
        let serial_ports: Vec<String> = serialport::available_ports()
            .unwrap()
            .iter()
            .map(|port| port.port_name.clone())
            .collect();
        comport_setting_combo_box(ui, selected_comport, &serial_ports);
        buadrate_setting_combo_box(ui, &mut port_settings.baud_rate);
        databits_setting_combo_box(ui, &mut port_settings.data_bits);
        flowcontrol_setting_combo_box(ui, &mut port_settings.flow_control);
        parity_setting_combo_box(ui, &mut port_settings.parity);
        stopbits_setting_combo_box(ui, &mut port_settings.stop_bits);
        timeout_setting_text_integer(ui, &mut port_settings.timeout);
    });
}

pub fn create_serial_port_button(
    ui: &mut Ui,
    selected_comport: &mut String,
    port_settings: &mut SerialPortSettings,
) -> Option<Box<dyn SerialPort>> {
    if ui.button("Connect").clicked() {
        if selected_comport.len() > 0 {
            if let Ok(port) = serialport::new(selected_comport.clone(), port_settings.baud_rate)
                .data_bits(port_settings.data_bits)
                .flow_control(port_settings.flow_control)
                .parity(port_settings.parity)
                .stop_bits(port_settings.stop_bits)
                .timeout(Duration::from_millis(port_settings.timeout))
                .open()
            {
                println!("Opened the Serial Port!");
                return Some(port);
            } else {
                println!("Can't open port");
                return None;
            }
        }
    }
    None
}

pub fn new_session_window(
    ctx: &egui::Context,
    selected_comport: &mut String,
    open: &mut bool,
) -> Option<Session> {
    let mut result = None;
    let mut port_settings = SerialPortSettings::default();
    let mut serial_port: Option<Box<dyn SerialPort>> = None;
    egui::Window::new("New Session")
        .open(open)
        .default_size(vec2(200.0, 200.0))
        .show(ctx, |ui| {
            serial_settings(ui, selected_comport, &mut port_settings);
            ui.add_space(20.0);
            ui.vertical_centered(|ui| {
                match create_serial_port_button(ui, selected_comport, &mut port_settings) {
                    Some(port) => serial_port = Some(port),
                    None => (),
                }
            });
            ui.add_space(10.0);
            // This line allows for freely resizable windows
            // ui.allocate_space(ui.available_size());
        });

    match serial_port {
        Some(port) => {
            *open = false;
            result = Some(Session::new(
                selected_comport.clone(),
                port_settings,
                Some(port),
            ));
        }
        None => (),
    }
    result
}

pub fn edit_session_setting(
    ctx: &egui::Context,
    open: &mut bool,
    selected_comport: &mut String,
    session: &mut Session,
) {
    let mut close = false;
    egui::Window::new("Edit Settings")
        .open(open)
        .default_size(vec2(200.0, 200.0))
        .show(ctx, |ui| {
            serial_settings(ui, selected_comport, &mut session.settings);
            ui.add_space(20.0);
            ui.vertical_centered(|ui| {
                session.port = None;
                match create_serial_port_button(ui, selected_comport, &mut session.settings) {
                    Some(port) => {
                        session.port = Some(port);
                        close = true;
                    }
                    None => (),
                }
            });
            ui.add_space(10.0);
            // This line allows for freely resizable windows
            // ui.allocate_space(ui.available_size());
        });
    if close {
        *open = false;
    }
}

pub fn terminal(ui: &mut Ui, session: &mut Session) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        if selectable_text(ui, &session.buffer).has_focus() {
            let events = ui.input().events.clone(); // avoid dead-lock by cloning. TODO: optimize
            for event in &events {
                match event {
                    Event::Text(text) => {
                        // Newlines are handled by `Key::Enter`.
                        if !text.is_empty() && text != "\n" && text != "\r" {
                            match session.port.as_mut() {
                                Some(serial_port) => {
                                    serial_port.write(text.as_bytes()).unwrap();
                                    ui.scroll_to_cursor(Some(Align::BOTTOM));
                                }
                                None => (),
                            }
                        }
                    }
                    Event::Key {
                        key: Key::Enter,
                        pressed: true,
                        ..
                    } => match session.port.as_mut() {
                        Some(serial_port) => {
                            serial_port.write("\r\n".as_bytes()).unwrap();
                            ui.scroll_to_cursor(Some(Align::BOTTOM));
                        }
                        None => (),
                    },
                    Event::Scroll(_) => (),
                    _ => ui.scroll_to_cursor(Some(Align::BOTTOM)),
                };
            }

            match session.port.as_mut() {
                Some(serial_port) => {
                    let result = read_byte(serial_port);
                    if result.len() > 0 {
                        session.buffer.push_str(&result);
                        ui.scroll_to_cursor(Some(Align::BOTTOM));
                    }
                }
                None => (),
            }
        }
    });
}
