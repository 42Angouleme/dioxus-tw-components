use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct NavbarProps {
    #[props(extends = nav, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Navbar(mut props: NavbarProps) -> Element {
    let default_classes = "navbar";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        nav { ..props.attributes, {props.children} }
    }
}
