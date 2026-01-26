use crate::components::icon::*;
use crate::dioxus_core::IntoAttributeValue;
use dioxus::prelude::*;
use dioxus_core::AttributeValue;

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

#[derive(Clone, PartialEq, Props)]
pub struct ModalProps {
    #[props(default = false)]
    is_active: bool,

    children: Element,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    let mut state = use_context_provider(|| Signal::new(ModalState::new(props.is_active)));

    rsx! {
        div {
            tabindex: 0,
            onkeydown: move |e: KeyboardEvent| {
                if e.key() == Key::Escape {
                    state.write().toggle();
                }
            },
            {props.children}
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct ModalTriggerProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    onclick: EventHandler<MouseEvent>,

    children: Element,
}

#[component]
pub fn ModalTrigger(mut props: ModalTriggerProps) -> Element {
    let mut state = use_context::<Signal<ModalState>>();

    let default_classes = "button";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let onclick = move |event: Event<MouseData>| {
        event.stop_propagation();
        state.write().toggle();
        props.onclick.call(event)
    };

    rsx! {
        button { onclick, ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct ModalCloseProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(default)]
    children: Element,
}

impl std::default::Default for ModalCloseProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: Ok(VNode::default()), // Default this way to be able to check the children in SidePanelClose
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
        let default_classes = "modal-close";
        crate::setup_class_attribute(&mut props.attributes, default_classes);
    }

    let onclick = move |event: Event<MouseData>| {
        event.stop_propagation();
        state.write().toggle();
    };

    rsx! {
        div {
            "data-state": state.read().into_value(),
            onclick,
            ..props.attributes,
            if !has_children {
                Icon { icon: Icons::Close }
            } else {
                {props.children}
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct ModalContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn ModalContent(mut props: ModalContentProps) -> Element {
    let state = use_context::<Signal<ModalState>>();

    let default_classes = "modal-content";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div {
            "data-state": state.read().into_value(),
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct ModalBackgroundProps {
    #[props(optional, default = true)]
    interactive: bool,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    onclick: EventHandler<MouseEvent>,

    children: Element,
}

#[component]
pub fn ModalBackground(mut props: ModalBackgroundProps) -> Element {
    let mut state = use_context::<Signal<ModalState>>();

    let default_classes = "modal-background";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

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
