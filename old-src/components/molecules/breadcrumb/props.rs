use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_tw_components_macro::UiComp;

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct BreadcrumbProps {
    #[props(extends = ol, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for BreadcrumbProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

/// Usage:
/// ```ignore
/// Breadcrumb {
///     BreadcrumbItem { "Home" },
///     BreadcrumbSeparator {},
///     BreadcrumbItem { "Library" },
///     BreadcrumbSeparator {},
///     BreadcrumbItem { "Data" },
/// }
/// ```
#[component]
pub fn Breadcrumb(mut props: BreadcrumbProps) -> Element {
    props.update_class_attribute();

    rsx! {
        ol { ..props.attributes,{props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct BreadcrumbItemProps {
    #[props(extends = li, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for BreadcrumbItemProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn BreadcrumbItem(mut props: BreadcrumbItemProps) -> Element {
    props.update_class_attribute();

    rsx! {
        li { ..props.attributes,{props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct BreadcrumbSeparatorProps {
    #[props(extends = li, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for BreadcrumbSeparatorProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn BreadcrumbSeparator(mut props: BreadcrumbSeparatorProps) -> Element {
    props.update_class_attribute();

    rsx! {
        li { aria_hidden: "true", ..props.attributes,
            if props.children == rsx! {} {
                "\u{203A}"
            } else {
                {props.children}
            }
        }
    }
}
