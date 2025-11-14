use crate::components::icon::*;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CheckboxProps {
    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    default_checked: bool,

    #[props(optional)]
    checked: Signal<bool>,

    /// Return value determines if the event should strop propagation (false by default)
    #[props(optional)]
    onchange: Callback<bool, bool>,
}

#[component]
pub fn Checkbox(mut props: CheckboxProps) -> Element {
    let default_classes = "checkbox";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let mut checked = use_signal(|| props.default_checked);

    let id = crate::use_unique_id();
    let id_clone = id.clone();

    // HTML's default checkbox input are notoriously difficult to style consistently across browsers.
    // This one uses a common pattern using a fake box for look and real input for sementics and
    // form integration, aria-hidden to prevent duplication.
    // The hidden native input ensures that assistive technologies (like screen readers) still
    // recognize the component as a proper checkbox.
    use_effect(move || {
        let checked = *checked.read();
        let js = document::eval(
            r#"
            let id = await dioxus.recv();
            let action = await dioxus.recv();
            let input = document.getElementById(id);

            switch(action) {
                case "checked":
                    input.checked = true;
                    input.indeterminate = false;
                    break;
                case "unchecked": 
                    input.checked = false;
                    input.indeterminate = false;
                    break;
            }
            "#,
        );

        let _ = js.send(id_clone.clone());
        let _ = js.send(if checked { "checked" } else { "unchecked" });
    });

    rsx! {
        button {
            type: "button",
            role: "checkbox",
            "data-checked": if *checked.read() { "checked" } else { "unchecked" },
            onclick: move |event| {
                let new_checked = !checked();
                checked.set(new_checked);
                props.checked.set(new_checked);
                if props.onchange.call(new_checked) {
                    event.stop_propagation();
                }
            },

            // Aria says only spacebar can change state of checkboxes.
            onkeydown: move |e| {
                if e.key() == Key::Enter {
                    e.prevent_default();
                }
            },
            ..props.attributes,
            span { class: "checkbox-indicator",
                if *checked.read() {
                    Icon {
                        icon: Icons::Check
                    }
                }
            }
        }
        input {
            id,
            type: "checkbox",
            aria_hidden: "true",
            tabindex: "-1",
            position: "absolute",
            pointer_events: "none",
            opacity: "0",
            margin: "0",
            transform: "translateX(-100%)",
        }
    }
}
