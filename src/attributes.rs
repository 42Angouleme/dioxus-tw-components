use std::ops::Neg;

use dioxus::{prelude::IntoAttributeValue, signals::Signal};

pub trait BaseClass {
    fn base(&self) -> &'static str;
}

#[derive(Default, Clone, Copy, PartialEq)]
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

pub trait Colorable {
    fn color(&self) -> &'static str;
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

pub trait Sizable {
    fn size(&self) -> &'static str;
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

pub trait Orientable {
    fn orientation(&self) -> &'static str;
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Animation {
    None,
    Light,
    #[default]
    Full,
    Custom(&'static str),
}

pub trait Animatable {
    fn animation(&self) -> &'static str;
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Side {
    #[default]
    Left,
    Right,
}

pub trait Variation {
    fn variant(&self) -> &'static str;
}

// TODO use this everywhere we can
#[derive(Clone, Copy, Debug)]
pub enum DataStateAttrValue {
    Active,
    Inactive,
}

impl DataStateAttrValue {
    pub fn state_attribute_to_str(&self) -> &'static str {
        match self {
            DataStateAttrValue::Active => "active",
            DataStateAttrValue::Inactive => "inactive",
        }
    }

    pub fn is_active(&self) -> bool {
        match self {
            DataStateAttrValue::Active => true,
            DataStateAttrValue::Inactive => false,
        }
    }
}

impl IntoAttributeValue for DataStateAttrValue {
    fn into_value(self) -> dioxus::prelude::dioxus_core::AttributeValue {
        dioxus::prelude::dioxus_core::AttributeValue::Text(
            self.state_attribute_to_str().to_string(),
        )
    }
}

impl Neg for DataStateAttrValue {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            DataStateAttrValue::Active => DataStateAttrValue::Inactive,
            DataStateAttrValue::Inactive => DataStateAttrValue::Active,
        }
    }
}

pub trait DataState: IntoAttributeValue + Clone {
    fn get_state_attr_value(&self) -> DataStateAttrValue;

    fn is_active(&self) -> bool;
}

pub trait HasStateAttr<T> {
    fn add_datastate_to_attr(&mut self, state: Signal<T>);
}
