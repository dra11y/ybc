use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BBoxProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

/// A white box to contain other elements.
///
/// [https://bulma.io/documentation/elements/box/](https://bulma.io/documentation/elements/box/)
#[function_component(BBox)]
pub fn r#box(props: &BBoxProps) -> Html {
    html! {
        <div class={classes!("box", props.classes.clone())}>
            {props.children.clone()}
        </div>
    }
}
