use iced::{
    alignment,
    widget::{container, text},
    Element, Length, Sandbox, Settings,
};

extern crate iced;

#[derive(Debug, Clone)]
enum Feature {
    Battery,
    Network,
    Time(String),
    Custom(String),
}

#[derive(Debug, Clone)]
enum Message {
    Add(Feature),
    Remove(usize),
    Export(Vec<Feature>),
}

struct PlaceSegmentsOn {
    features: Vec<Feature>,
}

impl Sandbox for PlaceSegmentsOn {
    type Message = Message;

    fn new() -> PlaceSegmentsOn {
        Self { features: vec![] }
    }

    fn title(&self) -> String {
        String::from("Place Segments On")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Add(feature) => {
                self.features.push(feature);
            }
            Message::Remove(position) => {
                self.features.remove(position);
            }
            Message::Export(feature_list) => {
                export(feature_list);
            }
        };
    }

    fn view(&self) -> Element<Self::Message> {
        container(text("Place Segments On"))
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}

fn export(feature_list: Vec<Feature>) -> String {
    String::from("")
    //TODO: convert features to string
}

fn main() {
    PlaceSegmentsOn::run(Settings::default()).unwrap();
}
