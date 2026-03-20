use iced::clipboard::read_primary;
use iced::time::{self, Duration};
use iced::widget::{
    button, column, container, row, scrollable,
    text::{Rich, Span},
    text_input, tooltip,
};
use iced::window;
use iced::{theme::Theme, Alignment, Element, Settings, Subscription, Task};
use reqwest;
use serde_json::Value;
use std::collections::HashMap;
mod colors;

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.default_text_size = 12.into();

    let mut window = window::Settings::default();
    window.size = iced::Size::new(300.0, 200.0);
    iced::application(JishoChibi::new, JishoChibi::update, JishoChibi::view)
        .settings(settings)
        .window(window)
        .font(include_bytes!("../fonts/ipag.ttf"))
        .title(JishoChibi::title)
        .theme(JishoChibi::theme)
        .subscription(JishoChibi::subscription)
        .run()
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
    fn view(&self, curr: bool) -> Rich<'_, String, Message> {
        let mut spans = vec![
            Span::new(" * ").size(14),
            Span::new(&self.word)
                .color(if curr {
                    colors::WORD
                } else {
                    colors::WORD_DERIVATIVE
                })
                .size(14),
            Span::new(" (").size(14),
            Span::new(&self.reading).color(colors::READING).size(16),
            Span::new(")\n").size(14),
        ];
        self.meanings.iter().enumerate().for_each(|(i, m)| {
            spans.push(Span::new(format!("  {:2}. ", i + 1)));
            if let [a, rest @ ..] = m.tags.as_slice() {
                spans.push(Span::new("["));
                spans.push(Span::new(a).color(colors::TAGS));
                for t in rest {
                    spans.push(Span::new("; "));
                    spans.push(Span::new(t).color(colors::TAGS));
                }
                spans.push(Span::new("] "));
            }
            spans.push(Span::new(&m.meaning).color(colors::MEANING));
            spans.push(Span::new("\n"));
        });
        Rich::with_spans(spans).font(iced::font::Font::MONOSPACE)
    }

    #[allow(unused)]
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

#[derive(Default)]
struct JishoChibi {
    watching: bool,
    current_word: String,
    meanings: Vec<JishoWord>,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    SearchPressed,
    WatchMode,
    CheckClipboard,
    ClipChanged(String),
    // NextClicked,
    // PreviousClicked,
}

impl JishoChibi {
    fn new() -> Self {
        Self {
            watching: false,
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

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SearchPressed => {
                if !self.current_word.trim().is_empty() {
                    self.meanings = search(&self.current_word).unwrap();
                }
            }
            Message::InputChanged(inp) => {
                self.current_word = inp;
            }
            Message::ClipChanged(inp) => {
                if inp != self.current_word {
                    self.current_word = inp;
                    return Task::done(Message::SearchPressed);
                }
            }
            Message::WatchMode => {
                self.watching = !self.watching;
            }
            Message::CheckClipboard if self.watching => {
                return read_primary().then(|r| match r {
                    Some(txt) => return Task::perform(async { txt }, Message::ClipChanged),
                    _ => Task::none(),
                })
            }
            _ => (),
        }

        Task::none()
    }

    fn view(&'_ self) -> Element<'_, Message> {
        column![
            if self.watching {
                row![
                    tooltip(
                        button("X").on_press(Message::WatchMode),
                        "End Sync with Selection",
                        tooltip::Position::Top
                    )
                    .style(container::rounded_box),
                    text_input("Word", &self.current_word),
                ]
            } else {
                row![
                    tooltip(
                        button("A").on_press(Message::WatchMode),
                        "Sync from Selection",
                        tooltip::Position::Top
                    )
                    .style(container::rounded_box),
                    text_input("Word", &self.current_word)
                        .on_input(Message::InputChanged)
                        .on_submit(Message::SearchPressed),
                    button("Search").on_press(Message::SearchPressed),
                ]
            },
            scrollable(
                column(
                    self.meanings
                        .iter()
                        .map(|m| m.view(m.word == self.current_word).into())
                        .collect::<Vec<Element<_>>>()
                )
                .spacing(5)
                .padding(5)
            )
        ]
        .padding(1)
        .align_x(Alignment::Center)
        .width(300)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(600)).map(|_| Message::CheckClipboard)
    }
}
