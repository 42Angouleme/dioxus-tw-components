use dioxus::prelude::*;

#[derive(Default, Clone, PartialEq, Props)]
pub struct BootstrapConfig {
    #[props(optional, default)]
    pub icon: Option<Asset>,
}

#[component]
pub fn DioxusTwComponentsBootstrap(props: BootstrapConfig) -> Element {
    rsx! {
        document::Stylesheet { href: "https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded:FILL@1" }
        if let Some(icon) = props.icon {
            document::Link { rel: "icon", href: icon }
        }
    }
}
