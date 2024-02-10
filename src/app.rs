use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_dom::log;
use leptos_meta::*;
use leptos_router::*;
use std::path::Path;
use rand::prelude::SliceRandom;
use serde::{Serialize, Deserialize};
// use thaw::{Button, Slider, Spinner};

static ASSETS: &str = "public";

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
    view! {
        <Solitaire />
    }
}

#[component]
fn Card(#[prop(into)] card: String) -> impl IntoView {
    let (border, set_border) = create_signal(false);
    let click = {
        let card = card.clone();
        move |_| {
            log!("Clicked: {:?}", card);
            set_border(!border());
        }
    };

    view! {
        <img on:click=click
        class="card"
        class:selected=border
        src=card />
    }
}

#[component]
fn FaceDownCard() -> impl IntoView {
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
}

// impl IntoView for Card {
//     fn into_view(self) -> View {
//         let (border, set_border) = create_signal(false);
//         let click = move |_| {
//             set_border(!border());
//         };
//         view! {
//             <img
//               class="card"
//               class:selected=border
//               on:click=click
//               src={self.filename()}
//             />
//         }.into_view()
//     }
// }

// impl Into<String> for Card {
//     fn into(self) -> String {
//         self.filename()
//     }
// }

impl From<&Card> for String {
    fn from(card: &Card) -> String {
        card.filename()
    }
}

// impl From<Card> for String {
//     fn from(card: Card) -> String {
//         card.filename()
//     }
// }

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

        Self { suit, value, filename }
    }

    fn filename(&self) -> String {
        format!("cards/{}", self.filename.to_owned())
    }

    // fn view(&self) -> View {
    //     let (border, set_border) = create_signal(false);
    //     let click = move |_| {
    //         set_border(!border());
    //     };

    //     view! {
    //         <img
    //           class="card"
    //           class:selected=border
    //           on:click=click
    //           src={self.filename()}
    //         />
    //     }.into_view()
    // }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solitaire {
    deck: RwSignal<Vec<Card>>,
    waste: RwSignal<Vec<Card>>,
    piles: [RwSignal<Vec<Card>>; 7],
    foundations: [RwSignal<Vec<Card>>; 4],
    // deck: Vec<RwSignal<Card>>,
    // waste: Vec<RwSignal<Card>>,
    // piles: [Vec<RwSignal<Card>>; 7],
    // foundations: [Vec<RwSignal<Card>>; 4],
}

impl Solitaire {
    fn new(mut cards: Vec<Card>) -> Self {
        log!("Creating new solitaire game");
        cards.shuffle(&mut rand::thread_rng());

        let mut drain_into_signal = |r| {
            create_rw_signal(cards.drain(r).collect())
        };
        let piles = [
            drain_into_signal(0..1),
            drain_into_signal(0..2),
            drain_into_signal(0..3),
            drain_into_signal(0..4),
            drain_into_signal(0..5),
            drain_into_signal(0..6),
            drain_into_signal(0..7),
            // cards.drain(0..1).collect(),
            // cards.drain(0..2).collect(),
            // cards.drain(0..3).collect(),
            // cards.drain(0..4).collect(),
            // cards.drain(0..5).collect(),
            // cards.drain(0..6).collect(),
            // cards.drain(0..7).collect(),
        ];

        Self {
            // deck: cards.iter().map(|c| create_rw_signal(c.to_owned())).collect(),
            // waste: Vec::new(),
            // piles,
            // foundations: std::array::from_fn(|_| Vec::new()),
            deck: create_rw_signal(cards.to_vec()),
            waste: create_rw_signal(Vec::new()),
            piles,
            foundations: std::array::from_fn(|_| create_rw_signal(Vec::new())),
            // selected: create_rw_signal(None),
            // destination: create_rw_signal(None),
            // deck: cards.to_vec(),
            // waste: Vec::new(),
            // piles: piles,
            // foundations: std::array::from_fn(|_| Vec::new()),
        }
    }

    // fn play(&mut self, m: Move) {
    //     match (m.source, m.destination) {
    //         (Source::Pile(s), Destination::Pile(d)) => {
    //             let card = self.piles[s].pop().unwrap();
    //             self.piles[d].push(card);
    //         }
    //         (Source::Pile(s), Destination::Foundation(d)) => {
    //             let card = self.piles[s].pop().unwrap();
    //             self.foundations[d].push(card);
    //         }
    //         (Source::Foundation(s), Destination::Pile(d)) => {
    //             let card = self.foundations[s].pop().unwrap();
    //             self.piles[d].push(card);
    //         }
    //         (Source::Foundation(s), Destination::Foundation(d)) => {
    //             let card = self.foundations[s].pop().unwrap();
    //             self.foundations[d].push(card);
    //         }
    //     }
    // }

    // fn view(&self) -> impl IntoView {
    //     let piles = self.piles
    //         .iter()
    //         .enumerate()
    //         .map(|(i, pile)| view! {
    //             <Pile idx=i cards=pile.clone() />
    //         })
    //         .collect_view();

    //     let foundations = self.foundations
    //         .iter()
    //         .map(|f| {
    //             let card = f
    //                 .last()
    //                 .map(|card| card.to_owned())
    //                 .map_or_else(
    //                     || view! { <div /> },
    //                     |card| view! {
    //                         <div>
    //                             <Card card=card />
    //                         </div>
    //                     },
    //                 );

    //             view! {
    //                 <div class="foundation">
    //                 <CardOutline />
    //                 {card}
    //                 </div>
    //             }
    //         })
    //         .collect_view();

    //     let deck = self.deck
    //         .is_empty()
    //         .then(|| view! { <div /> })
    //         .unwrap_or_else(|| view! { <div> <FaceDownCard /> </div> });

    //     let draw = self.waste
    //         .last()
    //         .map(|card| card.to_owned())
    //         .map(|card| view! { <div> <Card card=card /> </div> })
    //         .unwrap_or_else(|| view! { <div /> });

    //     view! {
    //         <div class="top-row">
    //             <div class="foundations"> {foundations} </div>
    //             <div class="deck-area">
    //                 <div class="deck">
    //                     <CardOutline />
    //                     {deck}
    //                 </div>
    //                 <div class="deck">
    //                     <CardOutline />
    //                     {draw}
    //                 </div>
    //             </div>
    //         </div>
    //         <div class="piles"> {piles} </div>
    //     }
    // }
}

fn unique_id(id: &str) -> String {
    use rand::Rng;
    let r = rand::thread_rng().gen::<u64>();
    format!("{}-{}", id, r)
}

#[component]
fn Pile<F>(
    idx: usize,
    #[prop(into)]
    cards: Signal<Vec<Card>>,
    on_click: F,
) -> impl IntoView
where
    F: Fn(usize) + 'static,
{
    let pile = move || {
        let cards = cards();
        cards
        .last()
        .map(|card| {
            std::iter::repeat_with(||
                (unique_id("facedown"), view! { <FaceDownCard /> })
            )
                .take(cards.len() - 1)
                .chain(std::iter::once(
                    (unique_id(&card.filename()), view! { <Card card /> })
                ))
                .collect::<Vec<(_, _)>>()
        })
        .unwrap_or_default()
    };

    view! {
        <div class="pile"
             on:click=move |_| on_click(idx)
        >
            <CardOutline />
        <For each=pile
              key=|(k, _)| k.clone()
        let:card
        >
        {card.1}
        </For>


        </div>
    }
}

#[component]
fn Piles() -> impl IntoView {
    let game = expect_context::<Solitaire>();
    let click = move |idx: usize| {
        let source = game.piles[idx];
        let destination = game.piles[idx + 1];
        let card = source().last().unwrap().to_owned();

        source.update(|s| { s.pop(); });
        destination.update(|d| { d.push(card); });
    };
    let piles = move || game
        .piles
        .iter()
        .enumerate()
        .map(|(idx, &cards)| view! { <Pile idx cards on_click=click /> })
        .collect_view();

    view! {
        <div class="piles">
            {piles}
        </div>
    }
}

#[component]
fn Foundations() -> impl IntoView {
    let game = expect_context::<Solitaire>();
    let foundations = game.foundations;
    let foundations = move || {
        foundations
            .map(|f| view! {
                <div class="foundation">
                <CardOutline />
                {f().last().map(|card| view! { <Card card /> })}
                </div>
            })
    };

    view! {
        <div class="foundations"> {foundations} </div>
    }
}

#[component]
fn Deck() -> impl IntoView {
    let game = expect_context::<Solitaire>();
    let deck = game.deck;
    let waste = game.waste;

    let click = move |_| {
        match deck().last() {
            Some(card) => {
                deck.update(|d| { d.pop(); });
                waste.update(|w| { w.push(card.to_owned()); })
            },
            None => {
                let mut cur_waste = waste();
                cur_waste.reverse();
                deck.update(|d| { d.extend(cur_waste); });
                waste.update(|w| { w.clear(); });
            }
        }
    };

    let deck = move || {
        deck().is_empty()
            .then(|| view! { <div /> }.into_view())
            .unwrap_or_else(|| view! { <FaceDownCard />  }.into_view())
    };

    view! {
        <div class="deck" on:click=click>
            <CardOutline />
            {deck}
        </div>
    }
}

#[component]
fn DeckArea() -> impl IntoView {
    let game = expect_context::<Solitaire>();
    let deck = game.deck;
    let waste = game.waste;

    let waste = move || {
        waste()
            .last()
            .map(|card| view! { <Card card /> })
    };

    view! {
        <div class="deck-area">
            <Deck />
            <div class="deck">
                <CardOutline />
                {waste}
            </div>
        </div>
    }
}

#[component]
fn TopRow() -> impl IntoView {
    view! {
        <div class="top-row">
            <Foundations />
            <DeckArea />
        </div>
    }
}

#[component]
fn Game(game: Solitaire) -> impl IntoView {
    provide_context(game);

    view! {
        <h1> "Solitaire" </h1>
        <div class="game">
            <TopRow />
            <Piles />
        </div>
    }
}

#[server]
pub async fn fetch_cards() -> Result<Solitaire, ServerFnError> {
    let cards = std::fs::read_dir(Path::new(ASSETS).join("cards"))
        .expect("public/cards does not exist")
        .map(|path| {
            path
                .expect("failed to read card path")
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
    let game = create_resource(|| (), |_| async {
        fetch_cards().await.unwrap()
    });

    view! {
        <Suspense
        fallback=move || view! { <div> "Loading..." </div> }
        >
        {move || game.get().map(|game| view! {
            <Game game />
        })}
        </Suspense>
    }


    // let game = Solitaire::new(&mut cards);

    // view! {
    //     <h1> "Solitaire" </h1>
    //     <Game game />
    // }
}
