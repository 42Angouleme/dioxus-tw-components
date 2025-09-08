pub mod bootstrap;
pub use bootstrap::*;

pub mod components;
pub use components::button::*;
pub use components::icon::*;
pub use components::placeholder::*;

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
