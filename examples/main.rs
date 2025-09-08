use dioxus::prelude::*;
use dioxus_tw_components::*;

static TAILWIND_CSS: Asset = asset!("/examples/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: TAILWIND_CSS }
        Bootstrap {}
        div { class: "flex flex-col space-y-2 p-4 w-fit mx-auto",
            Button {
                "data-color": "success",
                "data-size": "lg",
                "data-animation": "full",
                "test"
            }
            Button {
                "data-color": "destructive",
                "data-variant": "outline",
                "data-size": "lg",
                "data-animation": "full",
                "test"
            }
            Button {
                "data-color": "secondary",
                "data-variant": "ghost",
                "data-size": "lg",
                "data-animation": "full",
                "test"
            }
            Icon { class: "text-5xl mx-auto",
                icon: Icons::Discord,
            }
            Placeholder {
                "data-animation": "full",
            }
        }
    }
}
