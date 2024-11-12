
use egui::{FontFamily, RichText, Ui};
use egui_extras::{Column, TableBody, TableBuilder};

use crate::db::WorkWithDetails;

pub struct WorkList {
    pub(crate) works: 
        Vec<WorkWithDetails>,
}

impl WorkList {
    pub fn render(&mut self, ui: &mut Ui) {
        ui.heading("List of Works");

        egui::ScrollArea::vertical()
            .show(ui, |ui| {
                self.build_work_list_table(ui);                      
            }
        );
    }


    fn build_work_list_table(&self, ui: &mut Ui) {
        TableBuilder::new(ui)
            .striped(true)
            .column(Column::auto())
            .column(Column::remainder())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .header(20.0, |mut header| {
                header.col(|ui| { ui.heading("Rating"); });
                header.col(|ui| { ui.heading("Name"); });
                header.col(|ui| { ui.heading("Release"); });
                header.col(|ui| { ui.heading("Runtime"); });
                header.col(|ui| { ui.heading("Type"); });
            })
            .body(|mut body| {
                for work in &self.works {
                    Self::add_table_row(&mut body, work);
                }
            })
            ;
    }


    fn add_table_row(
        body: &mut TableBody,
        work: &WorkWithDetails,
    ) {
        body.row(16.0, |mut row| {
            row.col(|ui| { 
                ui.label(
                    RichText::new(
                        Self::rating_to_stars(&work.rating_value)
                    )
                    .family(FontFamily::Name("Code2000".into())) 
                );          
             });
            row.col(|ui| { ui.label(work.title.as_str()); });
            row.col(|ui| { 
                ui.label(work.release_date.as_deref().unwrap_or("N/A"));
            });
            row.col(|ui| {                 
                ui.label(work.runtime.map_or(
                    "N/A".to_string(), |v| v.to_string() + " min"
                ));
            });
            row.col(|ui| { 
                ui.label(
                    work.work_type_name.as_deref().unwrap_or("N/A")
                );
            });
        });
    }    


    fn rating_to_stars(rating_value: &Option<i32>) -> String {
        match rating_value {
            Some(x) => {
                format!(
                    "{}{}",
                    "★".repeat((x / 2) as usize),
                    if x % 2 == 1 {"⯨"} else {""}
                )                
            },
            None => "".to_string(),
        }
    }
}

