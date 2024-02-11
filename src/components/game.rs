use crate::{
    components::{DeckArea, Foundations, Pile},
    game::Solitaire,
};
use leptos::ev::MouseEvent;
use leptos::*;
use leptos_dom::log;

#[component]
pub fn Game(game: Solitaire) -> impl IntoView {
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
