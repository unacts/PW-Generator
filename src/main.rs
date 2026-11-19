#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use rand::seq::SliceRandom;
use rand::Rng;

const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()-_=+[]{}|;:,.<>?";
const AMBIGUOUS: &[u8] = b"Il1O0";

struct PasswordGeneratorApp {
    length: usize,
    use_lower: bool,
    use_upper: bool,
    use_numbers: bool,
    use_symbols: bool,
    exclude_ambiguous: bool,
    password: String,
    copied_feedback: bool,
}

impl Default for PasswordGeneratorApp {
    fn default() -> Self {
        let mut app = Self {
            length: 16,
            use_lower: true,
            use_upper: true,
            use_numbers: true,
            use_symbols: true,
            exclude_ambiguous: false,
            password: String::new(),
            copied_feedback: false,
        };
        app.regenerate();
        app
    }
}

impl PasswordGeneratorApp {
    fn filter_ambiguous(chars: &[u8], exclude: bool) -> Vec<u8> {
        if !exclude {
            return chars.to_vec();
        }
        chars
            .iter()
            .copied()
            .filter(|c| !AMBIGUOUS.contains(c))
            .collect()
    }

    fn regenerate(&mut self) {
        self.copied_feedback = false;
        let mut rng = rand::thread_rng();
        let mut pool: Vec<u8> = Vec::new();
        let mut required: Vec<u8> = Vec::new();

        let sets: [(bool, &[u8]); 4] = [
            (self.use_lower, LOWERCASE),
            (self.use_upper, UPPERCASE),
            (self.use_numbers, NUMBERS),
            (self.use_symbols, SYMBOLS),
        ];

        for (enabled, raw_set) in sets {
            if enabled {
                let filtered = Self::filter_ambiguous(raw_set, self.exclude_ambiguous);
                if let Some(&ch) = filtered.choose(&mut rng) {
                    required.push(ch);
                    pool.extend_from_slice(&filtered);
                }
            }
        }

        if pool.is_empty() {
            self.password = String::from("Select at least one character set");
            return;
        }

        while required.len() < self.length {
            let idx = rng.gen_range(0..pool.len());
            required.push(pool[idx]);
        }

        required.truncate(self.length);
        required.shuffle(&mut rng);
        self.password = String::from_utf8(required).unwrap_or_default();
    }

    fn strength_info(&self) -> (&'static str, f32, egui::Color32) {
        let categories = [
            self.use_lower,
            self.use_upper,
            self.use_numbers,
            self.use_symbols,
        ]
        .iter()
        .filter(|&&x| x)
        .count();

        if categories == 0 {
            return ("NONE", 0.0, egui::Color32::from_rgb(120, 120, 130));
        }

        let score = (self.length as f32 / 24.0).min(1.0) * 0.65 + (categories as f32 / 4.0) * 0.35;

        if self.length >= 16 && categories >= 3 {
            ("VERY STRONG", score.max(0.9), egui::Color32::from_rgb(46, 204, 113))
        } else if self.length >= 12 && categories >= 3 {
            ("STRONG", score.max(0.7), egui::Color32::from_rgb(52, 152, 219))
        } else if self.length >= 8 && categories >= 2 {
            ("MODERATE", score.max(0.45), egui::Color32::from_rgb(241, 196, 15))
        } else {
            ("WEAK", score.max(0.2), egui::Color32::from_rgb(231, 76, 60))
        }
    }
}

impl eframe::App for PasswordGeneratorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.visuals = egui::Visuals::dark();
        style.visuals.window_fill = egui::Color32::from_rgb(18, 20, 26);
        style.visuals.panel_fill = egui::Color32::from_rgb(18, 20, 26);
        style.spacing.item_spacing = egui::vec2(10.0, 12.0);
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(6.0);

            ui.horizontal(|ui| {
                ui.heading(
                    egui::RichText::new("Password Generator")
                        .size(24.0)
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new("v1.0 • Rust")
                            .size(12.0)
                            .color(egui::Color32::from_rgb(130, 140, 160)),
                    );
                });
            });

            ui.label(
                egui::RichText::new("Generate cryptographically random, secure passwords locally.")
                    .size(13.0)
                    .color(egui::Color32::from_rgb(150, 158, 175)),
            );

            ui.add_space(6.0);

            // Output card
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(28, 32, 42))
                .rounding(egui::Rounding::same(10.0))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(48, 55, 72)))
                .inner_margin(egui::Margin::same(14.0))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(&self.password)
                                    .monospace()
                                    .size(18.0)
                                    .color(egui::Color32::from_rgb(235, 242, 255)),
                            )
                            .truncate(),
                        );
                    });

                    ui.add_space(8.0);

                    let (label, progress, color) = self.strength_info();
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(format!("Strength: {}", label))
                                .size(12.0)
                                .strong()
                                .color(color),
                        );
                    });

                    let bar = egui::ProgressBar::new(progress)
                        .desired_height(6.0)
                        .fill(color);
                    ui.add(bar);
                });

            ui.add_space(4.0);

            // Action buttons
            ui.horizontal(|ui| {
                let copy_label = if self.copied_feedback {
                    "Copied to Clipboard!"
                } else {
                    "Copy Password"
                };

                let copy_btn = egui::Button::new(
                    egui::RichText::new(copy_label)
                        .size(14.0)
                        .strong()
                        .color(egui::Color32::WHITE),
                )
                .fill(if self.copied_feedback {
                    egui::Color32::from_rgb(39, 174, 96)
                } else {
                    egui::Color32::from_rgb(59, 130, 246)
                })
                .rounding(egui::Rounding::same(8.0))
                .min_size(egui::vec2(195.0, 38.0));

                if ui.add(copy_btn).clicked() {
                    ctx.copy_text(self.password.clone());
                    self.copied_feedback = true;
                }

                let regen_btn = egui::Button::new(
                    egui::RichText::new("Regenerate")
                        .size(14.0)
                        .strong()
                        .color(egui::Color32::from_rgb(225, 230, 240)),
                )
                .fill(egui::Color32::from_rgb(38, 43, 56))
                .rounding(egui::Rounding::same(8.0))
                .min_size(egui::vec2(195.0, 38.0));

                if ui.add(regen_btn).clicked() {
                    self.regenerate();
                }
            });

            ui.add_space(4.0);

            // Settings card
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(24, 27, 36))
                .rounding(egui::Rounding::same(10.0))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(42, 48, 62)))
                .inner_margin(egui::Margin::same(14.0))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());

                    let mut changed = false;

                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Password Length")
                                .size(14.0)
                                .strong()
                                .color(egui::Color32::from_rgb(220, 226, 238)),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(format!("{} chars", self.length))
                                    .monospace()
                                    .size(14.0)
                                    .strong()
                                    .color(egui::Color32::from_rgb(96, 165, 250)),
                            );
                        });
                    });

                    if ui
                        .add(
                            egui::Slider::new(&mut self.length, 6..=64)
                                .show_value(false),
                        )
                        .changed()
                    {
                        changed = true;
                    }

                    ui.separator();

                    ui.columns(2, |cols| {
                        if cols[0]
                            .checkbox(&mut self.use_upper, "Uppercase (A-Z)")
                            .changed()
                        {
                            changed = true;
                        }
                        if cols[0]
                            .checkbox(&mut self.use_lower, "Lowercase (a-z)")
                            .changed()
                        {
                            changed = true;
                        }
                        if cols[1]
                            .checkbox(&mut self.use_numbers, "Numbers (0-9)")
                            .changed()
                        {
                            changed = true;
                        }
                        if cols[1]
                            .checkbox(&mut self.use_symbols, "Symbols (!@#$...)")
                            .changed()
                        {
                            changed = true;
                        }
                    });

                    ui.add_space(2.0);
                    if ui
                        .checkbox(
                            &mut self.exclude_ambiguous,
                            "Exclude ambiguous characters (I, l, 1, O, 0)",
                        )
                        .changed()
                    {
                        changed = true;
                    }

                    if changed {
                        self.regenerate();
                    }
                });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([440.0, 430.0])
            .with_resizable(false)
            .with_title("Password Generator"),
        ..Default::default()
    };

    eframe::run_native(
        "Password Generator",
        options,
        Box::new(|_cc| Ok(Box::new(PasswordGeneratorApp::default()))),
    )
}