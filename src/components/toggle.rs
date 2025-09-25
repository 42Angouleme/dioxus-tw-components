use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ToggleProps {
    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    #[props(optional)]
    checked: Option<bool>,

    #[props(optional)]
    onclick: EventHandler<MouseEvent>,

    children: Element,
}

#[component]
pub fn Toggle(mut props: ToggleProps) -> Element {
    let default_classes = "toggle";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let mut interior_sig = use_signal(|| props.checked.unwrap_or_default());

    let onclick = move |event| {
        interior_sig.toggle();
        props.onclick.call(event);
    };

    rsx! {
        button {
            "data-state": if interior_sig() { "checked" } else { "unchecked" },
            r#type: "button",
            onclick,
            ..props.attributes,
            span { class: "toggle-thumb" }
            {props.children}
        }
    }
}
