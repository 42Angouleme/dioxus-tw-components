use crate::dioxus_core::IntoAttributeValue;
use dioxus::prelude::*;
use dioxus_core::AttributeValue;

#[derive(Clone, Copy)]
pub struct DropdownManager {
    generation: u64,
}

impl DropdownManager {
    fn new() -> Self {
        Self { generation: 0 }
    }

    fn advance(&mut self) -> u64 {
        self.generation += 1;
        self.generation
    }

    fn generation(&self) -> u64 {
        self.generation
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct DropdownManagerProviderProps {
    children: Element,
}

#[component]
pub fn DropdownManagerProvider(props: DropdownManagerProviderProps) -> Element {
    use_context_provider(|| Signal::new(DropdownManager::new()));
    rsx! { {props.children} }
}

#[derive(Clone, Copy)]
struct DropdownState {
    is_active: bool,
}

impl DropdownState {
    fn new() -> Self {
        Self { is_active: false }
    }

    fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }

    fn close(&mut self) {
        self.is_active = false;
    }

    fn get_is_active(&self) -> bool {
        self.is_active
    }
}

impl IntoAttributeValue for DropdownState {
    fn into_value(self) -> AttributeValue {
        match self.is_active {
            true => AttributeValue::Text("open".to_string()),
            false => AttributeValue::Text("closed".to_string()),
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct DropdownProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

/// Usage:
/// ```ignore
/// Dropdown {
///    DropdownToggle {
///        "Dropdown"
///     }
///     DropdownContent {
///       div { "content" }
///    }
/// }
/// ```
/// Use 0 closing_delay_ms to disable the auto close feature
#[component]
pub fn Dropdown(mut props: DropdownProps) -> Element {
    // Capture parent dropdown state before providing our own,
    // so clicking backdrop closes the entire dropdown chain.
    let parent_state = try_use_context::<Signal<DropdownState>>();
    let mut state = use_context_provider(|| Signal::new(DropdownState::new()));

    // Generation counter: close this dropdown when another interaction advances the counter
    let manager = try_use_context::<Signal<DropdownManager>>();
    let mut my_generation = use_signal(|| 0u64);
    let mut was_active = use_signal(|| false);

    // Watch for generation changes and open/close transitions
    if let Some(mgr) = manager {
        let global_gen = mgr.read().generation();
        let is_active = state.read().get_is_active();

        if is_active && !*was_active.read() {
            // Just opened — store current generation so we don't self-close
            my_generation.set(global_gen);
        } else if is_active && global_gen > *my_generation.read() {
            // Was already open and generation advanced externally — close
            state.write().close();
        }

        was_active.set(is_active);
    }

    let default_classes = "dropdown";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div { "data-state": state.read().into_value(), ..props.attributes, {props.children} }
        if state.read().get_is_active() {
            div {
                class: "dropdown-backdrop",
                onclick: move |_event| {
                    state.write().close();
                    if let Some(mut parent) = parent_state {
                        parent.write().close();
                    }
                    // Advance generation so sibling dropdowns close too
                    if let Some(mut mgr) = manager {
                        let new_gen = mgr.write().advance();
                        my_generation.set(new_gen);
                    }
                },
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct DropdownToggleProps {
    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn DropdownToggle(mut props: DropdownToggleProps) -> Element {
    let mut state = use_context::<Signal<DropdownState>>();
    let manager = try_use_context::<Signal<DropdownManager>>();

    let default_classes = "button";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        button {
            onclick: move |e: MouseEvent| {
                e.stop_propagation();
                e.prevent_default();

                // Advance generation before opening so other dropdowns close
                let will_open = !state.read().get_is_active();
                if will_open
                    && let Some(mut mgr) = manager
                {
                    mgr.write().advance();
                }
                state.write().toggle();
            },
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct DropdownContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn DropdownContent(mut props: DropdownContentProps) -> Element {
    let mut state = use_context::<Signal<DropdownState>>();

    let default_classes = "dropdown-content";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        div {
            "data-state": state.read().into_value(),
            onclick: move |_| {
                state.write().close();
            },
            ..props.attributes,
            {props.children}
        }
    }
}
