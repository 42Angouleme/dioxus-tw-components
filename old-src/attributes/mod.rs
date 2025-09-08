use dioxus::dioxus_core::{Attribute, AttributeValue, Element};
use std::str::FromStr;
use tailwind_fuse::tw_merge;

pub trait UiComp: HasChildren + BuildClass + std::fmt::Display {}

// This trait is used for the docsite
pub trait HasChildren {
    fn has_children(&self) -> bool {
        false
    }

    fn set_children(&mut self, _children: Element) {}
}

pub trait Class {
    fn base(&self) -> &'static str {
        ""
    }

    fn color(&self) -> Option<&'static str> {
        None
    }

    fn size(&self) -> Option<&'static str> {
        None
    }

    fn variant(&self) -> Option<&'static str> {
        None
    }

    fn animation(&self) -> Option<&'static str> {
        None
    }

    fn orientation(&self) -> Option<&'static str> {
        None
    }

    fn side(&self) -> Option<&'static str> {
        None
    }
}

pub trait BuildClass: Class {
    fn build_class(&self) -> String {
        let mut class = String::from(self.base());

        if let Some(color) = self.color() {
            class.push(' ');
            class.push_str(color);
        }

        if let Some(size) = self.size() {
            class.push(' ');
            class.push_str(size);
        }

        if let Some(variant) = self.variant() {
            class.push(' ');
            class.push_str(variant);
        }

        if let Some(animation) = self.animation() {
            class.push(' ');
            class.push_str(animation);
        }

        if let Some(orientation) = self.orientation() {
            class.push(' ');
            class.push_str(orientation);
        }

        if let Some(side) = self.side() {
            class.push(' ');
            class.push_str(side);
        }

        tw_merge!(class)
    }

    fn update_class_attribute(&mut self) {
        let class = self.build_class();

        // If the component have a vec attributes
        if let Some(vec_attributes) = self.get_attributes() {
            // Find the class attribute in the vec and modify it
            if let Some(class_attribute) =
                vec_attributes.iter_mut().find(|attr| attr.name == "class")
            {
                if let AttributeValue::Text(ref mut value) = class_attribute.value {
                    *value = tw_merge!(class, value.clone());
                }
            } else {
                // Else push the class attribute in the vec
                vec_attributes.push(Attribute::new("class", class, None, true));
            }
        }
    }

    fn get_attributes(&mut self) -> Option<&mut Vec<Attribute>> {
        None
    }

    // All those methods below are used for the docsite
    fn has_color(&self) -> bool {
        self.color().is_some()
    }

    fn set_color(&mut self, _color: Color) {}

    fn has_size(&self) -> bool {
        self.size().is_some()
    }

    fn set_size(&mut self, _size: Size) {}

    fn has_animation(&self) -> bool {
        self.animation().is_some()
    }

    fn set_animation(&mut self, _animation: Animation) {}

    fn has_orientation(&self) -> bool {
        self.orientation().is_some()
    }

    fn set_orientation(&mut self, _orientation: Orientation) {}

    fn has_side(&self) -> bool {
        self.side().is_some()
    }

    fn set_side(&mut self, _side: Side) {}
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Color {
    #[default]
    Default,
    Primary,
    Secondary,
    Destructive,
    Success,
    Accent,
    Muted,
}

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "primary" => Ok(Color::Primary),
            "secondary" => Ok(Color::Secondary),
            "destructive" => Ok(Color::Destructive),
            "success" => Ok(Color::Success),
            "accent" => Ok(Color::Accent),
            "muted" => Ok(Color::Muted),
            _ => Ok(Color::default()),
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Color::Default => "Default",
            Color::Primary => "Primary",
            Color::Secondary => "Secondary",
            Color::Destructive => "Destructive",
            Color::Success => "Success",
            Color::Accent => "Accent",
            Color::Muted => "Muted",
        };
        f.write_str(s)
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Size {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl FromStr for Size {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "xs" => Ok(Size::Xs),
            "sm" => Ok(Size::Sm),
            "md" => Ok(Size::Md),
            "lg" => Ok(Size::Lg),
            "xl" => Ok(Size::Xl),
            _ => Ok(Size::default()),
        }
    }
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Size::Xs => "Xs",
            Size::Sm => "Sm",
            Size::Md => "Md",
            Size::Lg => "Lg",
            Size::Xl => "Xl",
        };
        f.write_str(s)
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

impl FromStr for Orientation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "horizontal" => Ok(Orientation::Horizontal),
            _ => Ok(Orientation::Vertical),
        }
    }
}

impl std::fmt::Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Orientation::Vertical => "vertical",
            Orientation::Horizontal => "horizontal",
        };
        f.write_str(s)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Animation {
    None,
    Light,
    #[default]
    Full,
}

impl FromStr for Animation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Animation::None),
            "light" => Ok(Animation::Light),
            _ => Ok(Animation::Full),
        }
    }
}

impl std::fmt::Display for Animation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Animation::None => "None",
            Animation::Light => "Light",
            Animation::Full => "Full",
        };
        f.write_str(s)
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Side {
    Top,
    Bottom,
    Left,
    #[default]
    Right,
}

impl FromStr for Side {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "top" => Ok(Side::Top),
            "bottom" => Ok(Side::Bottom),
            "left" => Ok(Side::Left),
            _ => Ok(Side::Right),
        }
    }
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Side::Top => "Top",
            Side::Bottom => "Bottom",
            Side::Left => "Left",
            Side::Right => "Right",
        };
        f.write_str(s)
    }
}
