#![allow(non_snake_case)]
use dioxus::html::a;
use dioxus::prelude::*;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

mod up_the_guts;

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    text: String,
    class: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Buffer {
    current_line: usize,
    previous_line: usize,
    lines: Vec<String>,
    display: Vec<Line>,
    current_char: Option<String>,
    page_name: String,
}

enum Charatures {
    Bruce,
    Stan,
    Roger,
    Martin,
    Lisa,
    Raeywn,
    Chastity,
    Andrea,
    Extra1,
    Extra2,
    Extra3,
}

impl Charatures {
    fn get_text(self) -> String {
        match self {
            Charatures::Andrea => String::from("Andrea"),
            Charatures::Bruce => String::from("Bruce"),
            Charatures::Chastity => String::from("Chastity"),
            Charatures::Lisa => String::from("Lise"),
            Charatures::Martin => String::from("Martin"),
            Charatures::Raeywn => String::from("Raeywn"),
            Charatures::Roger => String::from("Roger"),
            Charatures::Stan => String::from("Stan"),
            _ => String::from("Extra"),
        }
    }
}
//impl Deref for Buffer{
//
//    type Target = Buffer;

//   fn deref(&self) -> &Self::Target{
//       &self
//   }
//}

impl Buffer {
    fn new() -> Buffer {
        Buffer {
            current_line: 0,
            previous_line: 0,
            lines: Vec::new(),
            display: Vec::new(),
            current_char: None,
            page_name: String::new(),
        }
    }

    fn clear(&mut self) {
        self.display = Vec::new();
        self.current_line = 0;
        self.previous_line = 0;
    }

    fn add(&mut self, line: Line) {
        if self.display.len() > 5 {
            self.display.remove(0);
        }
        self.display.push(line);
    }

    fn set_page(&mut self, page: String) {
        self.page_name = page;
    }

    fn set_current_counter(&mut self, number: usize) {
        self.current_line = number;
    }

    fn set_previous_counter(&mut self, number: usize) {
        self.previous_line = number;
    }
    fn set_counter_from_page(&mut self, lines: &Vec<String>, number: usize) {
        if number < 25 {
            self.current_line = 0;
            return;
        }
        for each in 0..lines.len() {
            if lines[each].contains((&number.to_string())) {
                self.current_line = each;
                return;
            }
        }
    }

    fn next(&mut self) {
        let chars: [&str; 9] = [
            "BRUCE", "STAN", "ROGER", "MARTIN", "LISA", "RAEYWN", "CHASTITY", "ANDREA", "EXTRA",
        ];
        if self.current_char.is_none() {
            return;
        }
        let current_char = match &self.current_char {
            Some(name) => String::from(name),
            None => String::from(""),
        };

        for each in self.current_line..self.lines.len() {
            for c in chars {
                if self.lines[each].contains(c) {
                    if self.lines[each].contains(&current_char) {
                        self.set_current_counter(each);
                        self.add(Line {
                            text: String::from(&self.lines[self.previous_line]),
                            class: String::from("red"),
                        });
                        self.add(Line {
                            text: String::from(&self.lines[self.current_line]),
                            class: String::from("red"),
                        });
                    }
                    self.set_previous_counter(each);
                }
            }
        }
    }

    fn next_out(&mut self, lines: &Vec<String>, current_line: &usize) {
        println!("{:?}", self.display);
        let chars: [&str; 9] = [
            "BRUCE", "STAN", "ROGER", "MARTIN", "LISA", "RAEYWN", "CHASTITY", "ANDREA", "EXTRA",
        ];
        if self.current_char.is_none() {
            self.current_char = Some("ROGER".to_owned());
        }
        let current_char = match &self.current_char {
            Some(name) => String::from(name),
            None => String::from(""),
        };

        for each in *current_line..lines.len() {
            for c in chars {
                if lines[each].contains(c) {
                    if lines[each].contains(&current_char) {
                        self.set_current_counter(each);
                        self.add(Line {
                            text: String::from(&lines[self.previous_line]),
                            class: String::from("red"),
                        });
                        self.add(Line {
                            text: String::from(&lines[*current_line]),
                            class: String::from("red"),
                        });
                        return;
                    }
                    self.set_previous_counter(each);
                }
            }
        }
    }
}

fn get_current_number(lines: &Vec<String>, char: &str, current_line: &usize) -> usize {
    let mut prev_line: usize = 0;
    let next_line = *current_line + 1;

    let chars: [&str; 9] = [
        "BRUCE", "STAN", "ROGER", "MARTIN", "LISA", "RAEYWN", "CHASTITY", "ANDREA", "EXTRA",
    ];

    for each in next_line..lines.len() {
        for c in chars {
            if lines[each].contains(c) {
                if lines[each].contains(char) {
                    return each;
                }
                prev_line = each;
            }
        }
    }
    0
}

fn get_previous_number(lines: &Vec<String>, char: &str, current_line: &usize) -> Vec<usize> {
    let chars: [&str; 9] = [
        "BRUCE", "STAN", "ROGER", "MARTIN", "LISA", "RAEYWN", "CHASTITY", "ANDREA", "EXTRA",
    ];
    let re = Regex::new(r"^\*\w").unwrap();
    let mut counters: Vec<usize> = Vec::new();
    let mut counter = *current_line;
    if counter == 0 {
        return Vec::new();
    }
    counter = counter - 1;
    while counter > 0 {
        //for c in chars {
        match re.captures(&lines[counter]) {
            Some(_i) => counters.push(counter),
            None => {
                for c in chars {
                    if lines[counter].contains(c) {
                        counters.push(counter);
                        return counters;
                    }
                }
            }
        }
        //if lines[counter].contains(c) {

        //if lines[counter].contains("sd:") {
        //    counters.push(counter);
        //} else {
        //    counters.push(counter);
        //    return counters;
        //}
        //}
        //}

        counter -= 1;
    }
    Vec::new()
}

fn get_page(lines: &Vec<String>, current_line: &usize) -> String {
    let mut page = String::from("Page");
    let mut counter = *current_line;
    if counter == 0 {
        return String::from("Beginning");
    }
    counter = counter - 1;
    while counter > 0 {
        if lines[counter].contains("Page") {
            let page = lines[counter].replace("# ", "");
            return page;
        }

        counter -= 1;
    }
    String::from("Unknown Page")
}

fn go_back(lines: &Vec<String>, char: &str, current_line: &usize, mut repeat: usize) -> usize {
    let mut counter = *current_line;
    if counter == 0 {
        return 0;
    }
    counter -= 1;
    if repeat == 0 {
        repeat = 1;
    }
    while repeat > 0 {
        while counter > 0 {
            if lines[counter].contains(char) {
                return counter;
            }

            counter -= 1;
        }
        repeat -= 1;
    }
    0
}

fn load_file() -> Vec<String> {
    let mut s = up_the_guts::get_text();

    let lines: Vec<String> = s.split("\n").map(str::to_string).collect();

    lines
}

fn create_line(line: &str, color: &str) -> Line {
    let mut line_int = line.replace("*", "");
    line_int = line_int.replace("_", "");

    Line {
        text: String::from(line_int),
        class: String::from(color),
    }
}

struct Char {
    charature: String,
}

impl Char {
    fn new() -> Char {
        Char {
            charature: String::new(),
        }
    }

    fn set(&mut self, name: &str) {
        let mut int_name = name;
        //if int_name.contains(".") {
        self.charature = String::from(int_name);
        //} else {
        //    let mut thing = String::from(int_name);
        //    thing.push('.');
        //    self.charature = thing;
        //}
    }
}

fn App(cx: Scope) -> Element {
    let lines = load_file();
    let lines2 = lines.clone();
    let lines3 = lines.clone();
    let lines4 = lines.clone();
    let lines5 = lines.clone();
    let mut buffer = use_ref(cx, Buffer::new);
    let mut char = use_ref(cx, Char::new);

    move || {
        char.with_mut(|c| c.set("ROGER"));
    };
    cx.render(rsx! {
        p{ "Ver 0.1.3 Character - {char.read().charature.clone()} - {buffer.read().page_name.clone()}"}
        div {
            height:"55vh",
            overflow:"auto",
            buffer.read().display.iter().map(|lines| {
                rsx!{ p{
                        color: "{lines.class}",
                        "{lines.text}"
                }
                }
            })
            }
        div {
            position: "fixed",
            bottom: "0",
            left: "0",
            width: "100vh",
        button {
            padding: "10px",
            margin: "5px",
            style: "-webkit-appearance: none;
            -webkit-border-radius: 0;
            border-radius: 0; color: black; font-weight: bold;
            margin: 0.5vh; padding: 1vh; background-color: #d690d0;",
            onclick: move |_| {
                let count = buffer.read().current_line;
                buffer.with_mut(|buff| buff.add(create_line(&lines2[count], "green")));
            },
            "line"
        }
        button {
            padding: "10px",
            margin: "5px",
            style: "-webkit-appearance: none;
            -webkit-border-radius: 0;
            border-radius: 0; color: black; font-weight: bold;
            margin: 0.5vh; padding: 1vh; background-color: #d690d0;",
            onclick: move |_| {
                let back_count = go_back(&lines5.clone(), &char.read().charature, &buffer.read().current_line, 0);
                let count = get_current_number(&lines5, &char.read().charature, &back_count);
                let pcount = get_previous_number(&lines5, &char.read().charature, &count);
                //let count = buffer.read().current_line;
                let pcount: Vec<usize> = pcount.into_iter().rev().collect();
                let re = Regex::new(r"^\*\w").unwrap();
                for each in &pcount {
                    match re.captures(&lines5[*each]) {
                        Some(_i) => buffer.with_mut(|buff| buff.add(create_line(&lines5[*each], "#e3aa19"))),
                        None => buffer.with_mut(|buff| buff.add(create_line(&lines5[*each], "red"))),
                    }
                }
                let it = match pcount.last() {
                    Some(number) => number.to_owned(),
                    None => 0,
                };
                buffer.with_mut(|buff| buff.set_current_counter(count));
                buffer.with_mut(|buff| buff.set_previous_counter(it));
            },
            "again"
        }
        button {
            padding: "10px",
            margin: "5px",
            style: "-webkit-appearance: none;
            -webkit-border-radius: 0;
            border-radius: 0; color: black; font-weight: bold;
            margin: 0.5vh; padding: 1vh; background-color: #d690d0;",
            onclick: move |_| {
                let count = get_current_number(&lines4, &char.read().charature, &buffer.read().current_line);
                let back_count = go_back(&lines4, &char.read().charature, &buffer.read().current_line, 0);
                let page = get_page(&lines4, &back_count);
                buffer.with_mut(|buff| buff.set_page(page));
                let pcount = get_previous_number(&lines4, &char.read().charature, &back_count);
                //let count = buffer.read().current_line;
                let pcount: Vec<usize> = pcount.into_iter().rev().collect();
                let re = Regex::new(r"^\*\w").unwrap();
                for each in &pcount {
                    match re.captures(&lines4[*each]) {
                        Some(_i) => buffer.with_mut(|buff| buff.add(create_line(&lines4[*each], "#e3aa19"))),
                        None => buffer.with_mut(|buff| buff.add(create_line(&lines4[*each], "red"))),
                    }
                    //if lines4[*each].contains("sd:") {
                    //    buffer.with_mut(|buff| buff.add(create_line(&lines4[*each], "#e3aa19")));
                    //} else {
                    //    buffer.with_mut(|buff| buff.add(create_line(&lines4[*each], "red")));
                    //}
                }
                let it = match pcount.last() {
                    Some(number) => number.to_owned(),
                    None => 0,
                };
                buffer.with_mut(|buff| buff.set_current_counter(back_count));
                buffer.with_mut(|buff| buff.set_previous_counter(it));
            },
            "previous"
        }
        button {
            padding: "10px",
            margin: "5px",
            style: "-webkit-appearance: none;
            -webkit-border-radius: 0;
            border-radius: 0; color: black; font-weight: bold;
            margin: 0.5vh; padding: 1vh; background-color: #d690d0;",
            onclick: move |_| {
                let count = get_current_number(&lines, &char.read().charature, &buffer.read().current_line);
                let pcount = get_previous_number(&lines, &char.read().charature, &count);
                //let count = buffer.read().current_line;

                let page = get_page(&lines, &count);
                buffer.with_mut(|buff| buff.set_page(page));
                let pcount: Vec<usize> = pcount.into_iter().rev().collect();
                let re = Regex::new(r"^\*\w").unwrap();
                for each in &pcount {
                    match re.captures(&lines[*each]) {
                        Some(_i) => buffer.with_mut(|buff| buff.add(create_line(&lines[*each], "#e3aa19"))),
                        None => buffer.with_mut(|buff| buff.add(create_line(&lines[*each], "red"))),
                    }
                }
                let it = match pcount.last() {
                    Some(number) => number.to_owned(),
                    None => 0,
                };
                buffer.with_mut(|buff| buff.set_current_counter(count));
                buffer.with_mut(|buff| buff.set_previous_counter(it));
            },
            "next"
        }
        p { "Goto Page Number" }
        input {
            r#type: "number",
            oninput: move |evt| buffer.with_mut(|buff| buff.set_counter_from_page(&lines3, evt.value.clone().parse::<usize>().unwrap_or(0))),
        }

        div {
        button {
            padding: "10px",
            margin: "5px",
            style: "-webkit-appearance: none;
            -webkit-border-radius: 0;
            border-radius: 0; color: black; font-weight: bold;
            margin: 0.5vw; padding: 1vw; background-color: #d690d0;",
            onclick: move |_| {
                char.with_mut(|s| s.set("ROGER"));
            },
            "ROGER"
        }
        button {
            padding: "10px",
            margin: "5px",
            style: "-webkit-appearance: none;
            -webkit-border-radius: 0;
            border-radius: 0; color: black; font-weight: bold;
            margin: 0.5vw; padding: 1vw; background-color: #d690d0;",
            onclick: move |_| {
                char.with_mut(|s| s.set("BRUCE"));
            },
            "BRUCE"
        }
        button {
            padding: "10px",
            margin: "5px",
            style: "-webkit-appearance: none;
            -webkit-border-radius: 0;
            border-radius: 0; color: black; font-weight: bold;
            margin: 0.5vw; padding: 1vw; background-color: #d690d0;",
            onclick: move |_| {
                char.with_mut(|s| s.set("STAN"));
            },
            "STAN"
        }
        button {
            padding: "10px",
            margin: "5px",
            style: "-webkit-appearance: none;
            -webkit-border-radius: 0;
            border-radius: 0; color: black; font-weight: bold;
            margin: 0.5vw; padding: 1vw; background-color: #d690d0;",
            onclick: move |_| {
                char.with_mut(|s| s.set("MARTIN"));
            },
            "MARTIN"
        }
        button {
            padding: "10px",
            margin: "5px",
            style: "-webkit-appearance: none;
            -webkit-border-radius: 0;
            border-radius: 0; color: black; font-weight: bold;
            margin: 0.5vw; padding: 1vw; background-color: #d690d0;",
            onclick: move |_| {
                char.with_mut(|s| s.set("LISA"));
            },
            "LISA"
        }
        button {
            padding: "10px",
            margin: "5px",
            style: "-webkit-appearance: none;
            -webkit-border-radius: 0;
            border-radius: 0; color: black; font-weight: bold;
            margin: 0.5vw; padding: 1vw; background-color: #d690d0;",
            onclick: move |_| {
                char.with_mut(|s| s.set("RAEYWN"));
            },
            "RAEYWN"
        }
        button {
            padding: "10px",
            margin: "5px",
            style: "-webkit-appearance: none;
            -webkit-border-radius: 0;
            border-radius: 0; color: black; font-weight: bold;
            margin: 0.5vw; padding: 1vw; background-color: #d690d0;",
            onclick: move |_| {
                char.with_mut(|s| s.set("CHASTITY"));
            },
            "CHASTITY"
        }
        button {
            padding: "10px",
            margin: "5px",
            style: "-webkit-appearance: none;
            -webkit-border-radius: 0;
            border-radius: 0; color: black; font-weight: bold;
            margin: 0.5vw; padding: 1vw; background-color: #d690d0;",
            onclick: move |_| {
                char.with_mut(|s| s.set("ANDREA"));
            },
            "ANDREA"
        }
        }


        }

    })
}

fn main() {
    dioxus_web::launch(App);
    //dioxus_desktop::launch(App);
}
