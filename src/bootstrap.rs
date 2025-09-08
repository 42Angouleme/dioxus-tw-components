use dioxus::prelude::*;

#[component]
pub fn Bootstrap() -> Element {
    let style = include_str!("../dioxus-tw-components-style.css");

    rsx! {
        document::Stylesheet { href: "https://fonts.googleapis.com/icon?family=Material+Icons" }
        style { {style} }
    }
}
