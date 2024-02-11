use crate::{
    components::{DeckArea, Foundations, Pile},
    game::{Selection, Solitaire},
};
use leptos::ev::{DragEvent, MouseEvent};
use leptos::*;
use leptos_dom::log;
use std::{cell::RefCell, rc::Rc};

pub type Game = Rc<RefCell<Solitaire>>;

#[component]
pub fn Game(game: Solitaire) -> impl IntoView {
    let game = Rc::new(RefCell::new(game));
    provide_context(game.clone());

    let clear_selection = {
        let game = game.clone();
        move |e: MouseEvent| {
            e.stop_propagation();
            log!("Clearing selection");
            game.borrow_mut().clear_selection();
        }
    };
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
    let game = expect_context::<Game>();
    let piles = {
        let piles = game.borrow().piles;
        move || {
            piles
                .iter()
                .enumerate()
                .map(|(idx, &cards)| view! { <Pile idx cards/> })
                .collect_view()
        }
    };

    view! { <div class="piles">{piles}</div> }
}
