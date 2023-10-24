use iced::{Alignment, Application, executor, Length, Result, Settings, Subscription};
use iced::widget::{Button, Row, Column, Container, Image, Text, Space};
use std::env;
use iced::keyboard::{Event as kbevent, KeyCode};
use iced::window::Event as winevent;
use iced_style::Theme;
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
    KeyboardUpdate(kbevent),
    WindowUpdate(winevent)
}

impl Application for Lionfish {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: () ) -> (Lionfish, iced::Command<Message>) {
        let args: Vec<String> = env::args().collect();
        println!("{}", args.len());

        (if args.len() >= 1 {
            println!("{}", args[1]);
            Lionfish{img_path: Some(args[1].clone())}
        } else {
            println!("{:#?}", args);
            Lionfish{img_path: None}
        }, iced::Command::none())
    }
    fn title(&self) -> String {
        String::from("Lionfish Image Viewer")
    }
    fn update(&mut self, message: Self::Message) -> iced::Command<Message> {
        match message {
            Message::OpenFile => {
                match FileDialog::new().pick_file() {
                    Some(image) => self.img_path = Some(image.to_string_lossy().to_string()),
                    None => self.img_path = None
                }
            }
            Message::KeyboardUpdate(event) => {
                match event {
                    kbevent::KeyPressed { key_code, modifiers} => {
                        if key_code == KeyCode::Enter {
                            match FileDialog::new().pick_file() {
                                Some(image) => self.img_path = Some(image.to_string_lossy().to_string()),
                                None => self.img_path = None
                            }
                        }
                    }
                    kbevent::KeyReleased { .. } => {}
                    kbevent::CharacterReceived(_) => {}
                    kbevent::ModifiersChanged(_) => {}
                }
            }
            Message::WindowUpdate(event) => {
                match event {
                    winevent::Moved { .. } => {}
                    winevent::Resized { .. } => {}
                    winevent::RedrawRequested(_) => {}
                    winevent::CloseRequested => {}
                    winevent::Focused => {}
                    winevent::Unfocused => {}
                    winevent::FileHovered(_) => {}
                    winevent::FileDropped(_) => {}
                    winevent::FilesHoveredLeft => {}
                }
            }
        }
        iced::Command::none()
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
        Container::new(col).center_x().center_y().width(Length::Fill).height(Length::Fill).into()
    }
    fn subscription(&self) -> Subscription<Self::Message> {
        iced::subscription::events_with(
            |event, _| {
                if let iced::Event::Keyboard(keyboard_event) = event {
                    Some(Message::KeyboardUpdate(keyboard_event))
                } else if let iced::Event::Window(window_event) = event {
                    Some(Message::WindowUpdate(window_event))
                } else {
                    None
                }
            }
        )
    }
}