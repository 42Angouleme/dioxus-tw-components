use crate::components::{button::*, icon::*};
use dioxus::dioxus_core::AttributeValue;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct PaginationProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub class: ReadOnlySignal<String>,
    #[props(into)]
    pub data_size: ReadOnlySignal<usize>,
    #[props(into)]
    pub page_size: ReadOnlySignal<usize>,
    pub page_number: Signal<usize>,
}

#[component]
pub fn Pagination(mut props: PaginationProps) -> Element {
    let default_classes = "pagination";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let max_pages = use_memo(move || (*props.data_size.read() / *props.page_size.read()) + 1);

    let data_variant_attribute = match props.attributes.iter_mut().find(|attr| attr.name == "data-variant") {
        Some(attribute) => if let AttributeValue::Text(ref value) = attribute.value { value.clone() } else { "none".to_string() },
        _ => "none".to_string(),
    };
    let data_color_attribute = match props.attributes.iter_mut().find(|attr| attr.name == "data-style") {
        Some(attribute) => if let AttributeValue::Text(ref value) = attribute.value { value.clone() } else { "none".to_string() },
        _ => "none".to_string(),
    };

    let page_selector = use_memo( {
        let data_variant_attribute = data_variant_attribute.clone();
        let data_color_attribute = data_color_attribute.clone();
        move || {
            let mut next_dots = false;
            let mut prev_dots = false;
            if *props.page_number.read() > 2 {
                prev_dots = true;
            }
            if *props.page_number.read() <= max_pages.read().checked_sub(2).unwrap_or(0) {
                next_dots = true;
            }
            rsx! {
                if prev_dots {
                    Button {
                        class: props.class,
                        disabled: *props.page_number.read() == 1,
                        "data-variant": data_variant_attribute.clone(),
                        "data-style": data_color_attribute.clone(),
                        onclick: move |_event: MouseEvent| {
                            props.page_number.set(1);
                        },
                        "1"
                    }
                    p { class: "pagination-dots", "..." }
                }
                for page in (std::cmp::max(1_isize, *props.page_number.read() as isize - 1)
                    as usize)..=std::cmp::min(*max_pages.read(), *props.page_number.read() + 1)
                {
                    Button {
                        class: props.class,
                        "data-variant": data_variant_attribute.clone(),
                        "data-style": data_color_attribute.clone(),
                        disabled: *props.page_number.read() == page,
                        onclick: move |_event: MouseEvent| {
                            props.page_number.set(page);
                        },
                        "{page}"
                    }
                }
                if next_dots {
                    p { class: "pagination-dots", "..." }
                    Button {
                        class: props.class,
                        "data-variant": data_variant_attribute.clone(),
                        "data-style": data_color_attribute.clone(),
                        disabled: *props.page_number.read() == *max_pages.read(),
                        onclick: move |_event: MouseEvent| {
                            props.page_number.set(*max_pages.peek());
                        },
                        "{*max_pages.read()}"
                    }
                }
            }
        }
    });

    rsx! {
        div { class: format!("{} {}", default_classes, props.class),
            ..props.attributes,
            Button {
                class: format!("pagination-nav-button {}", props.class),
                "data-variant": data_variant_attribute.clone(),
                "data-style": data_color_attribute.clone(),
                disabled: *props.page_number.read() == 1,
                onclick: move |_event: MouseEvent| {
                    let value = *props.page_number.peek();
                    props.page_number.set(value - 1);
                },
                Icon { icon: Icons::ArrowLeft }
            }
            {page_selector}
            Button {
                class: format!("pagination-nav-button {}", props.class),
                "data-variant": data_variant_attribute.clone(),
                "data-style": data_color_attribute.clone(),
                disabled: *props.page_number.read() == *max_pages.read(),
                onclick: move |_event: MouseEvent| {
                    let value = *props.page_number.peek();
                    props.page_number.set(value + 1);
                },
                Icon { icon: Icons::ArrowRight }
            }
        }
    }
}
