#![allow(unused)]
use crate::db::*;
use crate::{
    db::Work,
    view::{self, work_list::WorkList},
};
use diesel::SqliteConnection;
use eframe::egui::{self, FontData, FontDefinitions, FontFamily, RichText};

pub struct MovieApp {
    label: String,
    value: f32,
    sql_conn: SqliteConnection,
    work_list: Option<WorkList>,
}

impl MovieApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, sql_conn: SqliteConnection) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.5);

        configure_fonts(&cc.egui_ctx);
        MovieApp::with_connection(sql_conn)
    }

    fn with_connection(sql_conn: SqliteConnection) -> Self {
        Self {
            label: "Hello World!".to_owned(),
            value: 2.7,
            sql_conn,
            work_list: None,
        }
    }
}

impl eframe::App for MovieApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // call render on work_list.rs
            if self.work_list.is_none() {
                self.work_list = Some(WorkList::new(
                    query::select_all_works(&mut self.sql_conn),
                    None,
                ));
            }

            // Render WorkList if it exists
            if let Some(work_list) = &mut self.work_list {
                work_list.render(ctx, ui);
            }

            ui.separator();
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "code_2000".to_owned(),
        FontData::from_static(include_bytes!("../asset/font/CODE2000.TTF")),
    );

    fonts
        .families
        .entry(FontFamily::Name("Code2000".into()))
        .or_default()
        .push("code_2000".to_owned());

    ctx.set_fonts(fonts);
}
