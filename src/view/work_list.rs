
use egui::Ui;
use egui_extras::{Column, TableBody, TableBuilder};

pub struct WorkList {
    pub(crate) works: Vec<(crate::db::Work, Option<String>)>,
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
        work: &(crate::db::Work, Option<String>))
    {
        body.row(16.0, |mut row| {
            row.col(|ui| { ui.label("Rating"); });
            row.col(|ui| { ui.label(&work.0.title); });
            row.col(|ui| { 
                ui.label(work.0.release_date.as_deref().unwrap_or("N/A"));
            });
            row.col(|ui| { 
                ui.label(work.0.runtime.map_or(
                    "N/A".to_string(), |v| v.to_string() + " min"
                ));
            });
            row.col(|ui| { 
                ui.label(
                    work.1.as_deref().unwrap_or("N/A")
                );
            });
        });
    }    
}

