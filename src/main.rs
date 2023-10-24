use iced::{Alignment, Application, Color, executor, Length, Result, Settings, Subscription};
use iced::widget::{Button, Row, Column, Container, Image, Text, Space};
use std::env;
use std::fmt::Debug;
use iced::keyboard::{Event as kbevent, KeyCode};
use iced::window::Event as winevent;
use iced_style::{Theme, theme};
use rfd::FileDialog;
use crate::lib_style::{ButtonStyle, get_set_theme, ListStyle, make_custom_theme, MenuStyle, SelectedTheme, ThemeCustom, ThemeSet};

mod lib_style;

fn main() -> Result {
    Lionfish::run(Settings::default())
}

struct Lionfish {
    img_path: Option<String>,
    theme_set: ThemeSet,
    selected_theme: SelectedTheme,
}

#[derive(Debug, Clone)]
enum Message {
    OpenFile,
    KeyboardUpdate(kbevent),
    WindowUpdate(winevent)
}

impl Default for Lionfish {
    fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        println!("{}", args.len());
        let image = if args.len() >= 1 {
            println!("{}", args[1]);
            Some(args[1].clone())
        } else {
            println!("{:#?}", args);
            None
        };
        Lionfish {
            img_path: image,
            theme_set: ThemeSet {
                light: ThemeCustom {
                    application: theme::Palette {
                        background: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        text: Color::from_rgb8(0x00, 0x19, 0x36),
                        primary: Color::from_rgb8(0x00, 0x77, 0xFF),
                        success: Color::from_rgb8(0x00, 0xCB, 0x40),
                        danger: Color::from_rgb8(0xFF, 0x4C, 0x00),
                    },
                    sidebar: ButtonStyle {
                        border_radius: 2.0,
                        txt_color: Color::from_rgb8(0x00, 0x19, 0x36),
                        bg_color: Color::from_rgb8(0xD2, 0xF0, 0xFF),
                        border_color: Color::from_rgb8(0, 0, 0),
                        border_width: 0.0,
                        shadow_offset: iced::Vector { x: 0.0, y: 0.0 }
                    },
                    secondary: ButtonStyle {
                        border_radius: 2.0,
                        txt_color: Color::from_rgb8(0x00, 0x20, 0x46),
                        bg_color: Color::from_rgb8(0xC6, 0xEC, 0xFF),
                        border_color: Color::from_rgb8(0, 0, 0),
                        border_width: 0.0,
                        shadow_offset: iced::Vector { x: 0.0, y: 0.0 }
                    },
                    list: ListStyle {
                        txt_color: Color::from_rgb8(0x00, 0x19, 0x36),
                        bg_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        handle_color: Color::from_rgb8(0x00, 0x19, 0x36),
                        border_radius: 5.0,
                        border_width: 2.0,
                        border_color: Color::from_rgb8(0x00, 0x19, 0x36),
                        menu: MenuStyle {
                            txt_color: Color::from_rgb8(0x00, 0x19, 0x36),
                            bg_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                            border_radius: 5.0,
                            border_width: 2.0,
                            border_color: Color::from_rgb8(0x00, 0x19, 0x36),
                            sel_txt_color: Color::from_rgb8(0x00, 0x19, 0x36),
                            sel_bg_color: Color::from_rgb8(0x00, 0xF1, 0xD6),
                        }
                    }
                },
                dark: ThemeCustom { // TODO: set dark theme properly
                    application: theme::Palette {
                        background: Color::from_rgb8(0x00, 0x19, 0x36),
                        text: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        primary: Color::from_rgb8(0x00, 0xAB, 0xE1),
                        success: Color::from_rgb8(0x00, 0xA9, 0x35),
                        danger: Color::from_rgb8(0xC5, 0x3A, 0x00),
                    },
                    sidebar: ButtonStyle {
                        border_radius: 2.0,
                        txt_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        bg_color: Color::from_rgb8(0x00, 0x20, 0x46),
                        border_color: Color::from_rgb8(0, 0, 0),
                        border_width: 0.0,
                        shadow_offset: iced::Vector { x: 0.0, y: 0.0 }
                    },
                    secondary: ButtonStyle {
                        border_radius: 2.0,
                        txt_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        bg_color: Color::from_rgb8(0x00, 0x29, 0x58),
                        border_color: Color::from_rgb8(0, 0, 0),
                        border_width: 0.0,
                        shadow_offset: iced::Vector { x: 0.0, y: 0.0 }
                    },
                    list: ListStyle {
                        txt_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        bg_color: Color::from_rgb8(0x00, 0x29, 0x58),
                        handle_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        border_radius: 5.0,
                        border_width: 2.0,
                        border_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        menu: MenuStyle {
                            txt_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                            bg_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                            border_radius: 5.0,
                            border_width: 2.0,
                            border_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                            sel_txt_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                            sel_bg_color: Color::from_rgb8(0x00, 0xCD, 0xB6),
                        }
                    }
                },
                custom: make_custom_theme()
            },
            selected_theme: get_set_theme()
        }
    }
}

impl Application for Lionfish {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: () ) -> (Lionfish, iced::Command<Message>) {
        (Lionfish::default(), iced::Command::none())
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
    fn theme(&self) -> Self::Theme {
        let colors = match self.selected_theme {
            SelectedTheme::Light => self.theme_set.light.application,
            SelectedTheme::Dark => self.theme_set.dark.application,
            SelectedTheme::Custom => self.theme_set.custom.application,
        };
        let custom = Theme::Custom(Box::new(theme::Custom::new(colors)));
        custom
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