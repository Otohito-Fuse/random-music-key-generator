// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Copy, Clone)]
pub enum Key {
    Cmajor,
    Aminor,
    Fmajor,
    Dminor,
    Bflatmajor,
    Gminor,
    Eflatmajor,
    Cminor,
    Aflatmajor,
    Fminor,
    Dflatmajor,
    Bflatminor,
    Gflatmajor,
    Eflatminor,
    Bmajor,
    Gsharpminor,
    Emajor,
    Csharpminor,
    Amajor,
    Fsharpminor,
    Dmajor,
    Bminor,
    Gmajor,
    Eminor,
}

impl Key {
    pub fn make_vec() -> Vec<Self> {
        vec![
            Self::Cmajor,
            Self::Aminor,
            Self::Fmajor,
            Self::Dminor,
            Self::Bflatmajor,
            Self::Gminor,
            Self::Eflatmajor,
            Self::Cminor,
            Self::Aflatmajor,
            Self::Fminor,
            Self::Dflatmajor,
            Self::Bflatminor,
            Self::Gflatmajor,
            Self::Eflatminor,
            Self::Bmajor,
            Self::Gsharpminor,
            Self::Emajor,
            Self::Csharpminor,
            Self::Amajor,
            Self::Fsharpminor,
            Self::Dmajor,
            Self::Bminor,
            Self::Gmajor,
            Self::Eminor,
        ]
    }
}

#[derive(Copy, Clone)]
pub enum Languages {
    Japanese,
    English,
    Germany,
}

pub fn key_name(lang: Languages, key: Key) -> &'static str {
    match lang {
        Languages::Japanese => match key {
            Key::Cmajor => "ハ長調",
            Key::Aminor => "イ短調",
            Key::Fmajor => "ヘ長調",
            Key::Dminor => "ニ短調",
            Key::Bflatmajor => "変ロ長調",
            Key::Gminor => "ト短調",
            Key::Eflatmajor => "変ホ長調",
            Key::Cminor => "ハ短調",
            Key::Aflatmajor => "変イ長調",
            Key::Fminor => "ヘ短調",
            Key::Dflatmajor => "変ニ長調",
            Key::Bflatminor => "変ロ短調",
            Key::Gflatmajor => "変ト長調",
            Key::Eflatminor => "変ホ短調",
            Key::Bmajor => "ロ長調",
            Key::Gsharpminor => "嬰ト短調",
            Key::Emajor => "ホ長調",
            Key::Csharpminor => "嬰ハ短調",
            Key::Amajor => "イ長調",
            Key::Fsharpminor => "嬰ヘ短調",
            Key::Dmajor => "ニ長調",
            Key::Bminor => "ロ短調",
            Key::Gmajor => "ト長調",
            Key::Eminor => "ホ短調",
        },
        Languages::English => match key {
            Key::Cmajor => "C major",
            Key::Aminor => "A minor",
            Key::Fmajor => "F major",
            Key::Dminor => "D minor",
            Key::Bflatmajor => "Bb major",
            Key::Gminor => "G minor",
            Key::Eflatmajor => "Eb major",
            Key::Cminor => "C minor",
            Key::Aflatmajor => "Ab major",
            Key::Fminor => "F minor",
            Key::Dflatmajor => "Db major",
            Key::Bflatminor => "Bb minor",
            Key::Gflatmajor => "Gb major",
            Key::Eflatminor => "Eb minor",
            Key::Bmajor => "B major",
            Key::Gsharpminor => "G# minor",
            Key::Emajor => "E major",
            Key::Csharpminor => "C# minor",
            Key::Amajor => "A major",
            Key::Fsharpminor => "F# minor",
            Key::Dmajor => "D major",
            Key::Bminor => "B minor",
            Key::Gmajor => "G major",
            Key::Eminor => "E minor",
        },
        Languages::Germany => match key {
            Key::Cmajor => "C-Dur",
            Key::Aminor => "a-Moll",
            Key::Fmajor => "F-Dur",
            Key::Dminor => "d-Moll",
            Key::Bflatmajor => "B-Dur",
            Key::Gminor => "g-Moll",
            Key::Eflatmajor => "Es-Dur",
            Key::Cminor => "c-Moll",
            Key::Aflatmajor => "As-Dur",
            Key::Fminor => "f-Moll",
            Key::Dflatmajor => "Des-Dur",
            Key::Bflatminor => "b-Moll",
            Key::Gflatmajor => "Ges-Dur",
            Key::Eflatminor => "es-Moll",
            Key::Bmajor => "H-Dur",
            Key::Gsharpminor => "gis-Moll",
            Key::Emajor => "E-Dur",
            Key::Csharpminor => "cis-Moll",
            Key::Amajor => "A-Dur",
            Key::Fsharpminor => "fis-Moll",
            Key::Dmajor => "D-Dur",
            Key::Bminor => "h-Moll",
            Key::Gmajor => "G-Dur",
            Key::Eminor => "e-Moll",
        },
    }
}

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        language: Languages::Germany,
        key: None,
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    language: Languages,
    key: Option<Key>,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    SelectLanguage(Languages),
    Generate,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::SelectLanguage(lang) => {
            model.language = lang;
        }
        Msg::Generate => {
            let mut rng = thread_rng();
            model.key = Some(*Key::make_vec().choose(&mut rng).unwrap());
        }
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        C!["wrapper"],
        div![
            C!["menu-area"],
            div![
                C!["each-menu"],
                "日本語",
                ev(Ev::Click, move |_| Msg::SelectLanguage(Languages::Japanese)),
            ],
            div![
                C!["each-menu"],
                "English",
                ev(Ev::Click, move |_| Msg::SelectLanguage(Languages::English)),
            ],
            div![
                C!["each-menu"],
                "Deutsch",
                ev(Ev::Click, move |_| Msg::SelectLanguage(Languages::Germany)),
            ],
        ],
        div![
            C!["button"],
            "generate",
            ev(Ev::Click, move |_| Msg::Generate),
        ],
        div![
            C!["key-name"],
            if let Some(key) = model.key {
                key_name(model.language, key).to_string()
            } else {
                String::new()
            },
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
