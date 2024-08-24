use common::RandomStringGenerator;
use eframe::{self, egui, egui_wgpu::WgpuConfiguration, wgpu::Backends};
use std::error::Error;

const ICON_IMAGE: &[u8] = include_bytes!("icon.png");

fn main() -> eframe::Result {
    let string_generator = RandomStringGenerator {
        length: 64,
        ..Default::default()
    };

    let app = MyApp {
        string_generator,
        generated_strings: string_generator.generate(),
    };

    let viewport = egui::ViewportBuilder::default()
        .with_inner_size(egui::vec2(750.0, 400.0))
        .with_resizable(false)
        .with_maximize_button(false)
        .with_icon((eframe::icon_data::from_png_bytes(ICON_IMAGE)).unwrap());

    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        viewport,
        wgpu_options: WgpuConfiguration {
            supported_backends: Backends::DX12,
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "Random String Generator",
        options,
        Box::new(|_| Ok(Box::new(app))),
    )
}

struct MyApp {
    string_generator: RandomStringGenerator,
    generated_strings: Result<Vec<String>, Box<dyn Error>>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("settings_panel")
            .default_width(0.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| ui.heading("Settings"));
                ui.add_space(10.0);

                let mut string_count = if self.string_generator.count == 0 {
                    "".to_string()
                } else {
                    self.string_generator.count.to_string()
                };

                let mut string_length = if self.string_generator.length == 0 {
                    "".to_string()
                } else {
                    self.string_generator.length.to_string()
                };

                egui::Grid::new("num_grid")
                    .max_col_width(ui.available_width())
                    .show(ui, |ui| {
                        let count_label = ui.label("Number of strings to generate: ");
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut string_count).desired_width(50.0),
                            )
                            .labelled_by(count_label.id);
                        });
                        ui.end_row();

                        let length_label = ui.label("Length of the string(s): ");
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut string_length).desired_width(50.0),
                            )
                            .labelled_by(length_label.id);
                        });
                        ui.end_row();
                    });

                self.string_generator.count = if string_count.is_empty() {
                    0
                } else {
                    string_count.parse().unwrap_or(self.string_generator.count)
                };

                self.string_generator.length = if string_length.is_empty() {
                    0
                } else {
                    string_length
                        .parse()
                        .unwrap_or(self.string_generator.length)
                };

                ui.checkbox(
                    &mut self.string_generator.include_numbers,
                    "Include numbers",
                );
                ui.checkbox(
                    &mut self.string_generator.include_uppercase,
                    "Include uppercase letters",
                );
                ui.checkbox(
                    &mut self.string_generator.include_special_chars,
                    "Include special charecters",
                );

                if ui
                    .add_enabled(
                        self.string_generator.count != 0 && self.string_generator.length != 0,
                        egui::Button::new("Generate"),
                    )
                    .clicked()
                {
                    self.generated_strings = self.string_generator.generate()
                };
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.heading("Output"));
            ui.add_space(10.0);

            match &self.generated_strings {
                Ok(strings) => {
                    egui::ScrollArea::vertical().auto_shrink(false).show_rows(
                        ui,
                        ui.text_style_height(&egui::TextStyle::Body),
                        strings.len(),
                        |ui, _row_range| {
                            for string in strings {
                                ui.horizontal(|ui| {
                                    if ui.button("📋").on_hover_text("Copy to clipboard").clicked()
                                    {
                                        ui.output_mut(|o| o.copied_text = string.clone());
                                    }

                                    ui.add(egui::Label::new(string).wrap());
                                });

                                ui.add_space(5.0);
                            }
                        },
                    );
                }

                Err(err) => {
                    ui.label(format!("Error: {}", err));
                }
            }
        });
    }
}
