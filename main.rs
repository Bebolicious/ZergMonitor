use eframe::egui;
use std::time::Duration;
use sysinfo::System;

const MAX_SPACING: f32 = 10.0;
const MIN_SPACING: f32 = 5.0;

struct CpuApp {
    system: System,
    usage: f32,
    usages: Vec<f32>,
    total_mem_usage: u64,
    used_mem_usage: u64,
}

impl Default for CpuApp {
    fn default() -> Self {
        let mut system = System::new_all();
        system.refresh_cpu();
        let usage = system.global_cpu_info().cpu_usage();
        let usages = system.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
        let total_mem_usage = system.total_memory();
        let used_mem_usage = system.used_memory();

        CpuApp {
            system,
            usage,
            usages,
            total_mem_usage,
            used_mem_usage,
        }
    }
}

impl eframe::App for CpuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.system.refresh_cpu();
        self.usage = self.system.global_cpu_info().cpu_usage();
        self.usages = self
            .system
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .collect();
        self.system.refresh_memory();
        let tm = self.total_mem_usage / 1024 / 1024;
        let um = self.used_mem_usage / 1024 / 1024;

        egui::TopBottomPanel::top("topd_panel")
            .frame(egui::Frame::default().fill(egui::Color32::from_rgb(30, 30, 30))) 
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.vertical_centered(|ui| {
                    ui.heading(
                        egui::RichText::new("Zerg Monitor")
                            .color(egui::Color32::from_rgb(147, 112, 219)),
                    );
                });
                ui.add_space(10.0);

                ui.label(
                    egui::RichText::new(format!("System name: {}", System::name().unwrap()))
                        .color(egui::Color32::from_rgb(200, 200, 200)), 
                );
                ui.label(
                    egui::RichText::new(format!(
                        "System kernel version: {}",
                        System::kernel_version().unwrap()
                    ))
                    .color(egui::Color32::from_rgb(200, 200, 200)), 
                );
                ui.label(
                    egui::RichText::new(format!(
                        "System host name: {}",
                        System::host_name().unwrap()
                    ))
                    .color(egui::Color32::from_rgb(200, 200, 200)), 
                );
                ui.label(
                    egui::RichText::new(format!(
                        "System OS version: {}",
                        System::os_version().unwrap()
                    ))
                    .color(egui::Color32::from_rgb(200, 200, 200)), 
                );
                ui.add_space(10.0);
            });

        let overall_cpu_usage: f32 = self
            .system
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .sum::<f32>()
            / self.system.cpus().len() as f32;

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(egui::Color32::from_rgb(40, 40, 40))) 
            .show(ctx, |ui| {
                ui.add_space(MIN_SPACING);

                ui.horizontal(|ui| {
                    ui.add_space(MIN_SPACING); 
                    ui.heading(
                        egui::RichText::new("CPU Usage")
                            .color(egui::Color32::from_rgb(255, 165, 0)),
                    );
                });

                ui.horizontal(|ui| {
                    ui.add_space(MAX_SPACING);
                    ui.label(format!("Overall CPU Usage: {:.2}%", overall_cpu_usage));
                });
                ui.add_space(MAX_SPACING);
                ui.horizontal(|ui| {
                    ui.add_space(MAX_SPACING);
                    ui.add(
                        egui::ProgressBar::new(overall_cpu_usage / 100.0)
                            .desired_width(200.0) 
                            .show_percentage(),
                    );
                });
                ui.add_space(MAX_SPACING);

                ui.separator();
                ui.horizontal(|ui| {
                    ui.add_space(MIN_SPACING); 
                    ui.heading(
                        egui::RichText::new("Memory Usage")
                            .color(egui::Color32::from_rgb(255, 165, 0)),
                    );
                });
                ui.horizontal(|ui| {
                    ui.add_space(MAX_SPACING); 
                    ui.label(format!("Total Memory Usage: {} MB", tm));
                });
                ui.horizontal(|ui| {
                    ui.add_space(MAX_SPACING); 
                ui.label(format!("Used Memory Usage: {} MB", um));
                });
               
            });

        ctx.request_repaint_after(Duration::from_secs_f32(1.0));
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Zerg Monitor",
        options,
        Box::new(|_cc| Box::new(CpuApp::default())),
    )
}
