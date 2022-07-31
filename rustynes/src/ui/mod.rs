use std::sync::{Arc, mpsc};
use std::sync::mpsc::{Receiver, Sender};
use std::{fs, thread};
use std::fs::File;
use std::time::Duration;
use crate::cpu::Cpu;
use eframe::epaint::Rounding;
use eframe::{egui, CreationContext, Frame};
use eframe::epaint::mutex::RwLock;
use egui::{Color32, Context, Key, Rect, Sense, Vec2};
use crate::cpu::disassembly::Instruction;
use crate::memory::nes::NesBus;
use crate::rom::Rom;

pub struct RustyNesUi {
    cpu: Arc<RwLock<Cpu>>,
    stop_tx: Option<Sender<()>>,
    halted_rx: Option<Receiver<()>>,
    memory_start_address: String,
    memory_end_address: String,
    memory_write_address: String,
    memory_write_value: String,
    first_frame: bool,
}

impl RustyNesUi {
    pub fn new(cc: &CreationContext) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        let mut rom_bytes = fs::read("roms/nestest.nes").unwrap();
        rom_bytes[0x400c] = 0x00;
        let rom = Rom::new(&rom_bytes).unwrap();
        let bus = NesBus::new(rom);

        let mut cpu = Cpu::new(Box::new(bus));
        cpu.reset();
        RustyNesUi {
            cpu: Arc::new(RwLock::new(cpu)),
            stop_tx: None,
            halted_rx: None,
            memory_start_address: "0000".to_string(),
            memory_end_address: "0100".to_string(),
            memory_write_address: "0000".to_string(),
            memory_write_value: "00".to_string(),
            first_frame: true,
        }
    }
}

impl eframe::App for RustyNesUi {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Options");
            if ui.button("Organize Windows").clicked() {
                ui.ctx().memory().reset_areas();
            }
        });

        self.draw_register_window(ctx);
        self.draw_memory_window(ctx);
        self.draw_controls_window(ctx);
        self.draw_disassembly_window(ctx);
        self.draw_stack_window(ctx);
        self.draw_memory_write_window(ctx);
        //self.draw_display_window(ctx);
        //self.handle_input(ctx);

        if self.first_frame {
            self.first_frame = false;
            ctx.memory().reset_areas();
        }

    }
}

impl RustyNesUi {
    fn draw_register_window(&self, ctx: &Context) {
        let cpu = self.cpu.read();
        egui::Window::new("Registers")
            .resizable(false)
            .show(ctx, |ui| {
                egui::Grid::new("register_grid")
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("PC");
                        ui.label(format!("0x{:04X}", cpu.register_pc));
                        ui.end_row();
                        ui.label("SP");
                        ui.label(format!("0x{:02X}", cpu.register_sp));
                        ui.end_row();
                        ui.label("A");
                        ui.label(format!("0x{:02X}", cpu.register_a));
                        ui.end_row();
                        ui.label("X");
                        ui.label(format!("0x{:02X}", cpu.register_x));
                        ui.end_row();
                        ui.label("Y");
                        ui.label(format!("0x{:02X}", cpu.register_y));
                        ui.end_row();
                        ui.label("P");
                        ui.label(format!("0x{:02X}", cpu.status_flags.status));
                        ui.end_row();
                    });
                egui::Grid::new("flag_grid")
                    .striped(false)
                    .min_col_width(10.0)
                    .max_col_width(10.0)
                    .show(ui, |ui| {
                        ui.label("N");
                        ui.label("V");
                        ui.label(" ");
                        ui.label("B");
                        ui.label("D");
                        ui.label("I");
                        ui.label("Z");
                        ui.label("C");
                        ui.end_row();
                        ui.label(if cpu.status_flags.get_negative() {
                            "1"
                        } else {
                            "0"
                        });
                        ui.label(if cpu.status_flags.get_overflow() {
                            "1"
                        } else {
                            "0"
                        });
                        ui.label(if cpu.status_flags.get_break_2() {
                            "1"
                        } else {
                            "0"
                        });
                        ui.label(if cpu.status_flags.get_break() {
                            "1"
                        } else {
                            "0"
                        });
                        ui.label(if cpu.status_flags.get_decimal() {
                            "1"
                        } else {
                            "0"
                        });
                        ui.label(if cpu.status_flags.get_interrupt() {
                            "1"
                        } else {
                            "0"
                        });
                        ui.label(if cpu.status_flags.get_zero() {
                            "1"
                        } else {
                            "0"
                        });
                        ui.label(if cpu.status_flags.get_carry() {
                            "1"
                        } else {
                            "0"
                        });
                    });
            });
    }

    fn draw_controls_window(&mut self, ctx: &Context) {
        egui::Window::new("Controls")
            .resizable(false)
            .show(ctx, |ui| {
                if self.stop_tx.is_some() {
                    if self.halted_rx.as_ref().unwrap().try_recv().is_ok() {
                        self.stop_tx = None;
                        self.halted_rx = None;
                    } else if ui.button("Stop").clicked() {
                        self.stop_tx.as_ref().unwrap().send(()).unwrap();
                        self.stop_tx = None;
                        self.halted_rx = None;
                    }
                } else {
                    if ui.button("Run").clicked() {
                        self.create_run_thread(false);
                    }
                    if ui.button("Run and save trace").clicked() {
                        self.create_run_thread(true);
                    }
                    if ui.button("Step").clicked() {
                        self.cpu.write().step();
                    }
                    if ui.button("Reset").clicked() {
                        self.cpu.write().reset();
                    }
                }
            });
    }

    fn draw_memory_window(&mut self, ctx: &Context) {
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
                let start = validate_word(&mut self.memory_start_address, old_start_address);
                let end = validate_word(&mut self.memory_end_address, old_end_address);

                egui::Grid::new("memory_grid")
                    .striped(true)
                    .min_col_width(10.0)
                    .show(ui, |ui| {
                        let draw_start = start / 16 * 16;
                        let draw_end = if end > 0x800 && end - 0x800 > start {
                            start + 0x800
                        } else {
                            end
                        };
                        let cpu = self.cpu.read();
                        for address in draw_start..draw_end {
                            if address % 16 == 0 {
                                ui.label(format!("{:04X}", address));
                            }
                            if address >= start {
                                match cpu.bus.read(address) {
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
                        if end == 0xFFFF {
                            match cpu.bus.read(0xFFFF) {
                                Ok(value) => ui.label(format!("{:02X}", value)),
                                Err(_) => ui.label("--"),
                            };
                        }
                        if draw_end != end {
                            ui.label("...");
                            ui.end_row();
                        }
                    });
            });

    }

    fn draw_disassembly_window(&mut self, ctx: &Context) {
        egui::Window::new("Disassembly")
            .resizable(false)
            .vscroll(false)
            .show(ctx, |ui| {
                egui::Grid::new("disassembly_grid")
                    .striped(true)
                    .min_col_width(10.0)
                    .show(ui, |ui| {
                        let cpu = self.cpu.read();
                        let mut pc = cpu.register_pc;
                        for _ in 0..20 {
                            let def = Instruction::default();
                            let disassembly = cpu.disassemble(pc, [
                                cpu.bus.read(pc).unwrap_or(0),
                                cpu.bus.read(pc.wrapping_add(1)).unwrap_or(0),
                                cpu.bus.read(pc.wrapping_add(2)).unwrap_or(0),
                            ]).unwrap_or(def);
                            ui.label(format!("{:04X}", pc));
                            ui.label(disassembly.to_string());
                            pc += disassembly.length;
                            ui.end_row();
                        }
                    });
            });

    }

    fn draw_stack_window(&mut self, ctx: &Context) {
        egui::Window::new("Stack")
            .resizable(true)
            .vscroll(true)
            .show(ctx, |ui| {
                egui::Grid::new("stack_grid")
                    .striped(true)
                    .min_col_width(10.0)
                    .show(ui, |ui| {
                        let cpu = self.cpu.read();
                        for i in (0x100_u16..0x200_u16).step_by(1).rev() {
                            if (i & 0xFF) as u8 == cpu.register_sp {
                                ui.label("SP => ");
                            } else {
                                ui.label("");
                            }
                            ui.label(format!("{:04X}", i));
                            match cpu.bus.read(i) {
                                Ok(value) => ui.label(format!("{:02X}", value)),
                                Err(_) => ui.label("--"),
                            };
                            match cpu.bus.read_word(i) {
                                Ok(value) => ui.label(format!("{:04X}", value)),
                                Err(_) => ui.label("--"),
                            };
                            ui.end_row();
                        }
                    });
            });
    }

    fn draw_memory_write_window(&mut self, ctx: &Context) {
        egui::Window::new("Memory Write")
            .resizable(false)
            .vscroll(false)
            .show(ctx, |ui| {
                let old_address = self.memory_write_address.clone();
                let old_value = self.memory_write_value.clone();
                ui.horizontal(|ui| {
                    ui.label("Address");
                    ui.text_edit_singleline(&mut self.memory_write_address);
                });
                ui.horizontal(|ui| {
                    ui.label("Value");
                    ui.text_edit_singleline(&mut self.memory_write_value);
                });
                let address = validate_word(&mut self.memory_write_address, old_address);
                let value = validate_byte(&mut self.memory_write_value, old_value);

                if ui.button("Write").clicked() {
                    self.cpu.write().bus.write(address, value);
                }
            });
    }

    fn draw_display_window(&mut self, ctx: &Context) {
        egui::Window::new("Display")
            .resizable(false)
            .show(ctx, |ui| {
                let (mut response, painter) =
                    ui.allocate_painter(Vec2 { x: 320.0, y: 320.0 }, Sense::click());

                let cpu = self.cpu.read();

                if self.stop_tx.is_some() {
                    ctx.request_repaint();
                }

                for row in 0..32 {
                    for col in 0..32 {
                        let color_code = cpu.bus.read(0x0200 + row * 32 + col).unwrap();
                        let color = match color_code & 0xF {
                            0x00 => Color32::BLACK,
                            0x01 => Color32::WHITE,
                            0x02 => Color32::RED,
                            0x03 => Color32::BLUE,
                            0x04 => Color32::KHAKI,
                            0x05 => Color32::GREEN,
                            0x06 => Color32::DARK_BLUE,
                            0x07 => Color32::YELLOW,
                            0x08 => Color32::LIGHT_RED,
                            0x09 => Color32::BROWN,
                            0x0A => Color32::DARK_RED,
                            0x0B => Color32::DARK_GRAY,
                            0x0C => Color32::GRAY,
                            0x0D => Color32::LIGHT_GREEN,
                            0x0E => Color32::LIGHT_YELLOW,
                            0x0F => Color32::LIGHT_GRAY,
                            _ => {
                                println!("WEIRD COLOR AT {:X}", 0x0200 + row * 32 + col);
                                Color32::GOLD
                            }
                        };
                        painter.rect_filled(
                            Rect::from_min_size(
                                painter.round_pos_to_pixels(
                                    painter.clip_rect().min
                                        + Vec2::new(10.0 * col as f32, 10.0 * row as f32),
                                ),
                                painter.round_vec_to_pixels(Vec2::new(10.0, 10.0)),
                            ),
                            Rounding::none(),
                            color,
                        );
                    }
                }
                response.mark_changed();
            });
    }

    fn handle_input(&mut self, ctx: &Context) {
        if self.stop_tx.is_some() {
            if ctx.input().key_pressed(Key::W) {
                self.cpu.write().bus.write(0xFF, 0x77).unwrap();
            } else if ctx.input().key_pressed(Key::A) {
                self.cpu.write().bus.write(0xFF, 0x61).unwrap();
            } else if ctx.input().key_pressed(Key::S) {
                self.cpu.write().bus.write(0xFF, 0x73).unwrap();
            } else if ctx.input().key_pressed(Key::D) {
                self.cpu.write().bus.write(0xFF, 0x64).unwrap();
            }
        }
    }

    fn create_run_thread(&mut self, save_trace: bool) {
        let cpu = self.cpu.clone();
        let (stop_tx, stop_rx) = mpsc::channel();
        let (halted_tx, halted_rx) = mpsc::channel();
        self.stop_tx = Some(stop_tx);
        self.halted_rx = Some(halted_rx);
        thread::spawn(move || {
            let mut trace_vec = Vec::new();
            'main: loop {
                if stop_rx.try_recv().is_ok() {
                    break;
                }
                for _ in 0..300 {
                    let mut cpu_lock = cpu.write();
                    if save_trace {
                        trace_vec.push(cpu_lock.trace());
                    }
                    match cpu_lock.step() {
                        Ok(halted) => {
                            if halted {
                                halted_tx.send(()).unwrap();
                                break 'main;
                            }
                        }
                        Err(_) => {
                            halted_tx.send(()).unwrap();
                            break 'main;
                        }
                    }
                }
                thread::sleep(Duration::from_nanos(1_000_000_000 / 60));
            }
            if save_trace {
                let mut to_write = trace_vec.iter().map(|trace| trace.to_string()).collect::<Vec<String>>().join("\n");
                to_write.push('\n');
                fs::write("log", to_write).unwrap();
            }
        });
    }
}

fn validate_word(to_validate: &mut String, old: String) -> u16 {
    u16::from_str_radix(to_validate, 16).unwrap_or_else(|_| {
        to_validate.clear();
        to_validate.push_str(&old);
        u16::from_str_radix(to_validate, 16).unwrap()
    })
}

fn validate_byte(to_validate: &mut String, old: String) -> u8 {
    u8::from_str_radix(to_validate, 16).unwrap_or_else(|_| {
        to_validate.clear();
        to_validate.push_str(&old);
        u8::from_str_radix(to_validate, 16).unwrap()
    })
}