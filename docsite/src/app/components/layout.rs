use dioxus::prelude::*;
use dioxus_components::{atoms::Separator, molecules::Scrollable};

use crate::app::router::Route;

#[component]
pub fn SideBarComponent() -> Element {
    // Use empty strings to render separator
    let components = vec![
        "Button",
        "Buttongroup",
        "Placeholder",
        "",
        "LightSwitch",
        "Tabs",
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
        ComponentPage { 
            SideBarTemplate { 
                ul { id: "component-list", class: "pl-2 space-y-1",
                    for component in components {
                        if component.is_empty() {
                            Separator {}
                        } else {
                            li { class: "anchor",
                                Link { to: format!("/components/{}", component.to_lowercase()), {component} }
                            }
                        }
                    }
                }
            }
            DocTemplate { Outlet::<Route> {} }
        }
    )
}

#[component]
pub fn SideBarTemplate(children: Element) -> Element {
    rsx!(
        aside { class: "max-w-[220px] border border-black flex flex-col", Scrollable { {children} } }
    )
}

#[component]
pub fn ComponentPage(children: Element) -> Element {
    rsx!(
        div { class: "grid grid-cols-[220px_minmax(0,1fr)] max-w-screen-xl border border-primary min-h-full",
            {children}
        }
    )
}

#[component]
pub fn DocTemplate(children: Element) -> Element {
    rsx!(
        div { class: "border border-secondary", {children} }
    )
}
