use crate::{
    components::game::Game,
    game::{Card, CardOutline, Selection, Solitaire},
};
use leptos::ev::{DragEvent, MouseEvent};
use leptos::*;
use leptos_dom::log;

#[component]
pub fn Pile(idx: usize, cards: RwSignal<Vec<Card>>) -> impl IntoView {
    let game = expect_context::<Game>();

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

    let click = move || {
        game.borrow_mut().play(Selection::Pile(idx, 0));
    };
    let drag = click.clone();

    view! {
        <div
            class="pile"
            on:click=move |_| click()
            on:drop=move |_| drag()
            on:dragover=move |e| e.prevent_default()
        >
            <CardOutline/>
            <For
                each=pile
                key=move |(i, card)| format!("{}-{}", i, card.id())
                children=cards
            />
        </div>
    }
}

#[component]
fn PileCard(
    pile_idx: usize,
    card_idx: usize,
    card: Card,
) -> impl IntoView {
    let game = expect_context::<Game>();
    let click = {
        let game = game.clone();
        move |e: MouseEvent| {
            e.stop_propagation();
            game.borrow_mut().play(Selection::Pile(pile_idx, card_idx))
        }
    };
    let drag = {
        let game = game.clone();
        move |e: DragEvent| {
            let mut game = game.borrow_mut();
            game.play(Selection::Pile(pile_idx, card_idx))
        }
    };

    view! {
        <span
            class="card"
            on:click=click
            on:dragstart=drag.clone()
            on:drop=drag
            on:dragover=move |e| e.prevent_default()
        >
            {card.view()}
        </span>
    }
}
