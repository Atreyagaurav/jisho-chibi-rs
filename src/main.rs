use dioxus::{events::*, prelude::*};
use jisho;

fn main() {
    dioxus::desktop::launch_cfg(APP, |c| {
        c.with_window(|w| {
            w.with_title("Jisho Chibi")
                .with_resizable(true)
                .with_inner_size(dioxus::desktop::wry::application::dpi::LogicalSize::new(
                    220.0, 220.0,
                ))
        })
    });
}

static APP: Component<()> = |cx| {
    let search_result = use_ref(&cx, || JishoSearch::new());
    let (search_input, set_search_input) = use_state(&cx, || "".to_string());
    let (message, set_message) = use_state(&cx, || "Search to start".to_string());

    let change_evt = move |evt: KeyboardEvent| match evt.key.as_str() {
        "ArrowDown" => {
            if !search_result.write().next() {
                set_message("No Next Entry".to_string())
            } else {
                set_message(format!(
                    "Showing: {} of {}",
                    search_result.read().curr + 1,
                    search_result.read().entries.len()
                ))
            }
        }
        "ArrowUp" => {
            if !search_result.write().prev() {
                set_message("No Previous Entry".to_string())
            } else {
                set_message(format!(
                    "Showing: {} of {}",
                    search_result.read().curr + 1,
                    search_result.read().entries.len()
                ))
            }
        }
        _ => (),
    };

    rsx!(cx, div {
    onkeydown: change_evt,
        link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet" }
        style { [include_str!("./style.css")] }
        header {
        // for future make it sync with clipboard
            // i { class: "material-icons", "sync" }
        input {
            "type": "text",
            value: "{search_input}",
            style: "width:190px",
            placeholder: "Search",
            oninput: move |evt| set_search_input(evt.value.clone()),
            onkeydown: move |evt| {
                if evt.key == "Enter" {
                    set_message(format!("Searching for {}", search_input));
            cx.spawn({
            let search_result = search_result.to_owned();
            let search_input = search_input.to_owned();
            let set_message = set_message.to_owned();
            async move {
            search_result.write().search(&search_input);
            if search_result.read().entries.len() == 0 {
            set_message("No Results".to_string())
            }else{
                    set_message(format!("Showing: {} of {}", search_result.read().curr + 1,
                    search_result.read().entries.len()))
            }}});
                }
            },
        }
       span {
        i { class: "material-icons", onclick: move |_| search_result.write().search(search_input), "search" }
           i { class: "material-icons", onclick: move |_| {if !search_result.write().prev(){set_message("No Previous Entry".to_string())}}, "chevron_left" }
           i { class: "material-icons", onclick: move |_| {if !search_result.write().next(){set_message("No Next Entry".to_string())}}, "chevron_right" }
        }
        }

        main {
        search_result.read().current().map(|entry| {
                rsx! (
                    div{
            if entry.kanji != "" {
            rsx!(ruby { "{entry.kanji}"
                 rp {"("}
                 rt {"{entry.reading}"}
                 rp {")"}
            })} else {
                rsx!("{entry.reading}")
            }
            entry.meanings.iter().map(|m| rsx!(li{"{m};"}))
                    }
                )
        })
    }

    footer{
        "{message}"
    }
    })
};

struct JishoSearch {
    entries: Vec<jisho::Entry>,
    curr: usize,
    link: String,
}

impl JishoSearch {
    fn new() -> Self {
        Self {
            entries: vec![jisho::Entry {
                kanji: "Kanji".to_string(),
                reading: "Reading".to_string(),
                meanings: vec!["Meanings;...".to_string()],
                frequency: 0,
            }],
            curr: 0,
            link: "".to_string(),
        }
    }

    fn search(&mut self, s: &str) {
        if s.trim() == "" {
            return;
        }
        println!("Searching: {}", s);
        self.entries = jisho::lookup(s)
            .iter()
            .map(|e| (*e).clone())
            .collect::<Vec<jisho::Entry>>();
        self.curr = 0;
        self.link = format!("https://jisho.org/search/{}", s);
    }

    fn prev(&mut self) -> bool {
        if self.curr == 0 {
            false
        } else {
            self.curr -= 1;
            true
        }
    }

    fn next(&mut self) -> bool {
        if self.curr >= self.entries.len() - 1 {
            false
        } else {
            self.curr += 1;
            true
        }
    }

    fn current(&self) -> Option<&jisho::Entry> {
        self.entries.get(self.curr)
    }
}
