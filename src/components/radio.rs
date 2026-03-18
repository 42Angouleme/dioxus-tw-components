use dioxus::prelude::*;

#[derive(Clone)]
struct RadioGroupCtx {
    value: Signal<String>,
    default_value: Signal<String>,
    onchange: EventHandler<MouseEvent>,
}

#[derive(Clone, PartialEq, Props)]
pub struct RadioGroupProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,

    #[props(optional)]
    default_value: String,

    #[props(optional)]
    value: Signal<String>,

    #[props(optional)]
    onchange: EventHandler<MouseEvent>,
}

#[component]
pub fn RadioGroup(mut props: RadioGroupProps) -> Element {
    let mut default_value = use_signal(|| props.default_value.clone());

    use_context_provider(|| RadioGroupCtx {
        value: props.value,
        default_value,
        onchange: props.onchange,
    });

    // Sync default_value from prop when it changes.
    // peek() avoids subscription; write only when different to prevent re-render loops.
    if *default_value.peek() != props.default_value {
        *default_value.write() = props.default_value.clone();
    }

    let default_classes = "radio";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div { ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct RadioItemProps {
    #[props(extends = input, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    value: String,
}

#[component]
pub fn RadioItem(mut props: RadioItemProps) -> Element {
    let mut state = use_context::<RadioGroupCtx>();

    let default_classes = "radio-input";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let value = props.value.clone();
    let checked = use_memo(move || {
        if state.value.read().is_empty() {
            *state.default_value.read() == value
        } else {
            (state.value)() == value
        }
    });

    rsx! {
        input {
            r#type: "radio",
            checked,
            onclick: move |event| {
                state.value.set(props.value.clone());
                state.onchange.call(event);
            },
            ..props.attributes,
        }
    }
}
