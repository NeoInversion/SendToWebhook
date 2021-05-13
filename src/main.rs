use iced::{Application, Settings, Command, Clipboard, executor, 
    Element, Button, button, TextInput, text_input, Column, Text,
    Container, Length, Align
};
use reqwest::{Client};

#[tokio::main]
async fn main() -> iced::Result {
    SendToWebhook::run(Settings {
        window: iced::window::Settings {
            size: (400, 200),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug, Clone)]
pub enum UserInput {
    SendPressed,
    RequestSent,
    URLChanged(String),
    MessageChanged(String),
}

#[derive(Default)]
struct SendToWebhook {
    client: Client,
    url: text_input::State,
    url_value: String,
    message: text_input::State,
    message_value: String,

    send_button: button::State,
}

impl Application for SendToWebhook {
    type Executor = executor::Default;
    type Message = UserInput;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (SendToWebhook::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("SendToWebhook")
    }

    fn view(&mut self) -> Element<Self::Message> {
        let ui = Column::new()
                .push(TextInput::new(&mut self.url, "Webhook URL", &mut self.url_value, UserInput::URLChanged).style(style::TextInput))
                .spacing(5)
                .push(TextInput::new(&mut self.message, "Message", &mut self.message_value, UserInput::MessageChanged).style(style::TextInput))
                .spacing(10)
                .align_items(Align::Center)
                .push(Button::new(&mut self.send_button, Text::new("Send")).on_press(UserInput::SendPressed).style(style::Button));
        
        Container::new(ui)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            // .center_y()
            .style(style::Container)
            .into()
    }

    fn update(&mut self, input: UserInput, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        match input {
            UserInput::SendPressed => {
                let client = &self.client;
                let json = serde_json::json!({
                    "content": &self.message_value,
                });
                
                // Debug print
                // println!("{} {} {}", self.url_value, self.message_value, json);

                Command::perform(client.post(&self.url_value).json(&json).send(), |_response| {
                    // TODO: Get the return status code
                    UserInput::RequestSent
                })
            },
            UserInput::RequestSent => {Command::none()},
            UserInput::URLChanged(value) => {self.url_value = value; Command::none()},
            UserInput::MessageChanged(value) => {self.message_value = value; Command::none()},
        }
    }
}

mod style {
    use iced::{Color, text_input, button, container};

    // iced's Styling example looks good; I'll take colors from that for now
    const SURFACE: Color = Color::from_rgb(
        0x40 as f32 / 255.0,
        0x44 as f32 / 255.0,
        0x4B as f32 / 255.0,
    );

    const ACCENT: Color = Color::from_rgb(
        0x6F as f32 / 255.0,
        0xFF as f32 / 255.0,
        0xE9 as f32 / 255.0,
    );

    const ACTIVE: Color = Color::from_rgb(
        0x45 as f32 / 255.0,
        0x4B as f32 / 255.0,
        0x54 as f32 / 255.0,
    );

    const HOVERED: Color = Color::from_rgb(
        0x45 as f32 / 255.0,
        0x4B as f32 / 255.0,
        0x54 as f32 / 255.0,
    );

    pub struct Container;
    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Color::from_rgb8(0x36, 0x39, 0x3F).into(),
                text_color: Color::WHITE.into(),
                ..container::Style::default()
            }
        }
    }

    pub struct Button;
    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: ACTIVE.into(),
                border_radius: 3.0,
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                background: HOVERED.into(),
                text_color: Color::WHITE,
                ..self.active()
            }
        }

        fn pressed(&self) -> button::Style {
            button::Style {
                border_width: 1.0,
                border_color: Color::WHITE,
                ..self.hovered()
            }
        }
    }

    pub struct TextInput;
    impl text_input::StyleSheet for TextInput {
        fn active(&self) -> text_input::Style {
            text_input::Style {
                background: SURFACE.into(),
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            }
        }

        fn focused(&self) -> text_input::Style {
            text_input::Style {
                border_width: 1.0,
                border_color: ACCENT,
                ..self.active()
            }
        }

        fn hovered(&self) -> text_input::Style {
            text_input::Style {
                border_width: 1.0,
                border_color: Color { a: 0.3, ..ACCENT },
                ..self.focused()
            }
        }

        fn placeholder_color(&self) -> Color {
            return Color::from_rgb(
                200 as f32 / 255.0,
                200 as f32/ 255.0,
                200 as f32 / 255.0,
            );
        }

        fn value_color(&self) -> Color {
            Color::WHITE
        }

        fn selection_color(&self) -> Color {
            ACTIVE
        }
    }
}