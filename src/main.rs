use iced::{Application, Settings, Command, Clipboard, executor, 
    Element, Button, button, TextInput, text_input, Column, Text
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

#[derive(Default)]
struct SendToWebhook {
    client: Client,
    url: text_input::State,
    url_value: String,
    message: text_input::State,
    message_value: String,

    send_button: button::State,
}

#[derive(Debug, Clone)]
pub enum UserInput {
    SendPressed,
    RequestSent,
    URLChanged(String),
    MessageChanged(String),
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
        Column::new()
            .push(TextInput::new(&mut self.url, "Webhook URL", &mut self.url_value, UserInput::URLChanged))
            .push(TextInput::new(&mut self.message, "Message", &mut self.message_value, UserInput::MessageChanged))
            .push(Button::new(&mut self.send_button, Text::new("Send")).on_press(UserInput::SendPressed))
            .into()
    }

    fn update(&mut self, input: UserInput, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        match input {
            UserInput::SendPressed => {
                let client = &self.client;
                let json = serde_json::json!({
                    "content": &self.message_value,
                });
                
                // Basically for testing
                // println!("{} {} {}", self.url_value, self.message_value, json);

                Command::perform(client.post(&self.url_value).json(&json).send(), |_response| {
                    // In the future, we need to retrieve the status code from _response.
                    // Currently (4/20/2021), my limited knowledge of Rust and the libraries involved
                    // makes the resulting errors impossible to solve.
                    UserInput::RequestSent
                })
            },
            UserInput::RequestSent => {Command::none()},
            UserInput::URLChanged(value) => {self.url_value = value; Command::none()},
            UserInput::MessageChanged(value) => {self.message_value = value; Command::none()},
        }
    }
}