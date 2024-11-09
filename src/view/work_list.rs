
use egui::Ui;

pub struct WorkList {
    pub(crate) works: Vec<crate::db::Work>,
}

impl WorkList {
    pub fn render(&mut self, ui: &mut Ui) {
        ui.heading("List of Works");

/*         if self.works.is_none() {
            self.works = Some(query::select_all_works(sql_conn));
        } */

        egui::ScrollArea::vertical().show(ui, |ui| {
            for work in &self.works {
                ui.label(work.title.to_string());
            }                        
        });
    }
}