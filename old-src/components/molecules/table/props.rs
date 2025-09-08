use crate::attributes::*;
use dioxus::prelude::*;
use dioxus_tw_components_macro::UiComp;

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct TableProps {
    #[props(extends = table, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for TableProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn Table(mut props: TableProps) -> Element {
    props.update_class_attribute();

    rsx! {
        table { ..props.attributes,{props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct TableHeaderProps {
    #[props(extends = thead, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for TableHeaderProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn TableHeader(mut props: TableHeaderProps) -> Element {
    props.update_class_attribute();

    rsx! {
        thead { ..props.attributes,{props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct TableBodyProps {
    #[props(extends = tbody, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for TableBodyProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn TableBody(mut props: TableBodyProps) -> Element {
    props.update_class_attribute();

    rsx! {
        tbody { ..props.attributes,{props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct TableFooterProps {
    #[props(extends = tfoot, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for TableFooterProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn TableFooter(mut props: TableFooterProps) -> Element {
    props.update_class_attribute();

    rsx! {
        tfoot { ..props.attributes,{props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct TableHeadProps {
    #[props(extends = th, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    onclick: EventHandler<MouseEvent>,

    children: Element,
}

impl std::default::Default for TableHeadProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            onclick: EventHandler::<MouseEvent>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn TableHead(mut props: TableHeadProps) -> Element {
    props.update_class_attribute();

    let onclick = move |event| props.onclick.call(event);

    rsx! {
        th { onclick, ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct TableRowProps {
    #[props(extends = tr, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for TableRowProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn TableRow(mut props: TableRowProps) -> Element {
    props.update_class_attribute();

    rsx! {
        tr { ..props.attributes,{props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct TableCellProps {
    #[props(extends = td, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for TableCellProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn TableCell(mut props: TableCellProps) -> Element {
    props.update_class_attribute();

    rsx! {
        td { ..props.attributes,{props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct TableCaptionProps {
    #[props(extends = caption, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for TableCaptionProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn TableCaption(mut props: TableCaptionProps) -> Element {
    props.update_class_attribute();

    rsx! {
        caption { ..props.attributes,{props.children} }
    }
}
