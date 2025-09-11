use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct BreadcrumbProps {
    #[props(extends = ol, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
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
    let default_classes = "breadcrumb";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        ol { ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct BreadcrumbItemProps {
    #[props(extends = li, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn BreadcrumbItem(mut props: BreadcrumbItemProps) -> Element {
    let default_classes = "breadcrumb-item";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        li { ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct BreadcrumbSeparatorProps {
    #[props(extends = li, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn BreadcrumbSeparator(mut props: BreadcrumbSeparatorProps) -> Element {
    let default_classes = "breadcrumb-separator";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        li {
            aria_hidden: "true",
            ..props.attributes,
            if props.children == rsx! {} {
                "\u{203A}"
            } else {
                {props.children}
            }
        }
    }
}
