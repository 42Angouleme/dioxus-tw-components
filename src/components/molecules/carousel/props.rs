use dioxus::prelude::*;
use dioxus_core::AttributeValue;
use dioxus_tw_components_macro::UiComp;

use crate::{attributes::*, components::atoms::icon::*};

struct CarouselState {
    is_circular: bool,
    carousel_size: u32,
    // Use a key there so we can just +1 or -1 instead of having a vec
    current_item_key: u32,
    content_width: i32,
    current_translation: i32,
}

impl CarouselState {
    fn new(current_item_key: u32, is_circular: bool) -> Self {
        Self {
            current_item_key,
            is_circular,
            carousel_size: 0,
            content_width: 0,
            current_translation: 0,
        }
    }

    fn increment_carousel_size(&mut self) {
        self.carousel_size += 1;
    }

    fn set_content_size(&mut self, scroll_width: i32) {
        self.content_width = scroll_width;
    }

    fn go_to_next_item(&mut self) {
        self.current_item_key += 1;
    }

    fn go_to_prev_item(&mut self) {
        self.current_item_key -= 1;
    }

    fn go_to_item(&mut self, item_key: u32) {
        self.current_item_key = item_key;
    }

    fn is_active_to_attr_value(&self, key: u32) -> AttributeValue {
        match self.current_item_key == key {
            true => AttributeValue::Text("active".to_string()),
            false => AttributeValue::Text("inactive".to_string()),
        }
    }

    fn translate(&mut self) {
        self.set_current_translation(self.current_item_key as i32 * self.content_width)
    }

    fn set_current_translation(&mut self, translation: i32) {
        self.current_translation = translation;
    }

    fn get_current_translation(&self) -> i32 {
        self.current_translation
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct CarouselProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(default = 0)]
    default_item_key: u32,
    #[props(default = false)]
    is_circular: bool,

    children: Element,
}

impl std::default::Default for CarouselProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            default_item_key: 0,
            is_circular: false,
            children: rsx! {},
        }
    }
}

/// Usage :
/// ```ignore
/// Carousel { default_item_key: 0,
///     CarouselTrigger { next: false }
///     CarouselWindow {
///         CarouselContent { id: "carousel-1",
///             CarouselItem { item_key: 0, div { "ITEM 1" } }
///             CarouselItem { item_key: 1, div { "ITEM 2" } }
///         }
///     }
///     CarouselTrigger { next: true }
/// }
#[component]
pub fn Carousel(mut props: CarouselProps) -> Element {
    use_context_provider(|| {
        Signal::new(CarouselState::new(
            props.default_item_key,
            props.is_circular,
        ))
    });

    props.update_class_attribute();

    rsx! {
        div { ..props.attributes,{props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct CarouselWindowProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for CarouselWindowProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn CarouselWindow(mut props: CarouselWindowProps) -> Element {
    props.update_class_attribute();

    rsx! {
        div { ..props.attributes,{props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct CarouselContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    id: ReadOnlySignal<String>,

    #[props(default)]
    pub animation: ReadOnlySignal<Animation>,

    children: Element,
}

impl std::default::Default for CarouselContentProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            id: ReadOnlySignal::<String>::default(),
            animation: ReadOnlySignal::<Animation>::default(),
            children: rsx! {},
        }
    }
}

/// You need to pass it an id for it to work
#[component]
pub fn CarouselContent(mut props: CarouselContentProps) -> Element {
    let mut carousel_state = use_context::<Signal<CarouselState>>();

    props.update_class_attribute();

    let style = use_memo(move || {
        format!(
            "transform: translateX(-{}px)",
            carousel_state.read().get_current_translation()
        )
    });

    rsx! {
        div {
            style,
            id: props.id,
            onresize: move |element| {
                carousel_state
                    .write()
                    .set_content_size(
                        match element.data().get_content_box_size() {
                            Ok(size) => size.width as i32,
                            Err(_) => 0,
                        },
                    );
            },
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct CarouselItemProps {
    /// Represent position in the carousel
    item_key: u32,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for CarouselItemProps {
    fn default() -> Self {
        Self {
            item_key: 0,
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn CarouselItem(mut props: CarouselItemProps) -> Element {
    let mut state = use_context::<Signal<CarouselState>>();

    props.update_class_attribute();

    let onmounted = move |_| {
        state.write().increment_carousel_size();
    };

    rsx! {
        div {
            "data-state": state.read().is_active_to_attr_value(props.item_key),
            onmounted,
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Default, Clone, PartialEq, Props, UiComp)]
pub struct CarouselTriggerProps {
    #[props(default = false)]
    next: bool,

    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn CarouselTrigger(mut props: CarouselTriggerProps) -> Element {
    let mut carousel_state = use_context::<Signal<CarouselState>>();

    props.update_class_attribute();

    let onclick = move |_| async move {
        scroll_carousel(props.next, carousel_state);
        carousel_state.write().translate();
    };

    let icon = get_next_prev_icons(props.next);

    rsx! {
        button { onclick, ..props.attributes, {icon} }
    }
}

fn scroll_carousel(next: bool, mut carousel_state: Signal<CarouselState>) {
    let mut carousel_state = carousel_state.write();
    let current_key = carousel_state.current_item_key;
    let carousel_size = carousel_state.carousel_size;

    if next {
        if current_key < carousel_size - 1 {
            carousel_state.go_to_next_item();
        } else if carousel_state.is_circular {
            carousel_state.go_to_item(0);
        }
    } else if current_key > 0 {
        carousel_state.go_to_prev_item();
    } else if carousel_state.is_circular {
        carousel_state.go_to_item(carousel_size - 1);
    }
}

fn get_next_prev_icons(is_next: bool) -> Element {
    match is_next {
        true => rsx! {
            Icon { icon: Icons::ChevronRight }
        },
        false => rsx! {
            Icon { icon: Icons::ChevronLeft }
        },
    }
}
