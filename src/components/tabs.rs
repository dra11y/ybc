use yew::prelude::*;

use crate::{Alignment, Size};

#[derive(Clone, Debug, PartialEq)]
pub struct Tab {
    /// Stable identifier for the tab (used for comparisons/keys).
    pub id: AttrValue,
    /// Visible label.
    pub label: AttrValue,
    /// Optional icon class for an <i> element inside the tab label, e.g. "fas fa-image".
    pub icon_class: Option<AttrValue>,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TabsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The alignment of this component.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Add a more classic style with borders to this component.
    #[prop_or_default]
    pub boxed: bool,
    /// Add the "radio button" style to the elements of this component.
    #[prop_or_default]
    pub toggle: bool,
    /// Make the tab elements of this component rounded.
    #[prop_or_default]
    pub rounded: bool,
    /// Make this component fullwidth.
    #[prop_or_default]
    pub fullwidth: bool,
    /// Optional: Have Tabs render and manage its own clickable tabs.
    /// When provided, `Tabs` will handle selection state internally.
    #[prop_or_default]
    pub tabs: Option<Vec<Tab>>,
    /// Optional initial selected tab index when using `items`.
    #[prop_or_default]
    pub initial: Option<usize>,
    /// Optional selection callback invoked with the selected index when using `items`.
    #[prop_or_default]
    pub onselect: Option<Callback<usize>>,
}

/// Simple responsive horizontal navigation tabs, with different styles.
///
/// [https://bulma.io/documentation/components/tabs/](https://bulma.io/documentation/components/tabs/)
///
/// For integration with Yew Router, it is recommended that the `RouterButton` or `RouterAnchor`
/// components be used as the individual tab elements for this component.
#[function_component(Tabs)]
pub fn tabs(props: &TabsProps) -> Html {
    let class = classes!(
        "tabs",
        props.classes.clone(),
        props.alignment.as_ref().map(ToString::to_string),
        props.size.as_ref().map(ToString::to_string),
        props.boxed.then_some("is-boxed"),
        props.toggle.then_some("is-toggle"),
        props.rounded.then_some("is-rounded"),
        props.fullwidth.then_some("is-fullwidth"),
    );
    // Self-managed selection state (always created to satisfy hook rules)
    let items_len = props.tabs.as_ref().map(|v| v.len()).unwrap_or(0);
    let initial = props.initial.unwrap_or(0).min(items_len.saturating_sub(1));
    let selected = use_state(|| initial);

    if let Some(items) = &props.tabs {
        let onselect = props.onselect.clone();

        html! {
            <div {class}>
                <ul>
                    { for items.iter().enumerate().map(|(idx, item)| {
                        let is_active = *selected == idx;
                        let selected_setter = selected.clone();
                        let onselect = onselect.clone();
                        let onclick = Callback::from(move |_| {
                            selected_setter.set(idx);
                            if let Some(cb) = onselect.clone() { cb.emit(idx); }
                        });
                        html!{
                            <li class={classes!(is_active.then_some("is-active"))} key={item.id.to_string()}>
                                <a {onclick}>
                                    { if let Some(icon) = &item.icon_class { html!{<span class="icon is-small"><i class={icon.clone()} aria-hidden="true"></i></span>} } else { html!{} } }
                                    <span>{ item.label.clone() }</span>
                                </a>
                            </li>
                        }
                    }) }
                </ul>
            </div>
        }
    } else {
        // Backwards-compatible, uncontrolled variant
        html! {
            <div {class}>
                <ul>
                    {props.children.clone()}
                </ul>
            </div>
        }
    }
}
