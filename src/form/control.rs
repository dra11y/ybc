use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ControlProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
    /// A modifier to have the controlled element fill up the remaining space.
    #[prop_or_default]
    pub expanded: bool,
    /// Display an icon on the left of the control; adds `has-icons-left` to the control.
    #[prop_or_default]
    pub icons_left: bool,
    /// Display an icon on the right of the control; adds `has-icons-right` to the control.
    #[prop_or_default]
    pub icons_right: bool,
}

/// A container with which you can wrap the form controls.
///
/// [https://bulma.io/documentation/form/general/](https://bulma.io/documentation/form/general/)
#[function_component(Control)]
pub fn control(props: &ControlProps) -> Html {
    let class = classes!(
        "control",
        props.classes.clone(),
        props.expanded.then_some("is-expanded"),
        props.icons_left.then_some("has-icons-left"),
        props.icons_right.then_some("has-icons-right"),
    );
    html! {
        <@{props.tag.clone()} {class}>
            {props.children.clone()}
        </@>
    }
}
