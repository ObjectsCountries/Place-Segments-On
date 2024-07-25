extern crate eframe;
extern crate egui;
#[derive(Debug, Clone, Default, PartialEq)]
enum Segment {
    #[default]
    Empty,
    User,
    UserDevice,
    Battery,
    Network,
    Time(String),
    Custom(String),
}

#[derive(Default)]
struct PlaceSegmentsOn {
    new_segment: Segment,
    segments: Vec<Segment>,
    strftime: String,
    custom_input: String,
}

impl PlaceSegmentsOn {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for PlaceSegmentsOn {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_singleline(&mut self.strftime);
            ui.text_edit_singleline(&mut self.custom_input);
            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", self.new_segment))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.new_segment, Segment::User, "User");
                    ui.selectable_value(
                        &mut self.new_segment,
                        Segment::UserDevice,
                        "User & Device",
                    );
                    ui.selectable_value(&mut self.new_segment, Segment::Empty, "Empty");
                    ui.selectable_value(&mut self.new_segment, Segment::Battery, "Battery");
                    ui.selectable_value(&mut self.new_segment, Segment::Network, "Network");
                    ui.selectable_value(
                        &mut self.new_segment,
                        Segment::Time(self.strftime.clone()),
                        "STRF Time",
                    );
                    ui.selectable_value(
                        &mut self.new_segment,
                        Segment::Custom(self.custom_input.clone()),
                        "Custom",
                    );
                });
            if ui.button("Add").clicked() {
                self.segments.push(self.new_segment.clone());
            }
            ui.label(format!("{:?}", self.segments));
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Place Segments On",
        native_options,
        Box::new(|cc| Ok(Box::new(PlaceSegmentsOn::new(cc)))),
    )
    .unwrap();
}
