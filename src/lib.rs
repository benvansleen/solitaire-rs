use cfg_if::cfg_if;
pub mod app;
pub mod components;
pub mod error_template;
pub mod fileserv;
pub mod game;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(app::App);
    }
}}
