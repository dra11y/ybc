use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct MenuItem {
    /// Stable identifier for this item.
    pub id: AttrValue,
    /// Visible label for the link.
    pub label: AttrValue,
    /// Href for the anchor (e.g. "#typography").
    pub href: AttrValue,
    /// Optional nested child items (Bulma supports 2 levels, but this allows more).
    pub children: Option<Vec<MenuItem>>,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MenuProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

/// A simple menu, for any type of vertical navigation.
///
/// [https://bulma.io/documentation/components/menu/](https://bulma.io/documentation/components/menu/)
#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    let class = classes!("menu", props.classes.clone());
    html! {
        <aside class={class}>
            {props.children.clone()}
        </aside>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MenuListProps {
    /// The child `li` elements of this list.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// Optional: Have MenuList render and manage its own clickable items.
    /// When provided, `MenuList` will handle selection state internally and support nested items.
    #[prop_or_default]
    pub items: Option<Vec<MenuItem>>,
    /// Optional initial selected item id when using `items`. If None, nothing is selected.
    #[prop_or_default]
    pub initial: Option<AttrValue>,
    /// Optional controlled selection by id. When Some, overrides internal state.
    #[prop_or_default]
    pub selected: Option<AttrValue>,
    /// Optional selection callback invoked with the selected item id when using `items`.
    #[prop_or_default]
    pub onselect: Option<Callback<AttrValue>>,
}

/// A container for menu list `li` elements.
///
/// [https://bulma.io/documentation/components/menu/](https://bulma.io/documentation/components/menu/)
#[function_component(MenuList)]
pub fn menu_list(props: &MenuListProps) -> Html {
    let class = classes!("menu-list", props.classes.clone());

    // Internal selection state (Option), only used when items are provided.
    let internal_selected: UseStateHandle<Option<AttrValue>> = {
        let initial = props.initial.clone();
        use_state(move || initial)
    };
    let effective_selected: Option<AttrValue> = props.selected.as_ref().cloned().or_else(|| (*internal_selected).clone());

    fn render_items(
        items: &[MenuItem], effective_selected: &Option<AttrValue>, onselect: &Option<Callback<AttrValue>>,
        internal_selected: &UseStateHandle<Option<AttrValue>>,
    ) -> Html {
        html! {
            <>
                { for items.iter().map(|item| {
                    let is_active = effective_selected
                        .as_ref()
                        .map(|sel| sel == &item.id)
                        .unwrap_or(false);
                    let a_classes = classes!(is_active.then_some("is-active"));
                    let click_id = item.id.clone();
                    let onselect_cb = onselect.as_ref().cloned();
                    let internal_selected_for_click = internal_selected.clone();
                    let onclick = Callback::from(move |_| {
                        internal_selected_for_click.set(Some(click_id.clone()));
                        if let Some(cb) = onselect_cb.clone() { cb.emit(click_id.clone()); }
                    });
                    html! {
                        <li key={item.id.to_string()}>
                            <a class={a_classes} href={item.href.clone()} {onclick}>{ item.label.clone() }</a>
                            {
                                if let Some(children) = &item.children {
                                    html! { <ul>{ render_items(children.as_slice(), effective_selected, onselect, internal_selected) }</ul> }
                                } else { html!{} }
                            }
                        </li>
                    }
                }) }
            </>
        }
    }

    if let Some(items) = &props.items {
        html! {
            <ul class={class}>
                { render_items(items.as_slice(), &effective_selected, &props.onselect, &internal_selected) }
            </ul>
        }
    } else {
        html! {
            <ul class={class}>
                {props.children.clone()}
            </ul>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MenuLabelProps {
    #[prop_or_default]
    pub classes: Classes,
    /// The text of the label.
    #[prop_or_default]
    pub text: String,
}

/// A label for a section of the menu.
///
/// [https://bulma.io/documentation/components/menu/](https://bulma.io/documentation/components/menu/)
#[function_component(MenuLabel)]
pub fn menu_label(props: &MenuLabelProps) -> Html {
    html! {
        <p class={classes!("menu-label", props.classes.clone())}>
            {props.text.clone()}
        </p>
    }
}
