#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::only_used_in_recursion)]
#![allow(clippy::never_loop)]
#![allow(clippy::useless_vec)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::repeat_once)]

use egui::{Button, Context, FontFamily, Response, RichText, Ui, Vec2};

use crate::{
    constants::{self},
    db::WorkWithDetails,
};

pub struct RateMovie {
    work: WorkWithDetails,
    rating_text: RichText,
    rating_value: Option<i32>,
}

impl RateMovie {
    pub fn new(work: WorkWithDetails) -> RateMovie {
        Self {
            work,
            rating_text: Self::format_rating_text(
                constants::EMPTY_STAR.repeat(5),
            ),
            rating_value: None,
        }
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
                should_close = Self::rate_movie_ui(self, ui);
            });

        should_close
    }

    fn rate_movie_ui(&mut self, ui: &mut Ui) -> bool {
        // These allocations actually FORCE the size
        ui.allocate_space(Vec2::new(0.0, 10.0));

        Self::content(self, ui);

        ui.allocate_space(ui.available_size());

        // Footer returns info when window shall be closed
        Self::footer(ui)
    }

    fn content(&mut self, ui: &mut Ui) {
        // A box with half stars from 0,5 to 5?
        // If movie is already rated show the rating

        // Let the user click the amount of stars he wants
        ui.columns_const(|[col_1, col_2]| {
            Self::rate_column(self, col_1);
            Self::ratings_column(self, col_2);
        });
    }

    fn rate_column(&mut self, col: &mut Ui) {
        col.label("Rate");
        col.label(format!("Rate movie: {}\n", self.work.title));
        col.label(format!("Movie ID: {}", self.work.work_id));

        let mut button_response: Option<Response> = None;

        col.with_layout(
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.allocate_space(egui::vec2(20.0, 0.0));

                button_response = Some(
                    ui.add(
                        Button::new(self.rating_text.clone())
                            .min_size(Vec2::new(50.0 * 4.0, 80.0)),
                    ),
                );

                ui.allocate_space(egui::vec2(20.0, 0.0));
            },
        );

        if let Some(response) = button_response {
            Self::rating_button_text(self, col, response);
        }
    }

    fn rating_button_text(&mut self, col: &mut Ui, response: Response) {
        if let Some(cursor_pos) = response.hover_pos() {
            let rect = response.rect;
            let relative_pos = cursor_pos - rect.min;
            let w = 20.0;

            // 5.0 stars
            if relative_pos.x > w * 9.0 {
                self.rating_text =
                    Self::format_rating_text(constants::FULL_STAR.repeat(5));
                self.rating_value = Some(10);
            // 4.5 stars
            } else if relative_pos.x > w * 8.0 {
                self.rating_text = Self::format_rating_text(format!(
                    "{}{}",
                    constants::FULL_STAR.repeat(4),
                    constants::HALF_STAR,
                ));
                self.rating_value = Some(9);
            // 4.0 stars
            } else if relative_pos.x > w * 7.0 {
                self.rating_text = Self::format_rating_text(format!(
                    "{}{}",
                    constants::FULL_STAR.repeat(4),
                    constants::EMPTY_STAR,
                ));
                self.rating_value = Some(8);
            // 3.5 stars
            } else if relative_pos.x > w * 6.0 {
                self.rating_text = Self::format_rating_text(format!(
                    "{}{}{}",
                    constants::FULL_STAR.repeat(3),
                    constants::HALF_STAR,
                    constants::EMPTY_STAR,
                ));
                self.rating_value = Some(7);
            // 3.0 stars
            } else if relative_pos.x > w * 5.0 {
                self.rating_text = Self::format_rating_text(format!(
                    "{}{}",
                    constants::FULL_STAR.repeat(3),
                    constants::EMPTY_STAR.repeat(2),
                ));
                self.rating_value = Some(6);
            // 2.5 stars
            } else if relative_pos.x > w * 4.0 {
                self.rating_text = Self::format_rating_text(format!(
                    "{}{}{}",
                    constants::FULL_STAR.repeat(2),
                    constants::HALF_STAR,
                    constants::EMPTY_STAR.repeat(2),
                ));
                self.rating_value = Some(5);
            // 2.0 stars
            } else if relative_pos.x > w * 3.0 {
                self.rating_text = Self::format_rating_text(format!(
                    "{}{}",
                    constants::FULL_STAR.repeat(2),
                    constants::EMPTY_STAR.repeat(3),
                ));
                self.rating_value = Some(4);
            // 1.5 stars
            } else if relative_pos.x > w * 2.0 {
                self.rating_text = Self::format_rating_text(format!(
                    "{}{}{}",
                    constants::FULL_STAR,
                    constants::HALF_STAR,
                    constants::EMPTY_STAR.repeat(3),
                ));
                self.rating_value = Some(3);
            // 1.0 stars
            } else if relative_pos.x > w * 1.0 {
                self.rating_text = Self::format_rating_text(format!(
                    "{}{}",
                    constants::FULL_STAR,
                    constants::EMPTY_STAR.repeat(4),
                ));
                self.rating_value = Some(2);
            // 0.5 stars
            } else if relative_pos.x > w * 0.5 {
                self.rating_text = Self::format_rating_text(format!(
                    "{}{}",
                    constants::HALF_STAR,
                    constants::EMPTY_STAR.repeat(4),
                ));
                self.rating_value = Some(1);
            // 0.0 stars
            } else {
                self.rating_text =
                    Self::format_rating_text(constants::EMPTY_STAR.repeat(5));
                self.rating_value = Some(0);
            }

            col.label(format!(
                "Relative cursor position: {:.2}, {:.2}",
                relative_pos.x, relative_pos.y
            ));
            if let Some(rating) = self.rating_value {
                col.label(format!("Rating: {}", rating));
            }
        } else {
            col.label("Cursor not over button");
            self.rating_text =
                Self::format_rating_text(constants::EMPTY_STAR.repeat(5));
            self.rating_value = None;
        }
    }

    fn format_rating_text(str: String) -> RichText {
        RichText::new(str)
            .family(FontFamily::Name("Code2000".into()))
            .size(50.0)
    }

    fn ratings_column(&self, col: &mut Ui) {
        col.label("Previous ratings");
    }

    fn footer(ui: &mut Ui) -> bool {
        let mut should_close = false;

        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            ui.horizontal_centered(|ui| {
                if Self::button(ui, "Close", constants::DEFAULT_BUTTON_SIZE) {
                    // Should first save the changed rating to database

                    should_close = true;
                }
            });
        });

        ui.add_space(10.0);

        should_close
    }

    fn button(ui: &mut Ui, text: &str, size: Vec2) -> bool {
        ui.add(Button::new(RichText::new(text)).min_size(size))
            .clicked()
    }
}
