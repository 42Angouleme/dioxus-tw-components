use dioxus::prelude::*;
use dioxus_components::atoms::Separator;

use crate::app::{router::Route, theme::ThemePicker};

#[component]
pub fn SideBarComponent() -> Element {
    // Empty strings are there to render a Separator
    let components = vec![
        "Button",
        "Buttongroup",
        "Placeholder",
        "",
        "Accordion",
        "Breadcrumb",
        "Carousel",
        "Dropdown",
        "HoverCard",
        "LightSwitch",
        "Modal",
        "ProgressBar",
        "Scrollable",
        "SortedTable",
        "Table",
        "Tabs",
        "Toast",
        "",
        "Checkbox",
        "FormList",
        "Input",
        "Radio",
        "Select",
        "Slider",
        "Textarea",
        "Toggle",
    ];

    rsx!(
        div {
            id: "component-div",
            class: "container grid grid-cols-[220px_minmax(0,1fr)] space-x-10",
            aside { id: "components-list-link", class: "space-y-2",
                for component in components {
                    if component.is_empty() {
                        Separator {}
                    } else {
                        p { class: "anchor",
                            Link { to: format!("/components/{}", component.to_lowercase()), {component} }
                        }
                    }
                }
            }
            ThemePicker {}
            div { id: "component-main", class: "max-w-screen-lg", Outlet::<Route> {} }
        }
    )
}
