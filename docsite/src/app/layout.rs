use dioxus::prelude::*;
use dioxus_components::{
    molecules::{LightSwitch, Navbar},
    templates::HeaderTemplate,
};

use crate::app::router::Route;

#[component]
pub fn Header() -> Element {
    rsx!(
        HeaderTemplate { class: "flex justify-center", Navbar { 
            div { class: "flex flex-1 space-x-2 items-center ml-6",
                Link { class: "mr-6", to: "/",
                    div { class: "font-bold", "Dioxus Lib Comp" }
                }
                ul { class: "text-sm list-none flex space-x-2",
                    li {
                        Link { class: "anchor", to: "/components/", "Components" }
                    }
                    li {
                        Link { class: "anchor", to: "/theme", "Theme" }
                    }
                }
            }
            div { class: "flex flex-1 items-center justify-end space-x-2 mr-6",
                DioxusLink {}
                GithubLink {}
                LightSwitch { class: "size-10 p-2 flex items-center justify-center rounded-global-radius hover:bg-foreground/30" }
            }
        } }
        main { class: "flex justify-center w-full min-h-screen mt-12", Outlet::<Route> {} }
    )
}

fn DioxusLink() -> Element {
    rsx!(
        a {
            class: "flex justify-center items-center size-10 rounded-global-radius hover:bg-foreground/30",
            href: "https://dioxuslabs.com/",
            target: "_blank",
            alt: "link to dioxus website",
            img {
                src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4",
                class: "h-8 w-auto",
                alt: "dioxus logo"
            }
        }
    )
}

fn GithubLink() -> Element {
    rsx!(
        a {
            class: "flex justify-center items-center size-10 rounded-global-radius p-2 hover:bg-foreground/30",
            href: "https://github.com/42Angouleme/SSCCE_dioxus",
            target: "_blank",
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                class: "fill-foreground",
                path { d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" }
            }
        }
    )
}
