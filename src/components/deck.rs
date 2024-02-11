use leptos::*;
use leptos::ev::MouseEvent;
use crate::game::{
    CardOutline,
    FaceDownCard,
    Selection,
    Solitaire,
};

#[component]
pub fn DeckArea() -> impl IntoView {
    view! {
        <div class="deck-area">
            <Deck/>
            <Waste/>
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
pub fn Foundations() -> impl IntoView {
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
