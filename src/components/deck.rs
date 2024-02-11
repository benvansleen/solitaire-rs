use crate::{
    components::game::Game,
    game::{CardOutline, FaceDownCard, Selection, Solitaire},
};
use leptos::ev::{DragEvent, MouseEvent};
use leptos::*;

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
    let mut game = expect_context::<Game>();
    let waste = game.borrow().waste;
    let waste =
        move || waste().last().map(|card| card.view()).collect_view();

    let click = {
        let game = game.clone();
        move |e: MouseEvent| {
            e.stop_propagation();
            game.borrow_mut().play(Selection::Waste);
        }
    };

    let drag = {
        let game = game.clone();
        move |e: DragEvent| {
            game.borrow_mut().play(Selection::Waste);
        }
    };
    view! {
        <div
            class="deck"
            on:click=click
            on:dragstart=drag.clone()
            on:drop=drag
            on:dragover=move |e| e.prevent_default()
        >
            <CardOutline/>
            {waste}
        </div>
    }
}

#[component]
fn Deck() -> impl IntoView {
    let game = expect_context::<Game>();
    let deck = game.borrow().deck;

    let click = move |_| {
        game.borrow_mut().draw();
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
    let game = expect_context::<Game>();
    let foundations = move || {
        game.borrow()
            .foundations
            .iter()
            .enumerate()
            .map(|(idx, _)| view! { <Foundation idx/> })
            .collect_view()
    };

    view! { <div class="foundations">{foundations}</div> }
}

#[component]
fn Foundation(idx: usize) -> impl IntoView {
    let mut game = expect_context::<Game>();
    let foundation = {
        let foundation = game.borrow().foundations[idx];
        move || foundation().last().map(|card| card.view())
    };

    let click = {
        let game = game.clone();
        move |e: MouseEvent| {
            e.stop_propagation();
            game.borrow_mut().play(Selection::Foundation(idx));
        }
    };
    let drag = {
        let game = game.clone();
        move |e: DragEvent| {
            let mut game = game.borrow_mut();
            game.play(Selection::Foundation(idx));
        }
    };

    view! {
        <div
            class="foundation"
            on:click=click
            on:dragstart=drag.clone()
            on:drop=drag
            on:dragover=move |e| e.prevent_default()
        >
            <CardOutline/>
            {foundation}
        </div>
    }
}
