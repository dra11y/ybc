use derive_more::Display;
use std::borrow::Cow;
use yew::html::IntoPropValue;

/// Common alignment classes.
#[derive(Clone, Debug, Display, PartialEq, Eq)]
#[display("is-{}")]
pub enum Alignment {
    #[display("left")]
    Left,
    #[display("centered")]
    Centered,
    #[display("right")]
    Right,
}

/// Common size classes.
#[derive(Clone, Debug, Display, PartialEq, Eq)]
#[display("is-{}")]
pub enum Size {
    #[display("small")]
    Small,
    #[display("normal")]
    Normal,
    #[display("medium")]
    Medium,
    #[display("large")]
    Large,
}

impl IntoPropValue<Cow<'static, str>> for Size {
    fn into_prop_value(self) -> Cow<'static, str> {
        Cow::from(self.to_string())
    }
}
