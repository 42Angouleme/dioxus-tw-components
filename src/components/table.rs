use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct TableProps {
    #[props(extends = table, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Table(mut props: TableProps) -> Element {
    let default_classes = "table";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        table { ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct TableHeaderProps {
    #[props(extends = thead, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn TableHeader(mut props: TableHeaderProps) -> Element {
    let default_classes = "table-header";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        thead { ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct TableBodyProps {
    #[props(extends = tbody, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn TableBody(mut props: TableBodyProps) -> Element {
    let default_classes = "table-body";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        tbody { ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct TableFooterProps {
    #[props(extends = tfoot, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn TableFooter(mut props: TableFooterProps) -> Element {
    let default_classes = "table-footer";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        tfoot { ..props.attributes,{props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct TableRowProps {
    #[props(extends = tr, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn TableRow(mut props: TableRowProps) -> Element {
    let default_classes = "table-row";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        tr { ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct TableHeadProps {
    #[props(extends = th, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn TableHead(mut props: TableHeadProps) -> Element {
    let default_classes = "table-head";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        th { ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct TableCellProps {
    #[props(extends = td, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn TableCell(mut props: TableCellProps) -> Element {
    let default_classes = "table-cell";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        td { ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct TableCaptionProps {
    #[props(extends = caption, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn TableCaption(mut props: TableCaptionProps) -> Element {
    let default_classes = "table-caption";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    rsx! {
        caption { ..props.attributes, {props.children} }
    }
}
