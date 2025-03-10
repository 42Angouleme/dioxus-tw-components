use std::str::FromStr;

use super::props::*;
use crate::attributes::*;
use dioxus::prelude::*;

impl Class for CalloutProps {
    fn base(&self) -> &'static str {
        "flex flex-col bg-muted border-l-6 border-solid p-2 pl-4 rounded-global-radius"
    }

    fn variant(&self) -> Option<&'static str> {
        Some(match *self.variant.read() {
            CalloutVariant::Note => "border-primary bg-primary/10 text-primary",
            CalloutVariant::Tip => "border-success bg-success/10 text-success",
            CalloutVariant::Warning => "border-yellow-500 bg-yellow-500/10 text-yellow-500",
            CalloutVariant::Caution => "border-destructive bg-destructive/10 text-destructive",
        })
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum CalloutVariant {
    #[default]
    Note,
    Tip,
    Warning,
    Caution,
}

impl FromStr for CalloutVariant {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tip" => Ok(CalloutVariant::Tip),
            "warning" => Ok(CalloutVariant::Warning),
            "caution" => Ok(CalloutVariant::Caution),
            _ => Ok(CalloutVariant::Note),
        }
    }
}

impl std::fmt::Display for CalloutVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CalloutVariant::Note => "Note",
            CalloutVariant::Tip => "Tip",
            CalloutVariant::Warning => "Warning",
            CalloutVariant::Caution => "Caution",
        };
        f.write_str(s)
    }
}
