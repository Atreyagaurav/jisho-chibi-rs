use iced::widget::{button, column, row, text, text_input};
use iced::{Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    JishoChibi::run(Settings::default())
}

struct JishoMeaning {
    meaning: String,
    tags: Vec<String>,
}

struct JishoWord {
    word: String,
    reading: String,
    meanings: Vec<JishoMeaning>,
}

impl JishoWord {
    fn to_text(&self) -> String {
        let mut text = String::new();
        text.push_str(&self.word);
        text.push_str("\n");
        self.meanings.iter().for_each(|m| {
            text.push_str(&format!("\t- {}\n", m.meaning));
        });
        text
    }
}

struct JishoChibi {
    primary_clipboard: bool,
    current_word: String,
    meanings: Vec<JishoWord>,
    history: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    SearchPressed,
    NextClicked,
    PreviousClicked,
    SyncClicked,
}

impl Sandbox for JishoChibi {
    type Message = Message;

    fn new() -> Self {
        Self {
            primary_clipboard: false,
            current_word: "".into(),
            meanings: vec![JishoWord {
                word: "Search".into(),
                reading: "Reading".into(),
                meanings: vec![
                    JishoMeaning {
                        meaning: "Meaning 1".into(),
                        tags: vec![],
                    },
                    JishoMeaning {
                        meaning: "Meaning 2".into(),
                        tags: vec![],
                    },
                ],
            }],
            history: vec![],
        }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SearchPressed => {
                self.current_word = "Hi".to_string();
            }
            Message::InputChanged(inp) => {
                self.current_word = inp;
            }
            _ => (),
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![
                text_input("Word", &self.current_word)
                    .on_input(Message::InputChanged)
                    .on_submit(Message::SearchPressed),
                button("Search").on_press(Message::SearchPressed),
            ],
            column(
                self.meanings
                    .iter()
                    .map(|m| text(&m.to_text()).into())
                    .collect()
            )
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
