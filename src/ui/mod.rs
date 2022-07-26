use eframe::{CreationContext, egui, Frame};
use egui::{Context, Ui};
use crate::cpu::Cpu;

pub struct RustyNesUi {
    cpu: Cpu,
    memory_start_address: String,
    memory_end_address: String,
}

impl RustyNesUi {
    pub fn new(cc: &CreationContext) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        let mut cpu = Cpu::new();
        cpu.load_rom(&[0x20, 0x09, 0x80, 0x20, 0x0c, 0x80, 0x20, 0x12, 0x80, 0xa2, 0x00, 0x60, 0xe8, 0xe0, 0x05, 0xd0, 0xfb, 0x60, 0x00]).unwrap();
        cpu.reset();
        RustyNesUi {
            cpu,
            memory_start_address: "0000".to_string(),
            memory_end_address: "0100".to_string(),
        }
    }
}

impl eframe::App for RustyNesUi {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Options");
            ui.horizontal(|ui| {
                ui.label("abc");
            });
        });

        egui::Window::new("Registers").resizable(false).show(ctx, |ui| {
            egui::Grid::new("register_grid")
                .striped(true)
                .show(ui, |ui| {
                    self.draw_register_table(ui);
                });
            egui::Grid::new("flag_grid")
                .striped(false)
                .min_col_width(10.0)
                .max_col_width(10.0)
                .show(ui, |ui| {
                    self.draw_flag_table(ui);
                });
        });

        egui::Window::new("Controls").resizable(false).show(ctx, |ui| {
            if ui.button("Step").clicked() {
                self.cpu.step();
            }
        });

        egui::Window::new("Memory")
            .resizable(true)
            .vscroll(true)
            .show(ctx, |ui| {
            let old_start_address = self.memory_start_address.clone();
            let old_end_address = self.memory_end_address.clone();
            ui.horizontal(|ui| {
                ui.label("Start Address");
                ui.text_edit_singleline(&mut self.memory_start_address);
            });
            ui.horizontal(|ui| {
                ui.label("End Address");
                ui.text_edit_singleline(&mut self.memory_end_address);
            });
            let (start, end) = self.validate_memory_addresses(old_start_address, old_end_address);


            egui::Grid::new("memory_grid")
                .striped(true)
                .min_col_width(10.0)
                .show(ui, |ui| {

                    self.draw_memory_table(ui, start, end);
                });
        });

        egui::Window::new("Disassembly")
            .resizable(true)
            .vscroll(true)
            .show(ctx, |ui| {
                let def = "???".to_string();
                let (disassembly, increment) = match self.cpu.memory.read(self.cpu.register_pc) {
                    Ok(opcode) => {
                        let disass = self.cpu.disassemble(opcode).unwrap_or((def, 1));
                        disass
                    }
                    _ => (def, 1),
                };
                ui.label(&disassembly);
                ui.label(&format!("Increment: {}", increment));
            });
    }
}

impl RustyNesUi {
    fn draw_register_table(&self, ui: &mut Ui) {
        ui.label("PC");
        ui.label(format!("0x{:04X}", self.cpu.register_pc));
        ui.end_row();
        ui.label("SP");
        ui.label(format!("0x{:02X}", self.cpu.register_sp));
        ui.end_row();
        ui.label("A");
        ui.label(format!("0x{:02X}", self.cpu.register_a));
        ui.end_row();
        ui.label("X");
        ui.label(format!("0x{:02X}", self.cpu.register_x));
        ui.end_row();
        ui.label("Y");
        ui.label(format!("0x{:02X}", self.cpu.register_y));
        ui.end_row();
        ui.label("P");
        ui.label(format!("0x{:02X}", self.cpu.status_flags.status));
        ui.end_row();

    }

    fn draw_flag_table(&self, ui: &mut Ui) {
        ui.label("N");
        ui.label("V");
        ui.label(" ");
        ui.label("B");
        ui.label("D");
        ui.label("I");
        ui.label("Z");
        ui.label("C");
        ui.end_row();
        ui.label(if self.cpu.status_flags.get_negative() { "1" } else { "0" });
        ui.label(if self.cpu.status_flags.get_overflow() { "1" } else { "0" });
        ui.label(if self.cpu.status_flags.get_break_2() { "1" } else { "0" });
        ui.label(if self.cpu.status_flags.get_break() { "1" } else { "0" });
        ui.label(if self.cpu.status_flags.get_decimal() { "1" } else { "0" });
        ui.label(if self.cpu.status_flags.get_interrupt() { "1" } else { "0" });
        ui.label(if self.cpu.status_flags.get_zero() { "1" } else { "0" });
        ui.label(if self.cpu.status_flags.get_carry() { "1" } else { "0" });
    }

    fn validate_memory_addresses(&mut self, old_start: String, old_end: String) -> (u16, u16) {
        let start = u16::from_str_radix(&self.memory_start_address, 16).unwrap_or_else(|_| {
            self.memory_start_address = old_start;
            u16::from_str_radix(&self.memory_start_address, 16).unwrap()
        });
        let end = u16::from_str_radix(&self.memory_end_address, 16).unwrap_or_else(|_| {
            self.memory_end_address = old_end;
            u16::from_str_radix(&self.memory_end_address, 16).unwrap()
        });
        (start, end)
    }


    fn draw_memory_table(&self, ui: &mut Ui, start: u16, end: u16) {
        let draw_start = start / 16 * 16;
        let draw_end = if end > start + 0x500 {
            start + 0x500
        } else {
            end
        };
        for address in draw_start..draw_end {
            if address % 16 == 0 {
                ui.label(format!("{:04X}", address));
            }
            if address >= start {
                match self.cpu.memory.read(address) {
                    Ok(value) => ui.label(format!("{:02X}", value)),
                    Err(_) => ui.label("--"),
                };
            } else {
                ui.label("  ");
            }
            if address % 16 == 15 {
                ui.end_row();
            }
        }
        if draw_end != end {
            ui.label("...");
            ui.end_row();
        }
    }
}