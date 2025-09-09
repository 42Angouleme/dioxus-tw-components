use crate::components::icon::*;
use dioxus::prelude::*;
use dioxus_core::AttributeValue;

#[derive(Clone, PartialEq, Props)]
pub struct CalloutProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    title: std::string::String,

    #[props(optional, default)]
    pub icon: Option<Icons>,

    children: Element,
}

#[component]
pub fn Callout(mut props: CalloutProps) -> Element {
    let icon = if let Some(data_attribute) = props
        .attributes
        .iter()
        .find(|attr| attr.name == "data-variant")
    {
        if let AttributeValue::Text(ref value) = data_attribute.value {
            match value.as_str() {
                "tip" => Icons::Lightbulb,
                "warning" => Icons::Warning,
                "caution" => Icons::Report,
                "info" | &_ => Icons::Info,
            }
        } else {
            Icons::Info
        }
    } else {
        Icons::Info
    };

    let default_classes = "callout";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div {..props.attributes,
            div { style: "display: flex; flex-direction: row; vertical-align: middle;",
                Icon {
                    style: "margin-right: 0.5rem; font-size: 0.875rem;",
                    icon: if props.icon.is_some() { props.icon.unwrap() } else { icon },
                }
                "{props.title}"
            }
            div { style: "font-size: 0.875rem; color: color-mix(in oklab, var(--foreground) 70%, transparent)",
                {props.children}
            }
        }
    }
}
