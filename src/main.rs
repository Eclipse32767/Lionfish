use iced::{Alignment, Length, Result, Sandbox, Settings};
use iced::widget::{Button, Row, Column, Container, Image, Text, Space};
use std::env;
use rfd::FileDialog;

fn main() -> Result {
    Lionfish::run(Settings::default())
}

struct Lionfish {
    img_path: Option<String>
}

#[derive(Debug, Clone)]
enum Message {
    OpenFile,
}

impl Sandbox for Lionfish {
    type Message = Message;
    fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        println!("{}", args.len());

        if args.len() >= 1 {
            println!("{}", args[1]);
            Lionfish{img_path: Some(args[1].clone())}
        } else {
            println!("{:#?}", args);
            Lionfish{img_path: None}
        }
    }
    fn title(&self) -> String {
        String::from("Lionfish Image Viewer")
    }
    fn update(&mut self, message: Self::Message) {
        match message {
            Message::OpenFile => {
                match FileDialog::new().pick_file() {
                    Some(image) => self.img_path = Some(image.to_string_lossy().to_string()),
                    None => self.img_path = None
                }
            }
        }
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        let open = Button::new("Open").on_press(Message::OpenFile);
        let open_row = Row::new().push(open).align_items(Alignment::Start);
        let mut col = Column::new().push(open_row).push(Space::with_height(Length::Fill));
        match self.img_path.clone() {
            Some(path) => {
                let display = Image::new(path);
                col = col.push(display);
            }
            None => {
                let display = Text::new("No Image is Selected!");
                println!("test");
                col = col.push(display);
            }
        }
        col = col.push(Space::with_height(Length::Fill));
        Container::new(col).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill).into()
    }
}