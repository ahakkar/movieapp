use egui::{Button, Context, FontFamily, Response, RichText, Ui, Vec2};
use egui_extras::{Column, TableBody, TableBuilder};

use super::rate_movie::RateMovie;
use crate::{constants, db::WorkWithDetails};

pub struct WorkList {
    works: Vec<WorkWithDetails>,
    rate_movie: Option<RateMovie>,
}

// What events are possible, ie. rate movie, view movie info, perhaps
// sort columns by value etc.
enum WorkListEvent {
    RateClicked(WorkWithDetails),
}

const ROW_HEIGHT: f32 = 16.0;

impl WorkList {
    pub fn new(
        works: Vec<WorkWithDetails>,
        rate_movie: Option<RateMovie>,
    ) -> Self {
        Self { works, rate_movie }
    }
    pub fn render(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.heading("List of Works");

        // Disable underlying view while the user is giving a rating
        let events = if self.rate_movie.is_some() {
            let mut events = Vec::new();
            ui.add_enabled_ui(false, |ui| {
                events = self.vertical_scrollarea(ui);
            });
            events
        } else {
            self.vertical_scrollarea(ui)
        };

        // Event handler
        for event in events {
            match event {
                WorkListEvent::RateClicked(work) => {
                    self.rate_movie = Some(RateMovie::new(work));
                }
            }
        }

        if let Some(rm) = &mut self.rate_movie {
            let should_close = rm.render(ctx, ui);

            if should_close {
                self.rate_movie = None;
            }
        }
    }

    fn vertical_scrollarea(&self, ui: &mut Ui) -> Vec<WorkListEvent> {
        let mut events = Vec::new();
        egui::ScrollArea::vertical()
            .show(ui, |ui| events = self.build_work_list_table(ui));
        events
    }

    fn build_work_list_table(&self, ui: &mut Ui) -> Vec<WorkListEvent> {
        let mut events = Vec::new();
        TableBuilder::new(ui)
            .striped(true)
            .column(Column::auto())
            .column(Column::remainder())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("Rating");
                });
                header.col(|ui| {
                    ui.heading("Name");
                });
                header.col(|ui| {
                    ui.heading("Release");
                });
                header.col(|ui| {
                    ui.heading("Runtime");
                });
                header.col(|ui| {
                    ui.heading("Type");
                });
            })
            .body(|mut body| {
                for work in &self.works {
                    if let Some(ev) = Self::add_table_row(&mut body, work) {
                        events.push(ev);
                    }
                }
            });
        events
    }

    fn add_table_row(
        body: &mut TableBody,
        work: &WorkWithDetails,
    ) -> Option<WorkListEvent> {
        let mut event = None;
        body.row(16.0, |mut row| {
            row.col(|ui| {
                if Self::add_rating_button(ui, &work.rating_value).clicked() {
                    event = Some(WorkListEvent::RateClicked(work.clone()));
                }
            });
            row.col(|ui| {
                ui.label(work.title.as_str());
            });
            row.col(|ui| {
                ui.label(work.release_date.as_deref().unwrap_or("N/A"));
            });
            row.col(|ui| {
                ui.label(
                    work.runtime
                        .map_or("N/A".to_string(), |v| v.to_string() + " min"),
                );
            });
            row.col(|ui| {
                ui.label(work.work_type_name.as_deref().unwrap_or("N/A"));
            });
        });
        event
    }

    fn add_rating_button(ui: &mut Ui, rating_value: &Option<i32>) -> Response {
        let text = RichText::new(Self::rating_to_stars(rating_value))
            .family(FontFamily::Name("Code2000".into()));
        ui.add(
            Button::new(text)
                .frame(false)
                .min_size(Vec2::new(40.0, ROW_HEIGHT)),
        )
    }

    fn rating_to_stars(rating_value: &Option<i32>) -> String {
        match rating_value {
            Some(x) => {
                format!(
                    "{}{}",
                    constants::FULL_STAR.repeat((x / 2) as usize),
                    if x % 2 == 1 { constants::HALF_STAR } else { "" }
                )
            }
            None => "".to_string(),
        }
    }
}
