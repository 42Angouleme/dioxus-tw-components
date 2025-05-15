use crate::{attributes::*, components::atoms::icon::*};
use dioxus::prelude::*;
use dioxus_core::AttributeValue;
use dioxus_tw_components_macro::UiComp;

struct AccordionState {
    multi_open: bool,
    active_items: Vec<String>,
}

impl AccordionState {
    fn new(multi_open: bool) -> Self {
        Self {
            multi_open,
            active_items: Vec::with_capacity(1),
        }
    }

    fn add_id(&mut self, id: String) {
        self.active_items.push(id);
    }

    fn remove_id(&mut self, id: String) {
        self.active_items.retain(|x| x != &id);
    }

    fn set_id(&mut self, id: String) {
        self.active_items.clear();
        self.active_items.push(id);
    }

    fn is_active(&self, id: &str) -> bool {
        self.active_items.contains(&id.to_string())
    }

    fn is_active_to_attr_value(&self, id: String) -> AttributeValue {
        match self.active_items.contains(&id) {
            true => AttributeValue::Text("active".to_string()),
            false => AttributeValue::Text("inactive".to_string()),
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct AccordionProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// Control if multiple items can be open at the same time
    #[props(default = false)]
    multi_open: bool,

    children: Element,
}

impl std::default::Default for AccordionProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            multi_open: false,
            children: rsx! {},
        }
    }
}

/// The Accordion component divides the content into collapsible items \
/// Usage:
/// ```ignore
/// rsx!{Accordion {
///     AccordionItem {
///         AccordionTrigger { id: "acc-1", "Trigger 1" }
///         AccordionContent { id: "acc-1", "Content 1" }
///     }
///     AccordionItem {
///         AccordionTrigger { id: "acc-2", "Trigger 2" }
///         AccordionContent { id: "acc-2", "Content 2" }
///     }
/// }}
/// ```
#[component]
pub fn Accordion(mut props: AccordionProps) -> Element {
    use_context_provider(|| Signal::new(AccordionState::new(props.multi_open)));

    props.update_class_attribute();

    rsx! {
        div { ..props.attributes,{props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct AccordionItemProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for AccordionItemProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

/// Wrapper for the [AccordionTrigger] and [AccordionContent] components
#[component]
pub fn AccordionItem(mut props: AccordionItemProps) -> Element {
    props.update_class_attribute();

    rsx! {
        div { ..props.attributes,{props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct AccordionTriggerProps {
    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    id: ReadOnlySignal<String>,

    /// Determine if the accordion item is open by default
    #[props(optional, default = false)]
    is_open: bool,

    /// Decoration element that is displayed next to the trigger text, by default a chevron
    #[props(optional, default = default_trigger_decoration())]
    trigger_decoration: Element,

    children: Element,
}

impl std::default::Default for AccordionTriggerProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            id: ReadOnlySignal::<String>::default(),
            is_open: false,
            trigger_decoration: default_trigger_decoration(),
            children: rsx! {},
        }
    }
}

/// The clickable element that toggles the visibility of the [AccordionContent] component
#[component]
pub fn AccordionTrigger(mut props: AccordionTriggerProps) -> Element {
    props.update_class_attribute();

    let mut state = use_context::<Signal<AccordionState>>();

    let onmounted = move |_| async move {
        if props.is_open {
            state.write().add_id(props.id.read().clone());
        }
    };

    let button_closure = move |_: Event<MouseData>| {
        // If the current item is active, remove it from the list, effectively closing it
        if state.read().is_active(&props.id.read()) {
            state.write().remove_id(props.id.read().clone());
        } else {
            // If the current item is not active
            // set it as the only active item if multi_open is false
            // or add it to the list of active items if multi_open is true
            if !state.read().multi_open {
                state.write().set_id(props.id.read().clone());
            } else {
                state.write().add_id(props.id.read().clone());
            }
        }
    };

    rsx! {
        button {
            "data-state": state.read().is_active_to_attr_value(props.id.read().to_string()),
            onclick: button_closure,
            onmounted,
            ..props.attributes,
            {props.children}
            {props.trigger_decoration}
        }
    }
}

#[component]
fn default_trigger_decoration() -> Element {
    rsx! {
        Icon {
            class: "transition-transform transform duration-300 group-data-[state=active]:-rotate-180",
            icon: Icons::ExpandMore,
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct AccordionContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    id: ReadOnlySignal<String>,

    #[props(optional)]
    height: ReadOnlySignal<String>,

    #[props(default)]
    pub animation: ReadOnlySignal<Animation>,

    children: Element,
}

impl std::default::Default for AccordionContentProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            id: ReadOnlySignal::<String>::default(),
            height: ReadOnlySignal::<String>::default(),
            animation: ReadOnlySignal::<Animation>::default(),
            children: rsx! {},
        }
    }
}

/// Collapsible element that is toggled by the [AccordionTrigger] component
#[component]
pub fn AccordionContent(mut props: AccordionContentProps) -> Element {
    // This is the height of the element when visible, we need to calcul it before rendering it to have a smooth transition
    let mut elem_height = use_signal(|| "".to_string());

    props.update_class_attribute();

    let state = use_context::<Signal<AccordionState>>();

    let final_height = match state.read().is_active(&props.id.read()) {
        true => {
            if props.height.read().is_empty() {
                elem_height()
            } else {
                props.height.read().clone()
            }
        }
        false => "0".to_string(),
    };

    rsx! {
        div {
            onmounted: move |element| async move {
                if !props.height.read().is_empty() {
                    return;
                }
                if props.animation == Animation::None {
                    elem_height.set("auto".to_string());
                    return;
                }
                elem_height
                    .set(
                        match element.data().get_scroll_size().await {
                            Ok(size) => format!("{}px", size.height),
                            Err(e) => {
                                log::error!(
                                    "AccordionContent: Failed to get element height(id probably not set): setting it to auto: {:?}",
                                    e
                                );
                                "auto".to_string()
                            }
                        },
                    );
            },
            "data-state": state.read().is_active_to_attr_value(props.id.read().to_string()),
            id: props.id,
            height: final_height,
            ..props.attributes,
            {props.children}
        }
    }
}
