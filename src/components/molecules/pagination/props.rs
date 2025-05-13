use dioxus::prelude::*;
use dioxus_tw_components_macro::UiComp;

use std::string::String;
use tailwind_fuse::tw_merge;

use crate::prelude::ButtonVariant;

use crate::{
    atoms::{Button, Icon, Icons},
    attributes::*,
};

#[derive(Clone, PartialEq, Default, Props, UiComp)]
pub struct PaginationProps {
    #[props(optional, default)]
    pub class: ReadOnlySignal<String>,
    #[props(optional, default)]
    pub style: ReadOnlySignal<String>,

    #[props(optional, default)]
    pub color: ReadOnlySignal<Color>,
    #[props(optional, default)]
    pub size: ReadOnlySignal<Size>,
    #[props(optional, default)]
    pub variant: ReadOnlySignal<ButtonVariant>,

    #[props(into)]
    pub data_size: ReadOnlySignal<usize>,
    #[props(into)]
    pub page_size: ReadOnlySignal<usize>,
    pub page_number: Signal<usize>,
}

#[component]
pub fn Pagination(mut props: PaginationProps) -> Element {
    let max_pages = use_memo(move || (*props.data_size.read() / *props.page_size.read()) + 1);

    let page_selector = use_memo(move || {
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
                    style: props.style,
                    color: props.color,
                    size: props.size,
                    variant: props.variant,
                    disabled: *props.page_number.read() == 1,
                    onclick: move |_event: MouseEvent| {
                        props.page_number.set(1);
                    },
                    p { "1" }
                }
                p { class: "text-center h-9 px-2 py-2", "..." }
            }
            for page in (std::cmp::max(1_isize, *props.page_number.read() as isize - 1) as usize)..=std::cmp::min(*max_pages.read(), *props.page_number.read() + 1) {
                Button {
                    class: props.class,
                    style: props.style,
                    color: props.color,
                    size: props.size,
                    variant: props.variant,
                    disabled: *props.page_number.read() == page,
                    onclick: move |_event: MouseEvent| {
                        props.page_number.set(page);
                    },
                    p { "{page}" }
                }
            }
            if next_dots {
                p { class: "text-center h-9 px-2 py-2", "..." }
                Button {
                    class: props.class,
                    style: props.style,
                    color: props.color,
                    size: props.size,
                    variant: props.variant,
                    disabled: *props.page_number.read() == *max_pages.read(),
                    onclick: move |_event: MouseEvent| {
                        props.page_number.set(*max_pages.peek());
                    },
                    p { "{*max_pages.read()}" }
                }
            }
        }
    });

    rsx! {
        div { class: tw_merge!("flex flex-row gap-2 justify-center items-center", props.class.read()),
            Button {
                class: tw_merge!("flex justify-center items-center", props.class.read()),
                style: props.style,
                color: props.color,
                size: props.size,
                variant: props.variant,
                disabled: *props.page_number.read() == 1,
                onclick: move |_event: MouseEvent| {
                    let value = *props.page_number.peek();
                    props.page_number.set(value - 1);
                },
                Icon {icon: Icons::ArrowLeft}
                p { "Previous" }
            }
            {page_selector}
            Button {
                class: tw_merge!("flex justify-center items-center", props.class.read()),
                style: props.style,
                color: props.color,
                size: props.size,
                variant: props.variant,
                disabled: *props.page_number.read() == *max_pages.read(),
                onclick: move |_event: MouseEvent| {
                    let value = *props.page_number.peek();
                    props.page_number.set(value + 1);
                },
                p { "Next" }
                Icon {icon: Icons::ArrowRight}
            }
        }
    }
}
