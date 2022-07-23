mod gui;
mod xmodem;

use eframe::egui;
use gui::*;
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

struct Terminal {
    selected_comport: String,
    selected_setting: SerialPortSettings,
    serial_settings_flag: bool,
    sessions: Vec<Session>,
    selected_session: usize,
    edit_settings_flag: bool,
    popup_manager: PopUpManager,
}

impl Terminal {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self {
            selected_comport: "".to_owned(),
            selected_setting: SerialPortSettings::default(),
            serial_settings_flag: false,
            sessions: vec![],
            selected_session: 0,
            edit_settings_flag: false,
            popup_manager: PopUpManager::default(),
        }
    }
}

#[derive(Default)]
struct PopUpManager {
    popups: Vec<(bool, String)>,
}

impl PopUpManager {
    fn add_popup(&mut self, title: String) {
        self.popups.push((true, title))
    }

    fn show(&mut self, ctx: &egui::Context) {
        for i in 0..self.popups.len() {
            let title = self.popups[i].1.clone();
            let open = &mut self.popups[i].0;
            let mut close = false;
            egui::Window::new("Error").open(open).show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(title);
                    if ui.button("Close").clicked() {
                        close = true;
                    }
                });
            });
            if close {
                *open = false;
            }
        }
    }
}

impl Terminal {
    fn add_popup(&mut self, title: String) {
        self.popup_manager.add_popup(title);
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
                                        let stream = File::open(picked_path).unwrap();
                                        match XModem::new().send(port, Box::new(stream)) {
                                            Ok(()) => {
                                                self.add_popup("File Send success".to_string())
                                            }
                                            Err(err) => self.add_popup(format!("Error: {}", err)),
                                        }
                                    }
                                    None => self.add_popup("No connected serial port!".to_string()),
                                }
                            } else {
                                self.add_popup("No connected serial port!".to_string());
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
                                        let stream = File::create(picked_path).unwrap();
                                        match XModem::new().receive(port, Box::new(stream), false) {
                                            Ok(bytes) => {
                                                self.add_popup(format!(
                                                    "File Receive success, Bytes: {} read.",
                                                    bytes
                                                ));
                                            }
                                            Err(err) => self.add_popup(format!("Error: {}", err)),
                                        }
                                    }
                                    None => self.add_popup("No connected serial port!".to_string()),
                                }
                            } else {
                                self.add_popup("No connected serial port!".to_string());
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
                &mut self.selected_setting,
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
        self.popup_manager.show(ctx);
        ctx.request_repaint();
    }
}
