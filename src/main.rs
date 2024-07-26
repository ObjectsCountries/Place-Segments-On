extern crate battery;
extern crate chrono;
extern crate eframe;
extern crate egui;
extern crate egui_dnd;
extern crate gethostname;
extern crate users;
#[derive(Debug, Clone, Default, PartialEq, Hash)]
enum Segment {
    #[default]
    Empty,
    User([u8; 3], [u8; 3], bool),
    UserDevice([u8; 3], [u8; 3], bool, String),
    Battery([u8; 3], [u8; 3], bool),
    Network([u8; 3], [u8; 3], bool),
    Time([u8; 3], [u8; 3], bool, String),
    Custom([u8; 3], [u8; 3], bool, String),
}

#[derive(Default)]
struct PlaceSegmentsOn {
    new_segment: Segment,
    segments: Vec<Segment>,
    color: [u8; 3],
    bg_color: [u8; 3],
    custom_input: String,
    translation: String,
    full: String,
    preview: egui::text::LayoutJob,
    icon: bool,
}

impl PlaceSegmentsOn {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for PlaceSegmentsOn {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "Fira Code Nerd Font Mono".to_owned(),
            egui::FontData::from_static(include_bytes!("../FiraCodeNerdFontMono-Regular.ttf")),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "Fira Code Nerd Font Mono".to_owned());
        ctx.set_fonts(fonts);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.checkbox(&mut self.icon, "Add an icon before the output");
            ui.label("Put string input here");
            ui.text_edit_singleline(&mut self.custom_input);
            ui.label("Select text color");
            ui.color_edit_button_srgb(&mut self.color);
            ui.label("Select background color");
            ui.color_edit_button_srgb(&mut self.bg_color);
            egui::ComboBox::from_label("Select Segment")
                .selected_text(format!("{:?}", self.new_segment))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.new_segment,
                        Segment::User(self.color, self.bg_color, self.icon),
                        "User",
                    );
                    ui.selectable_value(
                        &mut self.new_segment,
                        Segment::UserDevice(
                            self.color,
                            self.bg_color,
                            self.icon,
                            self.custom_input.clone(),
                        ),
                        "User & Device",
                    );
                    ui.selectable_value(
                        &mut self.new_segment,
                        Segment::Battery(self.color, self.bg_color, self.icon),
                        "Battery",
                    );
                    ui.selectable_value(
                        &mut self.new_segment,
                        Segment::Network(self.color, self.bg_color, self.icon),
                        "Network",
                    );
                    ui.selectable_value(
                        &mut self.new_segment,
                        Segment::Time(
                            self.color,
                            self.bg_color,
                            self.icon,
                            self.custom_input.clone(),
                        ),
                        "STRF Time",
                    );
                    ui.selectable_value(
                        &mut self.new_segment,
                        Segment::Custom(
                            self.color,
                            self.bg_color,
                            self.icon,
                            self.custom_input.clone(),
                        ),
                        "Custom",
                    );
                });
            if ui.button("Add").clicked() {
                self.segments.push(self.new_segment.clone());
            }
            self.preview = egui::text::LayoutJob::default();
            let mut color = egui::Color32::BLACK;
            let mut bg_color = egui::Color32::WHITE;
            let mut text = String::new();
            for index in 0..self.segments.len() {
                (color, bg_color, text) = colors_text(&self.segments[index]);
                if self.segments.len() == 1 {
                    self.preview.append(
                        "",
                        0.0,
                        egui::TextFormat {
                            font_id: egui::FontId::new(14.0, egui::FontFamily::Monospace),
                            color: bg_color,
                            ..Default::default()
                        },
                    );
                    self.preview.append(
                        text.as_str(),
                        0.0,
                        egui::TextFormat {
                            font_id: egui::FontId::new(14.0, egui::FontFamily::Monospace),
                            color,
                            background: bg_color,
                            ..Default::default()
                        },
                    );
                    self.preview.append(
                        "",
                        0.0,
                        egui::TextFormat {
                            font_id: egui::FontId::new(14.0, egui::FontFamily::Monospace),
                            color: bg_color,
                            ..Default::default()
                        },
                    );
                } else {
                    if index == 0 {
                        self.preview.append(
                            "",
                            0.0,
                            egui::TextFormat {
                                font_id: egui::FontId::new(14.0, egui::FontFamily::Monospace),
                                color: bg_color,
                                ..Default::default()
                            },
                        );
                        self.preview.append(
                            text.as_str(),
                            0.0,
                            egui::TextFormat {
                                font_id: egui::FontId::new(14.0, egui::FontFamily::Monospace),
                                color,
                                background: bg_color,
                                ..Default::default()
                            },
                        );
                        if self.segments.len() > 1 {
                            self.preview.append(
                                "󰍟",
                                0.0,
                                egui::TextFormat {
                                    font_id: egui::FontId::new(14.0, egui::FontFamily::Monospace),
                                    color: bg_color,
                                    background: colors_text(&self.segments[index + 1]).1,
                                    ..Default::default()
                                },
                            );
                        }
                    } else if index == self.segments.len() - 1 {
                        self.preview.append(
                            text.as_str(),
                            0.0,
                            egui::TextFormat {
                                font_id: egui::FontId::new(14.0, egui::FontFamily::Monospace),
                                color,
                                background: bg_color,
                                ..Default::default()
                            },
                        );
                        self.preview.append(
                            "",
                            0.0,
                            egui::TextFormat {
                                font_id: egui::FontId::new(14.0, egui::FontFamily::Monospace),
                                color: bg_color,
                                ..Default::default()
                            },
                        );
                    } else {
                        self.preview.append(
                            text.as_str(),
                            0.0,
                            egui::TextFormat {
                                font_id: egui::FontId::new(14.0, egui::FontFamily::Monospace),
                                color,
                                background: bg_color,
                                ..Default::default()
                            },
                        );
                        self.preview.append(
                            "󰍟",
                            0.0,
                            egui::TextFormat {
                                font_id: egui::FontId::new(14.0, egui::FontFamily::Monospace),
                                color: bg_color,
                                background: colors_text(&self.segments[index + 1]).1,
                                ..Default::default()
                            },
                        );
                    }
                }
            }
            self.full = String::from("export PS1=\"");
            let mut prev_color: egui::Color32 = egui::Color32::BLACK;
            for segment in &self.segments {
                if segment == &self.segments[0] && self.segments.len() == 1 {
                    self.translation =
                        match segment {
                            Segment::Empty => String::from(""),
                            Segment::User(color, bg_color, icon) => {
                                format!(
                                    "\\[\\e[38;2;{};{};{}m\\]\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]\\u\\[\\e[38;2;{};{};{};48;1m\\]\\[\\e[0m\\]",
                                    bg_color[0],
                                    bg_color[1],
                                    bg_color[2],
                                    color[0],
                                    color[1],
                                    color[2],
                                    bg_color[0],
                                    bg_color[1],
                                    bg_color[2],
                                    bg_color[0],
                                    bg_color[1],
                                    bg_color[2],
                                )
                            }
                            Segment::UserDevice(color, bg_color, icon, between) => format!(
                                "\\[\\e[38;2;{};{};{}m\\]\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]\\u{}\\h\\[\\e[38;2;{};{};{};48;1m\\]\\[\\e[0m\\]",
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                                color[0],
                                color[1],
                                color[2],
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                                between,
                                    bg_color[0],
                                    bg_color[1],
                                    bg_color[2],
                            ),
                            Segment::Battery(color, bg_color, icon) => {
                                format!(
                            "\\[\\e[38;2;{};{};{}m\\]\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]\\$(battery)\\[\\e[38;2;{};{};{};48;1m\\]\\[\\e[0m\\]",
                            bg_color[0], bg_color[1], bg_color[2], color[0],
                                color[1],
                                color[2],
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                                    bg_color[0],
                                    bg_color[1],
                                    bg_color[2],
                        )
                            }
                            Segment::Network(color, bg_color, icon) => {
                                format!(
                            "\\[\\e[38;2;{};{};{}m\\]\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]\\$(network)\\[\\e[38;2;{};{};{};48;1m\\]\\[\\e[0m\\]",
                            bg_color[0], bg_color[1], bg_color[2],color[0],
                                color[1],
                                color[2],
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                                    bg_color[0],
                                    bg_color[1],
                                    bg_color[2],
                        )
                            }
                            Segment::Time(color, bg_color, icon, strftime) => format!(
                                "\\[\\e[38;2;{};{};{}m\\]\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]\\D{{{}}}\\[\\e[38;2;{};{};{};48;1m\\]\\[\\e[0m\\]",
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                                color[0],
                                color[1],
                                color[2],
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                                strftime,
                                    bg_color[0],
                                    bg_color[1],
                                    bg_color[2],
                            ),
                            Segment::Custom(color, bg_color, icon, custom) => format!(
                                "\\[\\e[38;2;{};{};{}m\\]\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]{}\\[\\e[38;2;{};{};{};48;1m\\]\\[\\e[0m\\]",
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                                color[0],
                                color[1],
                                color[2],
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                                custom,
                                    bg_color[0],
                                    bg_color[1],
                                    bg_color[2],
                            ),
                        };
                } else if segment == &self.segments[0] {
                    self.translation =
                        match segment {
                            Segment::Empty => String::from(""),
                            Segment::User(color, bg_color, icon) => {
                                format!(
                                    "\\[\\e[38;2;{};{};{}m\\]\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]\\u",
                                    bg_color[0],
                                    bg_color[1],
                                    bg_color[2],
                                    color[0],
                                    color[1],
                                    color[2],
                                    bg_color[0],
                                    bg_color[1],
                                    bg_color[2],
                                )
                            }
                            Segment::UserDevice(color, bg_color, icon, between) => format!(
                                "\\[\\e[38;2;{};{};{}m\\]\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]\\u{}\\h",
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                                color[0],
                                color[1],
                                color[2],
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                                between,
                            ),
                            Segment::Battery(color, bg_color, icon) => {
                                format!(
                            "\\[\\e[38;2;{};{};{}m\\]\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]\\$(battery)",
                            bg_color[0], bg_color[1], bg_color[2], color[0],
                                color[1],
                                color[2],
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                        )
                            }
                            Segment::Network(color, bg_color, icon) => {
                                format!(
                            "\\[\\e[38;2;{};{};{}m\\]\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]\\$(network)",
                            bg_color[0], bg_color[1], bg_color[2],color[0],
                                color[1],
                                color[2],
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                        )
                            }
                            Segment::Time(color, bg_color, icon, strftime) => format!(
                                "\\[\\e[38;2;{};{};{}m\\]\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]\\D{{{}}}",
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                                color[0],
                                color[1],
                                color[2],
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                                strftime,
                            ),
                            Segment::Custom(color, bg_color, icon, custom) => format!(
                                "\\[\\e[38;2;{};{};{}m\\]\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]{}",
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                                color[0],
                                color[1],
                                color[2],
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                                custom,
                            ),
                        };
                } else if segment == &self.segments[self.segments.len() - 1] {
                    self.translation =
                        match segment {
                            Segment::Empty => String::from(""),
                            Segment::User(color, bg_color, icon) => format!(
                                "\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]󰍟\\[\\e[38;2;{};{};{}m\\]\\u\\[\\e[38;2;{};{};{};48;1m\\]\\[\\e[0m\\] ",
                                &prev_color[0],
                                &prev_color[1],
                                &prev_color[2],
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                                color[0],
                                color[1],
                                color[2],
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                            ),
                            Segment::UserDevice(color, bg_color, icon, between) => format!(
                                "\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]󰍟\\[\\e[38;2;{};{};{}m\\]\\u{}\\h\\[\\e[38;2;{};{};{};48;1m\\]\\[\\e[0m\\] ",
                                &prev_color[0],
                                &prev_color[1],
                                &prev_color[2],
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                                color[0],
                                color[1],
                                color[2],
                                between,
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                            ),
                            Segment::Battery(color, bg_color, icon) => format!(
                                "\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]󰍟\\[\\e[38;2;{};{};{}m\\]\\$(battery)\\[\\e[38;2;{};{};{};48;1m\\]\\[\\e[0m\\] ",
                                &prev_color[0],
                                &prev_color[1],
                                &prev_color[2],
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                                color[0],
                                color[1],
                                color[2],
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                        ),
                            Segment::Network(color, bg_color, icon) => format!(
                                "\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]󰍟\\[\\e[38;2;{};{};{}m\\]\\$(network)\\[\\e[38;2;{};{};{};48;1m\\]\\[\\e[0m\\] ",
                                &prev_color[0],
                                &prev_color[1],
                                &prev_color[2],
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                                color[0],
                                color[1],
                                color[2],
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                        ),
                            Segment::Time(color, bg_color, icon, strftime) => format!(
                                "\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]󰍟\\[\\e[38;2;{};{};{}m\\]\\D{{{}}}\\[\\e[38;2;{};{};{};48;1m\\]\\[\\e[0m\\] ",
                                &prev_color[0],
                                &prev_color[1],
                                &prev_color[2],
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                                color[0],
                                color[1],
                                color[2],
                                strftime,
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                            ),
                            Segment::Custom(color, bg_color, icon, custom) => format!(
                                "\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]󰍟\\[\\e[38;2;{};{};{}m\\]{}\\[\\e[38;2;{};{};{};48;1m\\]\\[\\e[0m\\] ",
                                &prev_color[0],
                                &prev_color[1],
                                &prev_color[2],
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                                color[0],
                                color[1],
                                color[2],
                                custom,
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                            ),
                        };
                } else {
                    self.translation = match segment {
                        Segment::Empty => String::from(""),
                        Segment::User(color, bg_color, icon) => format!(
                            "\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]󰍟\\[\\e[38;2;{};{};{}m\\]\\u",
                            &prev_color[0],
                            &prev_color[1],
                            &prev_color[2],
                            bg_color[0], bg_color[1], bg_color[2], color[0], color[1], color[2]
                        ),
                        Segment::UserDevice(color, bg_color, icon, between) => format!(
                            "\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]󰍟\\[\\e[38;2;{};{};{}m\\]\\u{}\\h",
                            &prev_color[0],
                            &prev_color[1],
                            &prev_color[2],
                            bg_color[0],
                            bg_color[1],
                            bg_color[2],
                            color[0],
                            color[1],
                            color[2],
                            between
                        ),
                        Segment::Battery(color, bg_color, icon) => format!(
                            "\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]󰍟\\[\\e[38;2;{};{};{}m\\]\\$(battery)",
                            &prev_color[0],
                            &prev_color[1],
                            &prev_color[2],
                            bg_color[0],
                            bg_color[1],
                            bg_color[2],
                            color[0], color[1], color[2], 
                        ),
                        Segment::Network(color, bg_color, icon) => format!(
                            "\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]󰍟\\[\\e[38;2;{};{};{}m\\]\\$(network)",
                            &prev_color[0],
                            &prev_color[1],
                            &prev_color[2],
                            bg_color[0],
                            bg_color[1],
                            bg_color[2],
                            color[0], color[1], color[2], 
                        ),
                        Segment::Time(color, bg_color, icon, strftime) => format!(
                            "\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]󰍟\\[\\e[38;2;{};{};{}m\\]\\D{{{}}}",
                            &prev_color[0],
                            &prev_color[1],
                            &prev_color[2],
                            bg_color[0],
                            bg_color[1],
                            bg_color[2],
                            color[0],
                            color[1],
                            color[2],
                            strftime
                        ),
                        Segment::Custom(color, bg_color, icon, custom) => format!(
                            "\\[\\e[38;2;{};{};{};48;2;{};{};{}m\\]󰍟\\[\\e[38;2;{};{};{}m\\]{}",
                            &prev_color[0],
                            &prev_color[1],
                            &prev_color[2],
                            bg_color[0],
                            bg_color[1],
                            bg_color[2],
                            color[0],
                            color[1],
                            color[2],
                            custom
                        ),
                    };
                }
                self.full.push_str(self.translation.as_str());
                prev_color = match segment {
                    Segment::Empty => egui::Color32::BLACK,
                    Segment::User(_, prev_color, _) => egui::Color32::from_rgb(prev_color[0], prev_color[1], prev_color[2]),
                    Segment::UserDevice(_, prev_color, _, _) => egui::Color32::from_rgb(prev_color[0], prev_color[1], prev_color[2]),
                    Segment::Battery(_, prev_color, _) => egui::Color32::from_rgb(prev_color[0], prev_color[1], prev_color[2]),
                    Segment::Network(_, prev_color, _) => egui::Color32::from_rgb(prev_color[0], prev_color[1], prev_color[2]),
                    Segment::Time(_, prev_color, _, _) => egui::Color32::from_rgb(prev_color[0], prev_color[1], prev_color[2]),
                    Segment::Custom(_, prev_color, _, _) => egui::Color32::from_rgb(prev_color[0], prev_color[1], prev_color[2]),
                };
            }
            self.full.push('"');
            egui_dnd::dnd(ui, "preview").show_vec(&mut self.segments, |ui, item, handle, state| {
                ui.horizontal(|ui| {
                    handle.ui(ui, |ui| {
                        if state.dragged {
                            ui.label("dragging");
                        } else {
                            ui.label("drag");
                        }
                    });
                    ui.label(colors_text(&item).2);
                });
            });
            ui.label("Preview:");
            ui.label(self.preview.clone());
            ui.label("Paste the following into your ~/.bashrc");
            ui.code(self.full.clone());
        });
    }
}

fn main() {
    eframe::run_native(
        "Place Segments On",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(PlaceSegmentsOn::new(cc)))),
    )
    .unwrap();
}

fn colors_text(segment: &Segment) -> (egui::Color32, egui::Color32, String) {
    match segment {
        Segment::Empty => (egui::Color32::BLACK, egui::Color32::WHITE, String::new()),
        Segment::User(color, bg_color, icon) => {
            let user = match users::get_current_username() {
                Some(uname) => uname,
                None => std::ffi::OsString::new(),
            };
            (
                egui::Color32::from_rgb(color[0], color[1], color[2]),
                egui::Color32::from_rgb(bg_color[0], bg_color[1], bg_color[2]),
                format!("{:?}", user),
            )
        }
        Segment::UserDevice(color, bg_color, icon, between) => {
            let user2 = match users::get_current_username() {
                Some(uname) => uname,
                None => std::ffi::OsString::new(),
            };
            let hostname = gethostname::gethostname();
            (
                egui::Color32::from_rgb(color[0], color[1], color[2]),
                egui::Color32::from_rgb(bg_color[0], bg_color[1], bg_color[2]),
                format!("{:?}{}{:?}", user2, between, hostname),
            )
        }
        Segment::Battery(color, bg_color, icon) => (
            egui::Color32::from_rgb(color[0], color[1], color[2]),
            egui::Color32::from_rgb(bg_color[0], bg_color[1], bg_color[2]),
            match battery::Manager::new().unwrap().batteries().unwrap().next() {
                None => String::from("No battery"),
                Some(battery) => {
                    let mut percentage = ((format!("{:?}", battery.unwrap().state_of_charge())
                        .parse::<f32>()
                        .unwrap()
                        * 100.0) as u8)
                        .to_string();
                    percentage.push('%');
                    percentage
                }
            },
        ),
        Segment::Network(color, bg_color, icon) => (
            egui::Color32::from_rgb(color[0], color[1], color[2]),
            egui::Color32::from_rgb(bg_color[0], bg_color[1], bg_color[2]),
            String::from("Your Network"),
        ),
        Segment::Time(color, bg_color, icon, strftime) => (
            egui::Color32::from_rgb(color[0], color[1], color[2]),
            egui::Color32::from_rgb(bg_color[0], bg_color[1], bg_color[2]),
            format!("{}", chrono::offset::Local::now().format(strftime)),
        ),
        Segment::Custom(color, bg_color, icon, text) => (
            egui::Color32::from_rgb(color[0], color[1], color[2]),
            egui::Color32::from_rgb(bg_color[0], bg_color[1], bg_color[2]),
            String::from(text.as_str()), // turning the &String into a String
        ),
    }
}
