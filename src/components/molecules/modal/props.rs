use crate::{attributes::*, components::atoms::icon::*};
use dioxus::prelude::*;
use dioxus_core::AttributeValue;
use dioxus_tw_components_macro::UiComp;

#[derive(Clone, Copy)]
pub struct ModalState {
    is_active: bool,
}

impl ModalState {
    fn new(is_active: bool) -> Self {
        Self { is_active }
    }

    pub fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }
}

impl IntoAttributeValue for ModalState {
    fn into_value(self) -> AttributeValue {
        match self.is_active {
            true => AttributeValue::Text("active".to_string()),
            false => AttributeValue::Text("inactive".to_string()),
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ModalProps {
    #[props(default = false)]
    is_active: bool,

    children: Element,
}

impl std::default::Default for ModalProps {
    fn default() -> Self {
        Self {
            is_active: false,
            children: rsx! {},
        }
    }
}

/// Usage: \
/// ```ignore
/// Modal {
///     ModalTrigger {
///          "Open Modal"
///     }
///     ModalBackground {}
///     ModalContent {
///        div {
///             ModalClose { "X" }
///        }
///        div { class: "h4", "TITLE" }
///        div { class: "paragraph", "CONTENT" }
///    }
/// }
/// ```
#[component]
pub fn Modal(props: ModalProps) -> Element {
    use_context_provider(|| Signal::new(ModalState::new(props.is_active)));

    rsx! {
        {props.children}
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ModalTriggerProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    onclick: EventHandler<MouseEvent>,

    children: Element,
}

impl std::default::Default for ModalTriggerProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            onclick: EventHandler::<MouseEvent>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn ModalTrigger(mut props: ModalTriggerProps) -> Element {
    let mut state = use_context::<Signal<ModalState>>();

    props.update_class_attribute();

    let onclick = move |event: Event<MouseData>| {
        event.stop_propagation();
        state.write().toggle();
        props.onclick.call(event)
    };

    rsx! {
        div { onclick, ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ModalCloseProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    children: Element,
}

impl std::default::Default for ModalCloseProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: Ok(VNode::default()), // Default this way to be able to check the children in ModalClose
        }
    }
}

/// Div to close the content modal, by default it is a cross located at the top left corner of the modal
/// If you provide a children, it will be used instead of the default cross and no internal styling will be provided
#[component]
pub fn ModalClose(mut props: ModalCloseProps) -> Element {
    let mut state = use_context::<Signal<ModalState>>();

    let has_children = props.children != Ok(VNode::default());

    if !has_children {
        props.update_class_attribute();
    }

    let onkeypress = move |event: KeyboardEvent| {
        event.stop_propagation();
        if event.data.key() == Key::Escape {
            state.write().toggle();
        }
    };

    let onclick = move |event: Event<MouseData>| {
        event.stop_propagation();
        state.write().toggle();
    };

    rsx! {
        div { onclick, onkeypress, ..props.attributes,
            if !has_children {
                Icon { icon: Icons::Close }
            } else {
                {props.children}
            }
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ModalContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub animation: ReadOnlySignal<Animation>,

    children: Element,
}

impl std::default::Default for ModalContentProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            animation: ReadOnlySignal::<Animation>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn ModalContent(mut props: ModalContentProps) -> Element {
    let state = use_context::<Signal<ModalState>>();

    props.update_class_attribute();

    rsx! {
        div { "data-state": state.read().into_value(), ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ModalBackgroundProps {
    #[props(optional, default = true)]
    interactive: bool,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub color: ReadOnlySignal<Color>,
    #[props(optional, default)]
    pub animation: ReadOnlySignal<Animation>,
    #[props(optional, default)]
    onclick: EventHandler<MouseEvent>,

    children: Element,
}

impl std::default::Default for ModalBackgroundProps {
    fn default() -> Self {
        Self {
            interactive: true,
            attributes: Vec::<Attribute>::default(),
            color: ReadOnlySignal::<Color>::default(),
            animation: ReadOnlySignal::<Animation>::default(),
            onclick: EventHandler::<MouseEvent>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn ModalBackground(mut props: ModalBackgroundProps) -> Element {
    let mut state = use_context::<Signal<ModalState>>();

    props.update_class_attribute();

    let onclick = move |event: Event<MouseData>| {
        event.stop_propagation();
        if props.interactive {
            state.write().toggle();
            props.onclick.call(event)
        }
    };

    rsx! {
        div {
            "data-state": state.read().into_value(),
            onclick,
            ..props.attributes,
            {props.children}
        }
    }
}
