mod gui;
mod xmodem;

use eframe::egui;
use gui::*;
use serialport::SerialPort;
use std::fs::File;
use xmodem::XModem;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "TerminalRS",
        options,
        Box::new(|cc| Box::new(Terminal::new(cc))),
    );
}

fn xmodem_send(picked_path: String, port: &mut Box<dyn SerialPort>) {
    let stream = File::open(picked_path).unwrap();
    match XModem::new().send(port, Box::new(stream)) {
        Ok(()) => println!("File Send success"),
        Err(err) => println!("Error: {err}"),
    }
}

fn xmodem_recieve(picked_path: String, port: &mut Box<dyn SerialPort>) {
    let stream = File::create(picked_path).unwrap();
    match XModem::new().receive(port, Box::new(stream), false) {
        Ok(bytes) => {
            println!("File Receive success, Bytes: {bytes} read.")
        }
        Err(err) => println!("Error: {err}"),
    }
}

struct Terminal {
    selected_comport: String,
    serial_settings_flag: bool,
    sessions: Vec<Session>,
    selected_session: usize,
    edit_settings_flag: bool,
}

impl Terminal {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self {
            selected_comport: "".to_owned(),
            serial_settings_flag: false,
            sessions: vec![],
            selected_session: 0,
            edit_settings_flag: false,
        }
    }
}

impl eframe::App for Terminal {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("Menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Transfer", |ui| {
                    if ui.button("xModem Send").clicked() {
                        ui.close_menu();
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            if !self.sessions.is_empty() {
                                match self.sessions[self.selected_session].port.as_mut() {
                                    Some(port) => {
                                        let picked_path = path.display().to_string();
                                        xmodem_send(picked_path, port);
                                    }
                                    None => println!("No connected serial port!"),
                                }
                            } else {
                                println!("No connected serial port!")
                            }
                        }
                    }
                    if ui.button("xModem Receive").clicked() {
                        ui.close_menu();
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            if !self.sessions.is_empty() {
                                match self.sessions[self.selected_session].port.as_mut() {
                                    Some(port) => {
                                        let picked_path = path.display().to_string();
                                        xmodem_recieve(picked_path, port);
                                    }
                                    None => println!("No connected serial port!"),
                                }
                            } else {
                                println!("No connected serial port!")
                            }
                        }
                    }
                });
                ui.menu_button("Sessions", |ui| {
                    if ui.button("New session").clicked() {
                        self.serial_settings_flag = true;
                        ui.close_menu();
                    }
                    if !self.sessions.is_empty() && ui.button("Edit Session").clicked() {
                        self.edit_settings_flag = true;
                        ui.close_menu();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut count = 0;
                self.sessions = std::mem::take(&mut self.sessions)
                    .into_iter()
                    .filter_map(|session| {
                        let checked = count == self.selected_session;
                        count += 1;
                        match tab(ui, &session.name, checked) {
                            Action::Select => {
                                self.selected_session = count - 1;
                                Some(session)
                            }
                            Action::Delete => {
                                self.selected_session = 0;
                                None
                            }
                            Action::None => Some(session),
                        }
                    })
                    .collect();
            });
            if !self.sessions.is_empty() {
                ui.separator();
                terminal(ui, &mut self.sessions[self.selected_session]);
            }
        });
        if self.serial_settings_flag {
            match new_session_window(
                ctx,
                &mut self.selected_comport,
                &mut self.serial_settings_flag,
            ) {
                Some(session) => {
                    self.sessions.push(session);
                }
                None => (),
            }
        }
        if self.edit_settings_flag {
            edit_session_setting(
                ctx,
                &mut self.edit_settings_flag,
                &mut self.selected_comport,
                &mut self.sessions[self.selected_session],
            );
        }
        ctx.request_repaint();
    }
}
