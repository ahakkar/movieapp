use egui::{Context, Ui, Vec2};

use crate::db::WorkWithDetails;

pub struct RateMovie {
    work: WorkWithDetails,
}

impl RateMovie {
    pub fn new(work: WorkWithDetails) -> RateMovie {
        Self { work }
    }

    pub fn render(&mut self, _ctx: &Context, ui: &mut Ui) -> bool {
        let mut should_close = false;
        let size = Vec2::new(600.0, 350.0);

        // Perhaps any ratings for work with work_id
        // should be first queried from rating table

        egui::Window::new("Rate Movie")
            .collapsible(false)
            .resizable(false)
            // These three are just SUGGESTIONS
            .default_size(size)
            .min_size(size)
            .max_size(size)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ui.ctx(), |ui| {
                // This actually FORCES the size
                ui.allocate_space(Vec2::new(0.0, 10.0));

                ui.label(format!("Rate movie: {}", self.work.title));
                ui.label(format!("Movie ID: {}", self.work.work_id));

                // A box with half stars from 0,5 to 5?
                // If movie is already rated show the rating

                // Let the user click the amount of stars he wants

                ui.allocate_space(ui.available_size());

                if ui.button("Close").clicked() {
                    // Should first save the changed rating to database
                    should_close = true;
                }

                ui.add_space(10.0);
            });

        should_close
    }
}
