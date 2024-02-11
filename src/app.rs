use crate::{
    components::game::Game,
    error_template::{AppError, ErrorTemplate},
    game::{Card, Solitaire},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
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
    let game =
        create_resource(|| (), |_| async { fetch_cards().await.unwrap() });

    view! {
        <Suspense fallback=move || {
            view! { <div>"Loading..."</div> }
        }>
            {move || game.get().map(|game| view! { <Game game/> })}
        </Suspense>
    }
}
