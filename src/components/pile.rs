use crate::game::{Card, CardOutline, Selection, Solitaire};
use leptos::ev::MouseEvent;
use leptos::*;

#[component]
pub fn Pile(idx: usize, cards: RwSignal<Vec<Card>>) -> impl IntoView {
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
fn PileCard(
    pile_idx: usize,
    card_idx: usize,
    card: Card,
) -> impl IntoView {
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
