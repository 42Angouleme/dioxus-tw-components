use dioxus::prelude::*;
use dioxus_tw_components::*;

static TAILWIND_CSS: Asset = asset!("/examples/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: TAILWIND_CSS }
        Bootstrap {}
        div { class: "flex flex-col space-y-2 p-4 w-fit mx-auto text-foreground",
            Button {
                "data-color": "primary",
                "data-size": "lg",
                "data-animation": "full",
                "test"
            }
            Button {
                "data-color": "secondary",
                "data-size": "lg",
                "data-animation": "full",
                "test"
            }
            Button {
                "data-color": "destructive",
                "data-size": "lg",
                "data-animation": "full",
                "test"
            }
            Button {
                "data-color": "success",
                "data-size": "lg",
                "data-animation": "full",
                "test"
            }
            Button {
                "data-color": "primary",
                "data-variant": "outline",
                "data-size": "lg",
                "data-animation": "full",
                "test"
            }
            Button {
                "data-color": "primary",
                "data-variant": "ghost",
                "data-size": "lg",
                "data-animation": "full",
                "test"
            }
            Icon { class: "text-5xl mx-auto", icon: Icons::Discord }
            Placeholder { "data-animation": "full" }
            Placeholder { "data-animation": "light" }
            Separator { class: "!bg-white" }
        }
        div { class: "flex flex-col space-y-2 p-4 w-fit mx-auto text-foreground",
            ButtonGroup {
                Button {
                    "data-color": "primary",
                    "data-size": "lg",
                    "data-animation": "full",
                    "test"
                }
                Button {
                    "data-color": "secondary",
                    "data-size": "lg",
                    "data-animation": "full",
                    "test"
                }
                Button {
                    "data-color": "destructive",
                    "data-size": "lg",
                    "data-animation": "full",
                    "test"
                }
                Button {
                    "data-color": "success",
                    "data-size": "lg",
                    "data-animation": "full",
                    "test"
                }
            }
        }
        div { class: "flex flex-col space-y-2 p-4 max-w-96 mx-auto text-foreground",
            Accordion {
                AccordionItem {
                    AccordionTrigger { id: "accordion-1", "Products" }
                    AccordionContent { id: "accordion-1",
                        p { "Check out our latest products!" }
                    }
                }
                AccordionItem {
                    AccordionTrigger { id: "accordion-2", "Services" }
                    AccordionContent { id: "accordion-2", "data-animation": "full",
                        p { "Discover our range of services." }
                    }
                }
                AccordionItem {
                    AccordionTrigger { id: "accordion-3", "Testimonials" }
                    AccordionContent { id: "accordion-3", "data-animation": "full",
                        p {
                            "I've been a customer for over a year now and I'm extremely satisfied with their services. The team is always responsive and goes above and beyond to ensure my needs are met. Their attention to detail is impressive and I highly recommend them to anyone looking for top-notch service."
                        }
                    }
                }
            }
        }
        div { class: "flex flex-col space-y-4 sm:w-2/3 mt-6 mx-auto",
            Callout { title: "Note", "data-variant": "note",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nunc vel tincidunt lacinia, nisl nunc aliquet nisl, vel aliquet nunc nisl vel nisl"
            }

            Callout { title: "Tip", "data-variant": "tip",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nunc vel tincidunt lacinia, nisl nunc aliquet nisl, vel aliquet nunc nisl vel nisl"
            }

            Callout { title: "Warning", "data-variant": "warning",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nunc vel tincidunt lacinia, nisl nunc aliquet nisl, vel aliquet nunc nisl vel nisl"
            }

            Callout { title: "Caution", "data-variant": "caution",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nunc vel tincidunt lacinia, nisl nunc aliquet nisl, vel aliquet nunc nisl vel nisl"
            }

            Callout {
                title: "Custom callout with custom icon",
                icon: Icons::LunchDining,
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nunc vel tincidunt lacinia, nisl nunc aliquet nisl, vel aliquet nunc nisl vel nisl"
            }
        }
        div { class: "flex flex-col space-y-4 sm:w-2/3 mt-6 mx-auto",
            Carousel {
                is_circular: true,
                autoscroll_duration: 1000,
                CarouselTrigger { next: false }
                CarouselWindow {
                    CarouselContent { id: "carousel-prev", class: "h-32",
                        CarouselItem { item_key: 0,
                            div { class: "flex items-center justify-center h-full w-full font-bold text-foreground",
                                "Item 1"
                            }
                        }
                        CarouselItem { item_key: 1,
                            div { class: "flex items-center justify-center h-full w-full font-bold text-foreground",
                                "Item 2"
                            }
                        }
                        CarouselItem { item_key: 2,
                            div { class: "flex items-center justify-center h-full w-full font-bold text-foreground",
                                "Item 3"
                            }
                        }
                    }
                }
                CarouselTrigger { next: true }
            }
        }
    }
}
