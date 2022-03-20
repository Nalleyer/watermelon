use eframe::{egui, epaint::FontFamily, epi};

use crate::arg::Args;
use crate::gap_buffer::GapBuffer;
use clap::Parser;
use std::error::Error;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct MainApp {
    buffer: GapBuffer,
}

impl MainApp {
    pub fn setup(&mut self) -> Result<(), Box<dyn Error>> {
        let args = Args::parse();
        println!("{:?}", args);

        let file_content = std::fs::read_to_string(args.file)?;

        let buffer = GapBuffer::from_string(file_content);

        println!("{:?}", buffer);

        self.buffer = buffer;

        Ok(())
    }
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            buffer: GapBuffer::default(),
        }
    }
}

impl epi::App for MainApp {
    fn name(&self) -> &str {
        "🍉watermelon"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }

        let mut fonts = egui::FontDefinitions::default();

        let fira_code =
            egui::FontData::from_static(include_bytes!("../fonts/FiraCode-Regular.ttf"));
        fonts.font_data.insert("fira_code".to_owned(), fira_code);

        let chinese = egui::FontData::from_static(include_bytes!("../fonts/wqy-zenhei.ttc"));
        fonts.font_data.insert("wqy".to_owned(), chinese);

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "fira_code".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("wqy".to_owned());

        ctx.set_fonts(fonts);

        let mut style: egui::Style = (*ctx.style()).clone();
        style.override_font_id = Some(egui::FontId {
            size: 16.0,
            family: egui::FontFamily::Monospace,
        });
        style.override_text_style = Some(egui::TextStyle::Monospace);
        ctx.set_style(style);
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ctx.available_rect();

            let painter = ui.painter_at(rect);

            let mut iter = self.buffer.iter();
            let mut current_string = "".to_string();
            let mut last_char: Option<char> = None;
            let mut current_line = 0usize;
            while let Some(ch) = iter.next() {
                if ch == '\r' {
                    continue;
                }
                if ch == '\n' {
                    if !last_char.is_none() {
                        draw_line(&painter, &current_string, current_line, rect);
                        current_string.clear();
                        last_char = None;
                    }
                    current_line += 1;
                    continue;
                }
                current_string.push(ch);
                last_char = Some(ch);
            }
            draw_line(&painter, &current_string, current_line, rect);
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}

fn draw_line(painter: &egui::Painter, content: &str, linenumber: usize, rect: egui::Rect) {
    let rect_after_ln = painter.text(
        egui::Pos2 {
            x: rect.min.x,
            y: rect.min.y + (linenumber as f32) * 25.0, // TODO 字号设置
        },
        egui::Align2::LEFT_TOP,
        linenumber,
        egui::FontId {
            size: 25.0,
            family: FontFamily::Monospace,
        },
        egui::Color32::GRAY,

    );
    painter.text(
        egui::Pos2 {
            x: rect.min.x + 30.0,
            y: rect.min.y + (linenumber as f32) * 25.0, // TODO 字号设置
        },
        egui::Align2::LEFT_TOP,
        content,
        egui::FontId {
            size: 25.0,
            family: FontFamily::Monospace,
        },
        egui::Color32::BLACK,
    );
}
