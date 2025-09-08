use dioxus::prelude::*;

#[derive(Default, Clone, PartialEq, Props)]
pub struct BootstrapConfig {
    #[props(optional, default)]
    pub icon: Option<Asset>,
}

#[component]
pub fn DioxusTwComponentsBootstrap(props: BootstrapConfig) -> Element {
    rsx! {
        document::Stylesheet { href: "https://fonts.googleapis.com/icon?family=Material+Icons" }
        if let Some(icon) = props.icon {
            document::Link { rel: "icon", href: icon }
        }
    }
}
