use crate::components::icon::*;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CheckboxProps {
    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// Form name. Empty = the checkbox does not participate in form submission.
    #[props(optional, into, default = String::new())]
    name: String,

    /// Value submitted when checked.
    #[props(optional, into, default = String::from("on"))]
    value: String,

    #[props(optional)]
    default_checked: bool,

    #[props(optional)]
    checked: Signal<bool>,

    /// Return value determines if the event should stop propagation (false by default).
    #[props(optional)]
    onchange: Callback<bool, bool>,
}

#[component]
pub fn Checkbox(mut props: CheckboxProps) -> Element {
    let default_classes = "checkbox";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let mut checked = use_signal(|| props.default_checked);
    let mut prev_default = use_signal(|| props.default_checked);

    // Re-sync only when the prop itself changes; comparing against `checked` would clobber user clicks.
    if *prev_default.peek() != props.default_checked {
        *prev_default.write() = props.default_checked;
        *checked.write() = props.default_checked;
    }

    let id = crate::use_unique_id();

    // Styled button is the interactive surface; hidden input carries name/checked for form submission and a11y.
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
            name: props.name,
            value: props.value,
            checked: *checked.read(),
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
