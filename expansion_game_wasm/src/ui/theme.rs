use gloo::console::{error, log};
use gloo::storage::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Theme {
    Light,
    Dark,
}

pub fn get_theme() -> Theme {
    let storage = LocalStorage::raw();
    match storage.get("theme").unwrap() {
        Some(dark) if dark == "dark" => Theme::Dark,
        _ => Theme::Light,
    }
}

pub fn set_theme(theme: Theme) {
    let storage = LocalStorage::raw();
    let document = gloo::utils::document();
    // let root = document.get_element_by_id("yew-root").unwrap();
    let root = document.body().unwrap();
    let mut class_name = root.class_name();
    let result = if theme == Theme::Dark {
        class_name = class_name.replace(" light", "");
        root.set_class_name(&format!("{} dark", class_name));
        storage.set("theme", "dark")
    } else {
        class_name = class_name.replace(" dark", "");
        root.set_class_name(&format!("{} light", class_name));
        storage.set("theme", "light")
    };

    match result {
        Ok(_) => {
            log!("Theme set in local storage:", format!("{:?}", theme));
        }
        Err(_) => {
            error!(
                "Failed to set theme in local storage:",
                format!("{:?}", theme)
            );
        }
    }
}
