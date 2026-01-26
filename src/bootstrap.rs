use dioxus::prelude::*;

#[component]
pub fn Bootstrap() -> Element {
    let style = include_str!(concat!(env!("OUT_DIR"), "/dioxus-tw-components-style.css"));

    rsx! {
        document::Stylesheet { href: "https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded:FILL@1" }
        style { {style} }
    }
}
