use super::CalloutVariant;
use crate::attributes::*;
use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_tw_components_macro::UiComp;

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct CalloutProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    title: std::string::String,

    #[props(optional, default)]
    pub variant: ReadOnlySignal<CalloutVariant>,
    #[props(optional, default)]
    pub icon: Option<Icons>,

    children: Element,
}

impl std::default::Default for CalloutProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            title: std::string::String::default(),
            variant: ReadOnlySignal::<CalloutVariant>::default(),
            icon: None,
            children: rsx! {},
        }
    }
}

#[component]
pub fn Callout(mut props: CalloutProps) -> Element {
    props.update_class_attribute();

    let icon = match *props.variant.read() {
        CalloutVariant::Note => Icons::Info,
        CalloutVariant::Tip => Icons::Lightbulb,
        CalloutVariant::Warning => Icons::Warning,
        CalloutVariant::Caution => Icons::Report,
    };

    rsx! {
        div {..props.attributes,
            div { class: "text-md flex flex-row align-middle",
                Icon {
                    class: "mr-2",
                    size: Size::Sm,
                    icon: if props.icon.is_some() { props.icon.unwrap() } else { icon },
                }
                "{props.title}"
            }
            div { class: "text-sm text-foreground/70", {props.children} }
        }
    }
}
