use eframe::egui;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{mpsc::channel, Arc, Mutex};
use std::thread;
use tokio::runtime::Runtime;

use crate::scanner::scan;

pub struct App {
    ip_input: String,
    start_port: u16,
    end_port: u16,
    results: Arc<Mutex<Vec<u16>>>,
    scanning: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            ip_input: "127.0.0.1".to_string(),
            start_port: 1,
            end_port: 1024,
            results: Arc::new(Mutex::new(vec![])),
            scanning: false,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Port Scanner");

            ui.horizontal(|ui| {
                ui.label("Target IP:");
                ui.text_edit_singleline(&mut self.ip_input);
                if let Ok(parsed_ip) = self.ip_input.parse::<IpAddr>() {
                    if matches!(parsed_ip, IpAddr::V6(_)) {
                        ui.colored_label(egui::Color32::RED, "IPv6 is not supported.");
                        //needs logic to disable scan btn!
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("Start Port:");
                ui.add(egui::DragValue::new(&mut self.start_port).clamp_range(1..=65535));
                ui.label("End Port:");
                ui.add(egui::DragValue::new(&mut self.end_port).clamp_range(1..=65535));
            });

            ui.horizontal(|ui| {
                if ui.button("Scan").clicked() && !self.scanning {
                    self.scanning = true;

                    let ip = self
                        .ip_input
                        .parse::<IpAddr>()
                        .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));
                    let start = self.start_port;
                    let end = self.end_port;
                    let results_arc = Arc::clone(&self.results);

                    thread::spawn(move || {
                        let rt = Runtime::new().unwrap();
                        let (tx, rx) = channel();

                        rt.block_on(async {
                            for port in start..=end {
                                let tx = tx.clone();
                                tokio::spawn(scan(tx, port, ip));
                            }
                        });

                        drop(tx);

                        for open_port in rx {
                            let mut results = results_arc.lock().unwrap();
                            results.push(open_port);
                        }
                    });
                }
                if ui.button("Refresh").clicked() {
                    if let Ok(mut results) = self.results.lock() {
                        results.clear();
                    }
                    self.scanning = false;
                }
            });

            ui.separator();

            if let Ok(mut results) = self.results.lock() {
                results.sort();
                if !results.is_empty() {
                    ui.label("Open Ports:");
                    for port in results.iter() {
                        ui.label(format!("Port {} is open", port));
                    }
                } else if self.scanning {
                    ui.label("Scanning...");
                }
            }
        });
    }
}
