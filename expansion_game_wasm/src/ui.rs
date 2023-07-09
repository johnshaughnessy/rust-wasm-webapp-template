use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod theme;
use theme::{get_theme, set_theme, Theme};

mod style;
use style::BUTTON_PRIMARY;

use self::style::BASE;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let add_to_counter = {
        let counter = counter.clone();
        Callback::from(move |_| {
            let value = *counter + 1;
            counter.set(value);
        })
    };

    let text = match get_theme() {
        Theme::Light => "Dark Mode",
        Theme::Dark => "Light Mode",
    };

    let change_theme_button_text = use_state(|| text);

    let change_theme = {
        let change_theme_button_text = change_theme_button_text.clone();

        Callback::from(move |_| {
            match get_theme() {
                Theme::Light => set_theme(Theme::Dark),
                Theme::Dark => set_theme(Theme::Light),
            };

            let text = match get_theme() {
                Theme::Light => "Dark Mode",
                Theme::Dark => "Light Mode",
            };
            change_theme_button_text.set(text);
        })
    };

    html! {
        <div>
            <button class={BUTTON_PRIMARY} onclick={change_theme} >{ *change_theme_button_text }</button>
            <button class={BUTTON_PRIMARY} onclick={add_to_counter} >{ "+1" }</button>
            <p class={"text-primary"}>{ *counter }</p>
        </div>
    }
}

#[wasm_bindgen]
pub fn start_ui() {
    let document = gloo::utils::document();
    let root = document.get_element_by_id("yew-root").unwrap();
    root.set_class_name(style::BASE);
    set_theme(get_theme());
    let renderer = yew::Renderer::<App>::with_root(root);
    renderer.render();
}
