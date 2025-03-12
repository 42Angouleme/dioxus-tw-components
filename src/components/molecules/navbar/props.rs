use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_tw_components_macro::UiComp;

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct NavbarProps {
    #[props(extends = nav, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for NavbarProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

pub fn Navbar(mut props: NavbarProps) -> Element {
    props.update_class_attribute();

    rsx! {
        nav { ..props.attributes,{props.children} }
    }
}
