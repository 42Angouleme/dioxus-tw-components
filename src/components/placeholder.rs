use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct PlaceholderProps {
    /// Additional attributes to apply to the element
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Placeholder(mut props: PlaceholderProps) -> Element {
    let default_classes = "placeholder";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    // Placeholders are fully animated by default
    if !props
        .attributes
        .iter()
        .any(|attr| attr.name == "data-animation")
    {
        props
            .attributes
            .push(Attribute::new("data-animation", "full", None, true));
    }

    rsx! {
        div { ..props.attributes }
    }
}
