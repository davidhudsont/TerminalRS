use eframe::egui::{self, epaint::vec2, Response, Ui};
use serialport::{DataBits, FlowControl, Parity, StopBits};

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

pub fn buadrate_setting_combo_box(ui: &mut Ui, baud_rate: &mut u32, baud_rates: &Vec<u32>) {
    ui.horizontal(|ui| {
        ui.label("BAUD:");
        egui::ComboBox::from_id_source("BAUD")
            .selected_text(format!("{}", baud_rate))
            .show_ui(ui, |ui| {
                for rate in baud_rates {
                    ui.selectable_value(baud_rate, *rate, rate.to_string());
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

pub fn serial_settings_window(
    ctx: &egui::Context,
    selected_comport: &mut String,
    comports: &Vec<String>,
    baud_rates: &Vec<u32>,
    port_settings: &mut SerialPortSettings,
    open: &mut bool,
) {
    egui::Window::new("Serial Settings")
        .open(open)
        .default_size(vec2(200.0, 200.0))
        .collapsible(true)
        .show(ctx, |ui| {
            ui.group(|ui| {
                ui.label("Serial Parameters");
                comport_setting_combo_box(ui, selected_comport, &comports);
                buadrate_setting_combo_box(ui, &mut port_settings.baud_rate, &baud_rates);
                databits_setting_combo_box(ui, &mut port_settings.data_bits);
                flowcontrol_setting_combo_box(ui, &mut port_settings.flow_control);
                parity_setting_combo_box(ui, &mut port_settings.parity);
                stopbits_setting_combo_box(ui, &mut port_settings.stop_bits);
                timeout_setting_text_integer(ui, &mut port_settings.timeout);
            });
            // This line allows for freely resizable windows
            ui.allocate_space(ui.available_size());
        });
}
