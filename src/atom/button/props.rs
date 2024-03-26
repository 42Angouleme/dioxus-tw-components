use self::styling::{BaseClass, Color, Size, Variant};
use crate::*;
use component_derive::Component;

pub use Color::{Accent, Primary, Secondary, Unset};
pub use Size::{Lg, Md, Sm, Xl, Xs};
pub use Variant::{Default, Ghost, Outline};

#[derive(PartialEq, Props, Clone, Component)]
pub struct ButtonProps {
    #[props(optional)]
    onclick: EventHandler<MouseEvent>,
    #[props(default = "button".to_string())]
    r#type: String,
    #[props(default)]
    name: String,
    children: Element,
    // Styling
    #[props(default)]
    class: String,
    #[props(default = Color::Primary)]
    color: Color<ButtonProps>,
    #[props(default)]
    variant: Variant<ButtonProps>,
    #[props(default = Size::Md)]
    size: Size<ButtonProps>,
}

impl Component for ButtonProps {
    fn view(self) -> Element {
        let mut class = class!(BaseClass::<ButtonProps>::Default, self.size, self.class);
        // If variant is not default, use the variant class instead of color
        if self.variant != Variant::Default {
            class = class!(class, self.variant);
        } else {
            class = class!(class, self.color);
        }
        rsx!(
            button { class: "{class}", onclick: move |e| { self.onclick.call(e) }, {self.children} }
        )
    }
}

pub enum ButtonOutline {
    Off,
    On,
}
