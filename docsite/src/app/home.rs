use dioxus::prelude::*;
use dioxus_components::{
    attributes::*,
    atoms::*,
    molecules::*,
    organisms::*,
    theme::*
};

#[component]
pub fn HomePage() -> Element {
    let mut theme_manager = use_context::<Signal<ThemeManager>>();
    let current_theme = theme_manager.read().current_theme;

    rsx!(
        section { class: "w-full mx-auto flex flex-col justify-between items-center flex-1 px-4",
            div { class: "flex w-full max-w-(--breakpoint-xl) flex-col text-cente gap-2 justify-evenly",
                div { class: "flex flex-col-reverse lg:flex-row items-center justify-end lg:justify-between lg:flex-1 flex-none",
                    div { class: "text-center lg:text-left lg:flex-1",
                        div { class: "text-[2.5em] md:text-[3.5em] font-semibold text-ghdarkmetal font-sans leading-snug text-balance",
                            span { "Dioxus Components Library" }
                        }
                        h3 { class: "text-[1.25em] font-light text-ghdarkmetal max-w-(--breakpoint-sm) md:max-w-(--breakpoint-md) md:text-left text-center flex flex-col",
                            span { class: "max-w-(--breakpoint-md) leading-loose",
                                "A simple but highly customizable and efficient components library for Dioxus 0.6 based on Tailwind."
                            }
                        }
                        div { class: "pt-8 lg:pt-16 text-[1em] flex flex-row space-x-4 mx-auto lg:mx-0 justify-center lg:justify-start",
                            Link {
                                to: "/components/button",
                                Button {
                                    color: Color::Default,
                                    size: Size::Lg,
                                    animation: Animation::Full,
                                    "Explore"
                                }
                            }
                            Link {
                                to: "https://github.com/42Angouleme/dioxus-components",
                                Button {
                                    color: Color::Default,
                                    variant: ButtonVariant::Outline,
                                    size: Size::Lg,
                                    animation: Animation::Full,
                                    "See sources"
                                }
                            }
                        }
                    }
                    div { class: "lg:pb-12 h-screen max-h-40 lg:max-h-80 my-8",
                        img {
                            src: if current_theme == 0 { asset!("/assets/multiplatform-dark.svg") } else { asset!("/assets/multiplatform-light.svg") },
                            class: "w-full h-full",
                            alt: "Animated Icon",
                        }
                    }
                }
                div { class: "flex mt-20 items-center justify-center",
                    Carousel { class: "w-[75%]", is_circular: true,
                        CarouselTrigger { next: false }
                        CarouselWindow {
                            CarouselContent { id: "home-components-preview", class: "align-middle",
                                CarouselItem { item_key: 0, class: "bg-linear-to-r from-foreground/10 to-foreground/30",
                                    div { class: "flex flex-row h-[6.5rem] items-center justify-center",
                                        div { class: "flex grow items-center justify-center basis 1/3",
                                            Button { "Button" }
                                        }
                                        div { class: "flex grow items-center justify-center basis 1/3",
                                            Toggle {}
                                        }
                                        div { class: "flex grow items-center justify-center basis 1/3",
                                            Slider { class: "w-24" }
                                        }
                                    }
                                }
                                CarouselItem { item_key: 1, class: "bg-linear-to-r from-foreground/30 via-foreground/20 to-foreground/30",
                                    div { class: "flex flex-row h-[6.5rem] items-center justify-center",
                                        div { class: "flex grow items-center justify-center basis 1/3",
                                            Dropdown { id: "dropdown-preview-home",
                                                DropdownToggle {
                                                    id: "dropdown-toggle-preview-home",
                                                    "Dropdown"
                                                }
                                                DropdownContent {
                                                    id: "dropdown-content-preview-home",
                                                    div { "Content" }
                                                }
                                            }
                                        }
                                        div { class: "flex grow items-center justify-center basis 1/3",
                                            LightSwitch { class: "px-4 py-2 text-sm font-medium bg-background rounded-global-radius whitespace-nowrap hover:bg-accent hover:text-accent-foreground cursor-pointer p-1 rounded-global-radius hover:bg-foreground/40 active:bg-foreground/60",
                                                onclick: move |_| {}
                                            }
                                        }
                                        div { class: "flex grow items-center justify-center basis 1/3",
                                            HoverCard { id: "hover-card-preview-home",
                                                HoverCardTrigger {
                                                    id: "hover-card-trigger-preview-home",
                                                    div { class: "px-4 py-2 text-sm font-medium bg-background border border-input rounded-global-radius whitespace-nowrap hover:bg-accent hover:text-accent-foreground",
                                                        "Hover me"
                                                    }
                                                }
                                                HoverCardContent {
                                                    id: "hover-card-content-preview-home",
                                                    div { "Content" }
                                                }
                                            }
                                        }
                                    }
                                }
                                CarouselItem { item_key: 2, class: "bg-linear-to-r from-foreground/30 to-foreground/10",
                                    div { class: "flex flex-row h-[6.5rem] items-center justify-center",
                                        div { class: "flex grow items-center justify-center basis 1/3",
                                            ButtonGroup {
                                                ButtonGroupItem { "1" }
                                                ButtonGroupItem { "2" }
                                                ButtonGroupItem { "3" }
                                            }
                                        }
                                        div { class: "flex grow items-center justify-center basis 1/3",
                                            Placeholder {}
                                        }
                                        div { class: "flex grow items-center justify-center basis 1/3",
                                            SelectGroup { class: "w-24",
                                                SelectPlaceholder { "Select" }
                                                SelectLabel { label: "Label 1" }
                                                SelectItem { "Option 1" }
                                                SelectItem { "Option 2" }
                                                SelectItem { "Option 3" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        CarouselTrigger { next: true }
                    }
                }
            }
        }
        div { class: "fixed right-0 top-[15%] pr-20 text-center",
            p { "With a theme" }
            p { "customizer" }
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                width: 35,
                height: 35,
                fill: "none",
                class: "stroke-foreground fixed right-14",
                stroke_width: 1,
                stroke_linecap: "round", 
                stroke_linejoin: "round",
                path { d: "M7 7L17 17M17 17V7M17 17H7" }
            }
        }
    )
}
