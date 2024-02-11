use crate::game::Card;
use leptos::*;
use leptos_dom::log;
use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Selection {
    Pile(usize, usize),
    Foundation(usize),
    Deck,
    Waste,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solitaire {
    pub deck: RwSignal<Vec<Card>>,
    pub waste: RwSignal<Vec<Card>>,
    pub piles: [RwSignal<Vec<Card>>; 7],
    pub foundations: [RwSignal<Vec<Card>>; 4],
    pub selected: RwSignal<Option<Selection>>,
}

fn move_card(
    from: RwSignal<Vec<Card>>,
    n_from: usize,
    to: RwSignal<Vec<Card>>,
) {
    let card: Vec<_> = from
        .try_update(|from| from.drain(from.len() - n_from..).collect())
        .expect("for signal to still be valid");

    to.update(|to| to.extend(card));
}

impl Solitaire {
    pub fn new(mut cards: Vec<Card>) -> Self {
        log!("Creating new solitaire game");
        cards.shuffle(&mut rand::thread_rng());

        let mut drain_into_signal =
            |r| create_rw_signal(cards.drain(r).collect());
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
            foundations: std::array::from_fn(|_| {
                create_rw_signal(Vec::new())
            }),
            selected: create_rw_signal(None),
        }
    }

    pub fn clear_selection(&self) {
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

        from_card.value + 1 == to_card.value
            && from_card.color() != to_card.color()
    }

    fn is_valid_move_to_foundation(
        &self,
        from: &[Card],
        to: &[Card],
    ) -> bool {
        if from.is_empty() {
            return false;
        }
        if to.is_empty() {
            return from.last().unwrap().value == 1;
        }
        let from_card = from.last().unwrap();
        let to_card = to.last().unwrap();

        from_card.value - 1 == to_card.value
            && from_card.suit == to_card.suit
    }

    pub fn play(&mut self, s: Selection) {
        use Selection::*;
        log!("Playing {:?}", s);
        match (self.selected.get(), s) {
            (None, _) => {
                self.selected.set(Some(s));
                return;
            }
            (Some(Pile(from, from_card)), Pile(to, _)) => {
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
                let valid = self.is_valid_move_to_foundation(
                    &source(),
                    &destination(),
                );

                if valid {
                    move_card(source, 1, destination);
                }
            }
            (Some(Foundation(source)), Pile(destination, _)) => {
                let source = self.foundations[source];
                let destination = self.piles[destination];
                let valid =
                    self.is_valid_move_to_pile(&source(), &destination());

                if valid {
                    move_card(source, 1, destination);
                }
            }
            (Some(Waste), Pile(destination, _)) => {
                let source = self.waste;
                let destination = self.piles[destination];
                let valid =
                    self.is_valid_move_to_pile(&source(), &destination());

                if valid {
                    move_card(source, 1, destination);
                }
            }
            (Some(Waste), Foundation(destination)) => {
                let source = self.waste;
                let destination = self.foundations[destination];
                let valid = self.is_valid_move_to_foundation(
                    &source(),
                    &destination(),
                );

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
