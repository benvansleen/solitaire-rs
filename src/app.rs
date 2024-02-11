use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_dom::log;
use leptos_meta::*;
use leptos_router::*;
use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <head>
            <Stylesheet id="leptos" href="/pkg/solitaire.css"/>
            <Title text="Solitaire"/>
        </head>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=Main/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Main() -> impl IntoView {
    view! { <Solitaire/> }
}

#[component]
fn FaceDownCard() -> impl IntoView {
    view! { <img class="card" draggable="false" clickable="false" src="cards/face_down.jpg"/> }
}

#[component]
fn CardOutline() -> impl IntoView {
    view! {
        <span class="card-outline">
            <img src="cards/ace_of_spades.png"/>
        </span>
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
struct Card {
    suit: Suit,
    value: u8,
    filename: String,
    faceup: bool,
}

impl Card {
    fn new(filename: String) -> Self {
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

    fn color(&self) -> &'static str {
        match self.suit {
            Suit::Spades | Suit::Clubs => "black",
            Suit::Hearts | Suit::Diamonds => "red",
        }
    }

    fn filename(&self) -> String {
        format!("cards/{}", self.filename)
    }

    fn flip(&mut self) {
        self.faceup = true;
    }

    fn id(&self) -> String {
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

    fn view(&self) -> View {
        if self.faceup {
            self.show_faceup()
        } else {
            view! { <FaceDownCard/> }
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Selection {
    Pile(usize, usize),
    Foundation(usize),
    Deck,
    Waste,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solitaire {
    deck: RwSignal<Vec<Card>>,
    waste: RwSignal<Vec<Card>>,
    piles: [RwSignal<Vec<Card>>; 7],
    foundations: [RwSignal<Vec<Card>>; 4],
    selected: RwSignal<Option<Selection>>,
}

fn move_card(from: RwSignal<Vec<Card>>, n_from: usize, to: RwSignal<Vec<Card>>) {
    let card: Vec<_> = from
        .try_update(|from| from.drain(from.len() - n_from..).collect())
        .expect("for signal to still be valid");

    to.update(|to| to.extend(card));
}

impl Solitaire {
    fn new(mut cards: Vec<Card>) -> Self {
        log!("Creating new solitaire game");
        cards.shuffle(&mut rand::thread_rng());

        let mut drain_into_signal = |r| create_rw_signal(cards.drain(r).collect());
        let piles = [
            drain_into_signal(0..1),
            drain_into_signal(0..2),
            drain_into_signal(0..3),
            drain_into_signal(0..4),
            drain_into_signal(0..5),
            drain_into_signal(0..6),
            drain_into_signal(0..7),
        ];

        Self {
            deck: create_rw_signal(cards.to_vec()),
            waste: create_rw_signal(Vec::new()),
            piles,
            foundations: std::array::from_fn(|_| create_rw_signal(Vec::new())),
            selected: create_rw_signal(None),
        }
    }

    fn clear_selection(&self) {
        self.selected.set(None);
    }

    fn is_valid_move_to_pile(&self, from: &[Card], to: &[Card]) -> bool {
        if from.is_empty() {
            return false;
        }
        if to.is_empty() {
            return from.last().unwrap().value == 13;
        }
        let from_card = from.last().unwrap();
        let to_card = to.last().unwrap();

        from_card.value + 1 == to_card.value && from_card.color() != to_card.color()
    }

    fn is_valid_move_to_foundation(&self, from: &[Card], to: &[Card]) -> bool {
        if from.is_empty() {
            return false;
        }
        if to.is_empty() {
            return from.last().unwrap().value == 1;
        }
        let from_card = from.last().unwrap();
        let to_card = to.last().unwrap();

        from_card.value - 1 == to_card.value && from_card.suit == to_card.suit
    }

    fn play(&mut self, s: Selection) {
        use Selection::*;
        log!("Playing {:?}", s);
        match (self.selected.get(), s) {
            (None, _) => {
                self.selected.set(Some(s));
                return;
            }
            (Some(Pile(from, from_card)), Pile(to, to_card)) => {
                let source = self.piles[from];
                let destination = self.piles[to];
                let valid = self.is_valid_move_to_pile(
                    &source()[..=source().len() - from_card],
                    &destination(),
                );
                if valid {
                    move_card(source, from_card, destination);
                }
            }
            (Some(Pile(from, _)), Foundation(destination)) => {
                let source = self.piles[from];
                let destination = self.foundations[destination];
                let valid = self.is_valid_move_to_foundation(&source(), &destination());

                if valid {
                    move_card(source, 1, destination);
                }
            }
            (Some(Foundation(source)), Pile(destination, _)) => {
                let source = self.foundations[source];
                let destination = self.piles[destination];
                let valid = self.is_valid_move_to_pile(&source(), &destination());

                if valid {
                    move_card(source, 1, destination);
                }
            }
            (Some(Waste), Pile(destination, _)) => {
                let source = self.waste;
                let destination = self.piles[destination];
                let valid = self.is_valid_move_to_pile(&source(), &destination());

                if valid {
                    move_card(source, 1, destination);
                }
            }
            (Some(Waste), Foundation(destination)) => {
                let source = self.waste;
                let destination = self.foundations[destination];
                let valid = self.is_valid_move_to_foundation(&source(), &destination());

                if valid {
                    move_card(source, 1, destination);
                }
            }
            _ => {
                log!("Invalid move");
            }
        }

        self.selected.set(None);
    }
}

fn unique_id(id: &str) -> String {
    use rand::Rng;
    let r = rand::thread_rng().gen::<u64>();
    format!("{}-{}", id, r)
}

use leptos::ev::MouseEvent;
#[component]
fn PileCard(pile_idx: usize, card_idx: usize, card: Card) -> impl IntoView {
    let mut game = expect_context::<Solitaire>();
    let click = move |e: MouseEvent| {
        e.stop_propagation();
        game.play(Selection::Pile(pile_idx, card_idx))
    };
    view! {
        <span class="card" on:click=click>
            {card.view()}
        </span>
    }
}

#[component]
fn Pile(idx: usize, cards: RwSignal<Vec<Card>>) -> impl IntoView {
    let mut game = expect_context::<Solitaire>();
    let click = move |_| {
        game.play(Selection::Pile(idx, 0));
    };

    let pile = move || {
        cards.update(|cards| {
            if let Some(card) = cards.last_mut() {
                card.flip();
            }
        });
        let cards = cards();
        (1..=cards.len()).rev().zip(cards.into_iter())
    };

    let cards = move |(card_idx, card)| {
        view! { <PileCard pile_idx=idx card_idx card=card/> }
    };

    view! {
        <div class="pile" on:click=click>
            <CardOutline/>
            <For each=pile key=move |(i, card)| format!("{}-{}", i, card.id()) children=cards/>
        </div>
    }
}

#[component]
fn Piles() -> impl IntoView {
    let game = expect_context::<Solitaire>();
    let piles = move || {
        game.piles
            .iter()
            .enumerate()
            .map(|(idx, &cards)| view! { <Pile idx cards/> })
            .collect_view()
    };

    view! { <div class="piles">{piles}</div> }
}

#[component]
fn Foundation(idx: usize) -> impl IntoView {
    let mut game = expect_context::<Solitaire>();
    let foundation = move || game.foundations[idx].get().last().map(|card| card.view());
    let click = move |e: MouseEvent| {
        e.stop_propagation();
        game.play(Selection::Foundation(idx));
    };

    view! {
        <div class="foundation" on:click=click>
            <CardOutline/>
            {foundation}
        </div>
    }
}

#[component]
fn Foundations() -> impl IntoView {
    let game = expect_context::<Solitaire>();
    let foundations = game.foundations;
    let foundations = move || {
        foundations
            .iter()
            .enumerate()
            .map(|(idx, _)| view! { <Foundation idx/> })
            .collect_view()
    };

    view! { <div class="foundations">{foundations}</div> }
}

#[component]
fn Deck() -> impl IntoView {
    let game = expect_context::<Solitaire>();
    let deck = game.deck;
    let waste = game.waste;

    let click = move |_| match deck().last() {
        Some(card) => {
            deck.update(|d| {
                d.pop();
            });
            waste.update(|w| {
                let mut card = card.to_owned();
                card.flip();
                w.push(card);
            });
        }
        None => {
            let mut cur_waste = waste();
            cur_waste.reverse();
            deck.update(|d| {
                d.extend(cur_waste);
            });
            waste.update(|w| {
                w.clear();
            });
        }
    };

    let deck = move || {
        deck()
            .is_empty()
            .then(|| view! { <div></div> }.into_view())
            .unwrap_or_else(|| view! { <FaceDownCard/> }.into_view())
    };

    view! {
        <div class="deck" on:click=click>
            <CardOutline/>
            {deck}
        </div>
    }
}

#[component]
fn Waste() -> impl IntoView {
    let mut game = expect_context::<Solitaire>();
    let waste = game.waste;
    let click = move |e: MouseEvent| {
        e.stop_propagation();
        game.play(Selection::Waste);
    };
    let waste = move || waste().last().map(|card| card.view()).collect_view();

    view! {
        <div class="deck" on:click=click>
            <CardOutline/>
            {waste}
        </div>
    }
}

#[component]
fn DeckArea() -> impl IntoView {
    view! {
        <div class="deck-area">
            <Deck/>
            <Waste/>
        </div>
    }
}

#[component]
fn TopRow() -> impl IntoView {
    view! {
        <div class="top-row">
            <Foundations/>
            <DeckArea/>
        </div>
    }
}

#[component]
fn Game(game: Solitaire) -> impl IntoView {
    let clear_selection = move |e: MouseEvent| {
        e.stop_propagation();
        log!("Clearing selection");
        game.selected.set(None);
    };
    provide_context(game);

    view! {
        <h1>"Solitaire"</h1>
        <div class="game" on:click=clear_selection>
            <TopRow/>
            <Piles/>
        </div>
    }
}

#[server]
pub async fn fetch_cards() -> Result<Solitaire, ServerFnError> {
    let cards = std::fs::read_dir(Path::new("public").join("cards"))
        .expect("public/cards does not exist")
        .map(|path| {
            path.expect("failed to read card path")
                .path()
                .file_name()
                .expect("failed to get card file name")
                .to_str()
                .expect("failed to convert card file name to string")
                .to_owned()
        })
        .filter(|card| !card.ends_with("joker.png"))
        .filter(|card| card.ends_with(".png"))
        .map(Card::new)
        .collect();

    Ok(Solitaire::new(cards))
}

#[component]
fn Solitaire() -> impl IntoView {
    let game = create_resource(|| (), |_| async { fetch_cards().await.unwrap() });

    view! {
        <Suspense fallback=move || {
            view! { <div>"Loading..."</div> }
        }>{move || game.get().map(|game| view! { <Game game/> })}</Suspense>
    }
}
