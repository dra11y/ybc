use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ProgressProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The maximum amount of progress; the 100% value.
    #[prop_or_else(|| 1.0)]
    pub max: f32,
    /// The amount of progress which has been made. Omit for indeterminate.
    #[prop_or_default]
    pub value: Option<f32>,
}

/// A native HTML progress bar.
///
/// [https://bulma.io/documentation/elements/progress/](https://bulma.io/documentation/elements/progress/)
#[function_component(Progress)]
pub fn progress(props: &ProgressProps) -> Html {
    let class = classes!("progress", props.classes.clone());
    let max = props.max.to_string();
    let value = props.value.as_ref().map(|v| v.to_string());
    let label: Html = if !props.children.is_empty() {
        props.children.clone().into()
    } else if let Some(v) = props.value {
        html! {format!("{}%", v)}
    } else {
        html! {}
    };
    html! { <progress class={class} max={max} value={value}>{label}</progress> }
}
