use leptos::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn FaceDownCard() -> impl IntoView {
    view! {
        <img
            class="card"
            draggable="false"
            clickable="false"
            src="cards/face_down.jpg"
        />
    }
}

#[component]
pub fn CardOutline() -> impl IntoView {
    view! {
        <span class="card-outline">
            <img src="cards/ace_of_spades.png"/>
        </span>
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub suit: Suit,
    pub value: u8,
    filename: String,
    faceup: bool,
}

impl Card {
    pub fn new(filename: String) -> Self {
        let cleaned = filename.replace("of_", "");
        let name = cleaned
            .split('.')
            .collect::<Vec<&str>>()
            .first()
            .unwrap()
            .split('_')
            .collect::<Vec<&str>>();

        let suit = match *name.last().unwrap() {
            "spades" => Suit::Spades,
            "hearts" => Suit::Hearts,
            "diamonds" => Suit::Diamonds,
            "clubs" => Suit::Clubs,
            _ => panic!("Invalid suit"),
        };
        let value = match *name.first().unwrap() {
            "ace" => 1,
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            "10" => 10,
            "jack" => 11,
            "queen" => 12,
            "king" => 13,
            _ => panic!("Invalid value"),
        };

        Self {
            suit,
            value,
            filename,
            faceup: false,
        }
    }

    pub fn color(&self) -> &'static str {
        match self.suit {
            Suit::Spades | Suit::Clubs => "black",
            Suit::Hearts | Suit::Diamonds => "red",
        }
    }

    fn filename(&self) -> String {
        format!("cards/{}", self.filename)
    }

    pub fn flip(&mut self) {
        self.faceup = true;
    }

    pub fn id(&self) -> String {
        format!("{}-{}", self.filename, self.faceup)
    }

    fn show_faceup(&self) -> View {
        let (border, set_border) = create_signal(false);
        // let click = move |_| set_border(!border());

        view! {
            <img
                class="card"
                class:selected=border
                // on:click=click
                src=self.filename()
            />
        }
        .into_view()
    }

    pub fn view(&self) -> View {
        if self.faceup {
            self.show_faceup()
        } else {
            view! { <FaceDownCard/> }
        }
    }
}
