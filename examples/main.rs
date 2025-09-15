use dioxus::prelude::*;
use dioxus_tw_components::*;

static TAILWIND_CSS: Asset = asset!("/examples/assets/tailwind.css");

fn main() {
    dioxus::launch(AppBootstrap);
}

#[component]
fn AppBootstrap() -> Element {
    rsx! {
        Toaster {
            App {}
        }
    }
}

#[component]
fn App() -> Element {
    let page_number = use_signal(|| 1_usize);
    let page_number_outlined = use_signal(|| 1_usize);
    let page_number_ghost = use_signal(|| 1_usize);
    let mut toast = use_toast();

    let elem_to_repeat = rsx! {
        div { class: "space-y-2 mx-1",
            "Field"
            Input {}
            Input {}
            Separator { class: "" }
        }
    };

    let list_fields = (0..6)
        .map(|_| elem_to_repeat.clone())
        .collect::<Vec<Element>>();

    rsx! {
        document::Stylesheet { href: TAILWIND_CSS }
        Bootstrap {}
        div { class: "flex flex-col space-y-2 p-4 w-fit mx-auto text-foreground justify-center items-center",
            Button {
                "data-style": "primary",
                "data-size": "lg",
                "data-animation": "full",
                "test"
            }
            Button {
                "data-style": "secondary",
                "data-size": "lg",
                "data-animation": "full",
                "test"
            }
            Button {
                "data-style": "destructive",
                "data-size": "lg",
                "data-animation": "full",
                "test"
            }
            Button {
                "data-style": "success",
                "data-size": "lg",
                "data-animation": "full",
                "test"
            }
            Button {
                "data-style": "primary",
                "data-variant": "outline",
                "data-size": "lg",
                "data-animation": "full",
                "test"
            }
            Button {
                "data-style": "primary",
                "data-variant": "ghost",
                "data-size": "lg",
                "data-animation": "full",
                "test"
            }
            Icon { class: "text-5xl mx-auto pb-10 border-b-1 border-white", icon: Icons::Discord }
            Placeholder { "data-animation": "full" }
            Placeholder { "data-animation": "light" }
            Checkbox { class: "!mb-2" }
            Separator { class: "!bg-white" }
        }
        div { class: "flex flex-col space-y-2 p-4 w-fit mx-auto text-foreground",
            ButtonGroup {
                Button {
                    "data-style": "primary",
                    "data-size": "lg",
                    "data-animation": "full",
                    "test"
                }
                Button {
                    "data-style": "secondary",
                    "data-size": "lg",
                    "data-animation": "full",
                    "test"
                }
                Button {
                    "data-style": "destructive",
                    "data-size": "lg",
                    "data-animation": "full",
                    "test"
                }
                Button {
                    "data-style": "success",
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
        div { class: "flex flex-col space-y-4 min-w-96 w-fit mt-6 mx-auto pb-10 border-b-1 border-white",
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
        div { class: "flex flex-col space-y-4 min-w-96 w-fit mt-6 mx-auto pb-10 border-b-1 border-white",
            Carousel {
                is_circular: true,
                autoscroll_duration: 1000,
                CarouselTrigger { next: false }
                CarouselWindow {
                    CarouselContent { id: "carousel-prev", class: "h-32",
                        "data-animation": "full",
                        CarouselItem { item_key: 0,
                            class: "bg-primary/40",
                            div { class: "flex items-center justify-center h-full w-full font-bold text-foreground",
                                "Item 1"
                            }
                        }
                        CarouselItem { item_key: 1,
                            class: "bg-gradient-to-r from-primary/40 to-secondary/40",
                            div { class: "flex items-center justify-center h-full w-full font-bold text-foreground",
                                "Item 2"
                            }
                        }
                        CarouselItem { item_key: 2,
                            class: "bg-secondary/40",
                            div { class: "flex items-center justify-center h-full w-full font-bold text-foreground",
                                "Item 3"
                            }
                        }
                    }
                }
                CarouselTrigger { next: true }
            }
        }
        div { class: "flex flex-col justify-start items-center space-y-4 min-w-96 min-h-64 w-fit mt-6 mx-auto pb-10 border-b-1 border-white",
            Dropdown {
                DropdownToggle {
                    id: "dropdown-toggle-demo",
                    class: "!p-4",
                    "Dropdown"
                }
                DropdownContent {
                    "data-animation": "full",
                    div { "Content" }
                    div { "Content" }
                    Separator {}
                    div { "Content" }
                    div { "Content" }
                }
            }
        }
        div { class: "flex flex-col space-y-4 min-w-96 w-fit min-h-36 mt-6 mx-auto pb-10 border-b-1 border-white items-center",
            HoverCard { id: "hover-card-demo",
                HoverCardTrigger {
                    id: "hover-card-trigger-demo",
                    "Hover me"
                }
                HoverCardContent {
                    id: "hover-card-content-demo",
                    "data-animation": "full",
                    div { "Content" }
                }
            }
        }
        div { class: "flex flex-col space-y-4 min-w-96 w-fit mt-6 mx-auto pb-10 border-b-1 border-white",
            LightSwitch {}
        }
        div { class: "hidden flex flex-col space-y-4 min-w-96 w-fit mt-6 mx-auto pb-10 border-b-1 border-white",
            Modal {
                ModalTrigger { "OpenModal" }
                ModalBackground {
                    "data-style": "secondary",
                    "data-animation": "full",
                }
                ModalContent {
                    div {
                        ModalClose {}
                    }
                    div { class: "h4", "TITLE" }
                    div { class: "paragraph", "CONTENT" }
                }
            }
        }
        div { class: "flex flex-col space-y-4 min-w-96 w-fit mt-6 mx-auto pb-10 border-b-1 border-white",
            div { class: "flex flex-col space-y-8 w-full h-fit",
                Pagination {
                    class: "p-2",
                    "data-style": "primary",
                    data_size: 1000_usize,
                    page_size: 25_usize,
                    page_number,
                }

                Pagination {
                    class: "p-2",
                    "data-style": "secondary",
                    "data-variant": "outline",
                    data_size: 1000_usize,
                    page_size: 25_usize,
                    page_number: page_number_outlined,
                }

                Pagination {
                    class: "p-2",
                    "data-style": "destructive",
                    "data-variant": "ghost",
                    data_size: 1000_usize,
                    page_size: 25_usize,
                    page_number: page_number_ghost,
                }
            }
        }
        div { class: "flex flex-col space-y-4 min-w-96 w-fit mt-6 mx-auto pb-10 border-b-1 border-white",
            div { class: "border rounded-global-radius max-w-96 min-w-64 h-fit",
                ProgressBar {
                    "data-style": "destructive",
                    ProgressBarInner {
                        progress: 50,
                    }
                }
            }
        }
        div { class: "flex flex-col space-y-4 min-w-96 w-fit h-72 mt-6 mx-auto pb-10 border-b-1 border-white",
            Scrollable {
                div { class: "flex flex-col space-y-2 text-sm font-medium",
                    p { class: "paragraph", "About Us" }
                    p { class: "paragraph", "Our Mission" }
                    p { class: "paragraph", "Our Team" }
                    Separator {}
                    p { class: "paragraph", "Services" }
                    p { class: "paragraph", "Service 1" }
                    p { class: "paragraph", "Service 2" }
                    p { class: "paragraph", "Service 3" }
                    p { class: "paragraph", "Products" }
                    p { class: "paragraph", "Product 1" }
                    p { class: "paragraph", "Product 2" }
                    p { class: "paragraph", "Product 3" }
                    Separator {}
                    p { class: "paragraph", "Testimonials" }
                    p { class: "paragraph", "Testimonial 1" }
                    p { class: "paragraph", "Testimonial 2" }
                    p { class: "paragraph", "Testimonial 3" }
                    Separator {}
                    p { class: "paragraph", "Contact Us" }
                    p { class: "paragraph", "Email: info@example.com" }
                    p { class: "paragraph", "Phone: +1234567890" }
                    p { class: "paragraph", "Address: 123 Main St, City, Country" }
                }
            }
        }
        div { class: "flex flex-col space-y-4 min-w-96 w-fit mt-6 mx-auto pb-10 border-b-1 border-white",
            SidePanel {
                SidePanelTrigger { "OpenSidePanel" }
                SidePanelBackground {
                    "data-animation": "full",
                }
                SidePanelContent {
                    "data-animation": "full",
                    div {
                        SidePanelClose {}
                    }
                    div { class: "h4 mb-6", "Title" }
                    div {
                        class: "paragraph",
                        r#"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt
                        ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
                        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
                        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat
                        non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
                        "#
                    }
                }
            }
        }
        div { class: "flex flex-col space-y-4 min-w-96 w-fit mt-6 mx-auto pb-10 border-b-1 border-white",
            div { class: "max-w-96 border rounded-global-radius bg-background p-1",
                Table {
                    TableCaption { "Product Inventory" }
                    TableHeader {
                        TableRow {
                            TableHead { "Product Name" }
                            TableHead { "Quantity" }
                            TableHead { "Price" }
                        }
                    }
                    TableBody {
                        TableRow {
                            TableCell { "Widget A" }
                            TableCell { "100" }
                            TableCell { "$1.99" }
                        }
                        TableRow {
                            TableCell { "Widget B" }
                            TableCell { "50" }
                            TableCell { "$2.99" }
                        }
                        TableRow {
                            TableCell { "Widget C" }
                            TableCell { "25" }
                            TableCell { "$3.99" }
                        }
                        TableRow {
                            TableCell { "Widget D" }
                            TableCell { "75" }
                            TableCell { "$4.99" }
                        }
                        TableRow {
                            TableCell { "Widget E" }
                            TableCell { "125" }
                            TableCell { "$5.99" }
                        }
                    }
                    TableFooter {
                        TableRow {
                            TableCell { "Total" }
                            TableCell { "350" }
                            TableCell { "$27.91" }
                        }
                    }
                }
            }
        }
        div { class: "flex flex-col space-y-4 min-w-96 w-fit mt-6 mx-auto pb-10 border-b-1 border-white",
            div { class: "min-h-64 items-start",
                Tabs {
                    default_tab: "tabs-0",
                    TabsList {
                        TabsTrigger { id: "tabs-0", "Home" }
                        TabsTrigger { id: "tabs-1", "About" }
                        TabsTrigger { id: "tabs-2", "Contact" }
                    }
                    TabsContent { id: "tabs-0", class: "space-y-4",
                        h4 { class: "h4 text-foreground", "Welcome to our home page!" }
                        p { class: "paragraph text-foreground",
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nunc vel tincidunt lacinia, nisl nunc aliquet nisl, vel aliquet nunc nisl vel nisl. Nulla facilisi."
                        }
                    }
                    TabsContent { id: "tabs-1", class: "space-y-4",
                        h4 { class: "h4 text-foreground", "Learn more about us here." }
                        p { class: "paragraph text-foreground",
                            "Vivamus eget nisl velit. Sed euismod, nunc vel tincidunt lacinia, nisl nunc aliquet nisl, vel aliquet nunc nisl vel nisl. Nulla facilisi."
                        }
                    }
                    TabsContent { id: "tabs-2", class: "space-y-4",
                        h4 { class: "h4 text-foreground", "Get in touch with us using the form below." }
                        p { class: "paragraph text-foreground",
                            "Praesent eget nisl velit. Sed euismod, nunc vel tincidunt lacinia, nisl nunc aliquet nisl, vel aliquet nunc nisl vel nisl. Nulla facilisi."
                        }
                    }
                }
            }
        }
        div { class: "flex flex-col space-y-4 min-w-96 w-fit mt-6 mx-auto pb-10 border-b-1 border-white",
            Button {
                onclick: move |_| {
                },
                "Toasting"
            }
        }
        div { class: "flex flex-col space-y-4 min-w-96 w-fit mt-6 mx-auto pb-10 border-b-1 border-white",
            FormList {
                FormListTriggerMinus {
                    Button { "-" }
                }
                FormListTriggerPlus {
                    Button { "+" }
                }
                FormListContent { list_fields }
            }
        }
        div { class: "flex flex-col space-y-4 min-w-96 w-fit mt-6 mx-auto pb-10 border-b-1 border-white",
            Input {
                "data-style": "primary",
                placeholder: "Input",
            }
        }
        div { class: "flex flex-col space-y-4 min-w-96 w-fit mt-6 mx-auto pb-10 border-b-1 border-white",
            div { class: "flex space-x-2",
                for i in 0..3 {
                    div { class: "flex flex-col items-center space-y-2",
                        p { class: "text-foreground font-bold", "{i}" }
                        Radio {
                            "data-style": "destructive",
                        }
                    }
                }
            }
        }
        div { class: "flex flex-col space-y-4 min-w-96 w-fit mt-6 mx-auto pb-10 border-b-1 border-white",
            SelectGroup {
                SelectPlaceholder { "Select an option" }
                SelectLabel { label: "Label 1" }
                SelectItem { "Option 1" }
                SelectItem { "Option 2" }
                SelectItem { "Option 3" }
            }
        }
        div { class: "flex flex-col space-y-4 min-w-96 w-fit mt-6 mx-auto pb-10 border-b-1 border-white",
            Slider {}
        }
        div { class: "flex flex-col space-y-4 min-w-96 w-fit mt-6 mx-auto pb-10 border-b-1 border-white",
            TextArea {}
        }
        div { class: "flex flex-col space-y-4 min-w-96 w-fit mt-6 mx-auto pb-10 border-b-1 border-white",
            Toggle {
                "data-animation": "full",
            }
        }
    }
}
