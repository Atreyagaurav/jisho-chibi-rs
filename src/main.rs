use std::collections::HashMap;

use iced::widget::{button, column, row, scrollable, text, text_input};
use iced::{Alignment, Element, Sandbox, Settings};
use reqwest;
use serde_json::Value;

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (300, 200);
    settings.window.resizable = true;
    settings.default_text_size = 12.0;
    settings.default_font = Some(include_bytes!("../fonts/ipag.ttf"));
    JishoChibi::run(settings.clone())
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
        text.push_str(&format!("* {} ({})\n", self.word, self.reading));
        self.meanings.iter().enumerate().for_each(|(i, m)| {
            text.push_str(&format!(
                "  {:2}. {} {}\n",
                i + 1,
                if m.tags.len() > 0 {
                    format!("[{}]", m.tags.join(";"))
                } else {
                    "".into()
                },
                m.meaning
            ));
        });
        text
    }
}

fn search(text: &str) -> Result<Vec<JishoWord>, reqwest::Error> {
    let res: HashMap<String, Value> = reqwest::blocking::get(format!(
        "http://beta.jisho.org/api/v1/search/words?keyword={}",
        text.trim()
    ))?
    .json()?;
    let result: Vec<JishoWord> = res["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|d| {
            let wrd = d.get("japanese").unwrap();
            let word: String = wrd[0]
                .get("word")
                .map(|w| w.as_str())
                .flatten()
                .unwrap_or("")
                .to_string();
            let reading: String = wrd[0]
                .get("reading")
                .map(|w| w.as_str())
                .flatten()
                .unwrap_or("")
                .to_string();
            let meanings = d
                .get("senses")
                .map(|s| s.as_array())
                .flatten()
                .map(|ss| {
                    ss.iter()
                        .map(|s| {
                            let meaning = s
                                .get("english_definitions")
                                .unwrap()
                                .as_array()
                                .unwrap()
                                .iter()
                                .filter_map(|m| m.as_str())
                                .collect::<Vec<&str>>()
                                .join(", ");
                            let tags = s
                                .get("parts_of_speech")
                                .unwrap()
                                .as_array()
                                .unwrap()
                                .iter()
                                .filter_map(|m| m.as_str().map(|s| s.to_string()))
                                .collect();

                            JishoMeaning { meaning, tags }
                        })
                        .collect()
                })
                .unwrap_or(vec![]);
            JishoWord {
                word,
                reading,
                meanings,
            }
        })
        .collect();
    Ok(result)
}

struct JishoChibi {
    current_word: String,
    meanings: Vec<JishoWord>,
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
            current_word: "".into(),
            meanings: vec![JishoWord {
                word: "Search 分".into(),
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
        }
    }

    fn title(&self) -> String {
        String::from("Jisho Chibi")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SearchPressed => {
                if !self.current_word.trim().is_empty() {
                    self.meanings = search(&self.current_word).unwrap();
                }
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
            scrollable(
                column(
                    self.meanings
                        .iter()
                        .map(|m| text(&m.to_text()).into())
                        .collect()
                )
                .padding(5)
            )
        ]
        .padding(1)
        .align_items(Alignment::Center)
        .width(300)
        .into()
    }
}
