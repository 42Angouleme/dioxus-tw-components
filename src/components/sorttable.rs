use crate::components::{icon::*, table::*};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
    None,
}

#[derive(Clone, PartialEq)]
pub struct SortableColumn {
    pub key: String,
    pub title: String,
    pub sortable: bool,
}

#[derive(Clone, PartialEq)]
pub struct SortTableState {
    pub sort_key: String,
    pub sort_direction: SortDirection,
}

impl SortTableState {
    pub fn new() -> Self {
        Self {
            sort_key: String::new(),
            sort_direction: SortDirection::None,
        }
    }

    pub fn sort_by(&mut self, key: String) {
        if self.sort_key == key {
            self.sort_direction = match self.sort_direction {
                SortDirection::Asc => SortDirection::Desc,
                SortDirection::Desc => SortDirection::None,
                SortDirection::None => SortDirection::Asc,
            };
        } else {
            self.sort_key = key;
            self.sort_direction = SortDirection::Asc;
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct SortTableProps {
    #[props(extends = table, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    columns: Vec<SortableColumn>,
    children: Element,
}

#[component]
pub fn SortTable(mut props: SortTableProps) -> Element {
    let default_classes = "sorttable";
    crate::setup_class_attribute(&mut props.attributes, default_classes);

    let state = use_context_provider(|| Signal::new(SortTableState::new()));

    rsx! {
        table { ..props.attributes,
            TableHeader {
                TableRow {
                    for column in &props.columns {
                        SortTableHead {
                            key: "{column.key}",
                            column: column.clone()
                        }
                    }
                }
            }
            {props.children}
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct SortTableHeadProps {
    column: SortableColumn,
}

#[component]
pub fn SortTableHead(props: SortTableHeadProps) -> Element {
    let mut state = use_context::<Signal<SortTableState>>();

    let is_sorted = state.read().sort_key == props.column.key;
    let sort_direction = &state.read().sort_direction;

    let onclick = move |_: MouseEvent| {
        if props.column.sortable {
            state.write().sort_by(props.column.key.clone());
        }
    };

    rsx! {
        TableHead {
            if props.column.sortable {
                div {
                    class: "sorttable-header",
                    "data-sorted": is_sorted,
                    "data-sort": match sort_direction {
                        SortDirection::Asc => "asc",
                        SortDirection::Desc => "desc",
                        SortDirection::None => "none",
                    },
                    onclick,
                    "{props.column.title}"
                    Icon {
                        class: "sorttable-sort-icon",
                        icon: Icons::ExpandMore
                    }
                }
            } else {
                "{props.column.title}"
            }
        }
    }
}
