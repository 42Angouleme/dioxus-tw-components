pub mod bootstrap;
pub use bootstrap::*;

pub mod components;
pub use components::accordion::*;
pub use components::button::*;
pub use components::buttongroup::*;
pub use components::callout::*;
pub use components::carousel::*;
pub use components::checkbox::*;
pub use components::dropdown::*;
pub use components::formlist::*;
pub use components::header::*;
pub use components::icon::*;
pub use components::input::*;
pub use components::lightswitch::*;
pub use components::modal::*;
pub use components::navbar::*;
pub use components::pagination::*;
pub use components::placeholder::*;
pub use components::progressbar::*;
pub use components::radio::*;
pub use components::scrollable::*;
pub use components::select::*;
pub use components::separator::*;
pub use components::sidepanel::*;
pub use components::slider::*;
pub use components::sorttable::*;
pub use components::table::*;
pub use components::tabs::*;
pub use components::textarea::*;
pub use components::toast::*;
pub use components::toggle::*;

use dioxus::dioxus_core::{Attribute, AttributeValue};

pub(crate) fn setup_class_attribute(attributes: &mut Vec<Attribute>, default_classes: &str) {
    // Find the class attribute in the vec and modify it
    if let Some(class_attribute) = attributes.iter_mut().find(|attr| attr.name == "class") {
        if let AttributeValue::Text(ref mut value) = class_attribute.value {
            *value = format!("{} {}", default_classes, value.clone());
        }
    } else {
        // Else push the class attribute in the vec
        attributes.push(Attribute::new("class", default_classes, None, true));
    }
}
