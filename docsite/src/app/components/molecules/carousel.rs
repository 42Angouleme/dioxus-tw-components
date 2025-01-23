use dioxus::prelude::*;
use dioxus_components::molecules::carousel::*;

use crate::app::{components::preview::*, doctrait::DemoComponent};

pub fn CarouselPage() -> Element {
    let _state = use_context_provider(|| {
        let mut hash = HashPreview::new();
        hash.insert(0, FieldPreview::default().class("w-96".to_string()));
        hash.insert(
            1,
            FieldPreview::default().class("bg-primary/40".to_string()),
        );
        hash.insert(
            2,
            FieldPreview::default()
                .class("bg-linear-to-r from-primary/40 to-secondary/40".to_string()),
        );
        hash.insert(
            3,
            FieldPreview::default().class("bg-secondary/40".to_string()),
        );
        Signal::new(hash)
    });

    rsx!(
        PreviewFull::<CarouselProps> {}
    )
}

impl DemoComponent for CarouselProps {
    fn comp_introduction() -> &'static str {
        "A carousel to display multiple items and navigate through them"
    }

    fn BuildCompPreview() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            Carousel { class: state.read()[&0].get_class(),
                CarouselTrigger { next: false }
                CarouselWindow {
                    CarouselContent { id: "carousel-prev", class: "h-32",
                        CarouselItem { item_key: 0, class: state.read()[&1].get_class(),
                            div { class: "flex items-center justify-center h-full w-full font-bold text-foreground",
                                "Item 1"
                            }
                        }
                        CarouselItem { item_key: 1, class: state.read()[&2].get_class(),
                            div { class: "flex items-center justify-center h-full w-full font-bold text-foreground",
                                "Item 2"
                            }
                        }
                        CarouselItem { item_key: 2, class: state.read()[&3].get_class(),
                            div { class: "flex items-center justify-center h-full w-full font-bold text-foreground",
                                "Item 3"
                            }
                        }
                    }
                }
                CarouselTrigger { next: true }
            }
        )
    }

    fn BuildCompSelectors() -> Element {
        let state = use_context::<Signal<HashPreview>>();

        rsx!(
            CompPreviewSelector::<CarouselProps> { index: 0, state, comp_props: CarouselProps::default() }
            CompPreviewSelector::<CarouselItemProps> {
                index: 1,
                state,
                comp_props: CarouselItemProps::default(),
            }
            CompPreviewSelector::<CarouselItemProps> {
                index: 2,
                state,
                comp_props: CarouselItemProps::default(),
            }
            CompPreviewSelector::<CarouselItemProps> {
                index: 3,
                state,
                comp_props: CarouselItemProps::default(),
            }
        )
    }
}
