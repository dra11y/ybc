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
/// https://bulma.io/documentation/components/menu/
#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    let class = classes!("menu", props.classes.clone());
    html! {
        <aside class={class}>
            {props.children.clone()}
        </aside>
    }
}

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

    /// Optional click behavior for in-page navigation (feature-gated).
    /// When None (default), behavior is Jump (anchor default).
    #[cfg(feature = "scroll-spy")]
    #[prop_or_default]
    pub click_behavior: Option<ClickBehavior>,

    /// Optional scroll spy configuration (feature-gated). When provided and `items` is Some,
    /// MenuList will update selection based on scroll position.
    #[cfg(feature = "scroll-spy")]
    #[prop_or_default]
    pub scroll_spy: Option<ScrollSpyConfig>,
}

/// A container for menu list `li` elements.
///
/// https://bulma.io/documentation/components/menu/
#[function_component(MenuList)]
pub fn menu_list(props: &MenuListProps) -> Html {
    let class = classes!("menu-list", props.classes.clone());

    // Internal selection state (Option), only used when items are provided.
    let internal_selected: UseStateHandle<Option<AttrValue>> = {
        let initial = props.initial.clone();
        use_state(move || initial)
    };
    let effective_selected: Option<AttrValue> = props.selected.as_ref().cloned().or_else(|| (*internal_selected).clone());

    // Feature-gated scroll spy: listen to scroll and update selection (only when items are provided)
    let effective_onselect = {
        #[cfg(feature = "scroll-spy")]
        {
            use std::cell::RefCell;
            use std::rc::Rc;
            use wasm_bindgen::closure::Closure;
            use wasm_bindgen::{JsCast, JsValue};
            use web_sys::{History, window};

            // Programmatic scroll lock while navigating by click
            let scroll_lock = use_mut_ref(|| false);
            let scroll_lock_effect = scroll_lock.clone();
            // Track the last clicked target id so we can unlock when reached
            let last_clicked_id = use_mut_ref(|| None::<String>);
            // Early clone for closures below
            let last_clicked_id_clone_for_click = last_clicked_id.clone();

            // Derive ids and config snapshot for effect dependencies
            fn collect_ids(items: &[MenuItem], out: &mut Vec<String>) {
                for it in items {
                    out.push(it.id.to_string());
                    if let Some(children) = &it.children {
                        collect_ids(children, out);
                    }
                }
            }
            let mut ids_vec: Vec<String> = Vec::new();
            if let Some(items) = &props.items {
                collect_ids(items, &mut ids_vec);
            }
            let cfg_opt = props.scroll_spy.clone();
            let uncontrolled = props.selected.is_none();

            // Scroll spy
            // {
            let selected_id_elements = internal_selected.clone();
            let scroll_lock_clone = scroll_lock_effect.clone();
            use_effect_with(
                (ids_vec.clone(), cfg_opt.clone(), uncontrolled),
                move |(ids_vec, cfg_opt, uncontrolled)| {
                    let scroll_cb_holder: Rc<RefCell<Option<Closure<dyn FnMut(web_sys::Event)>>>> = Rc::new(RefCell::new(None));
                    let wheel_cb_holder: Rc<RefCell<Option<Closure<dyn FnMut(web_sys::Event)>>>> = Rc::new(RefCell::new(None));
                    let touch_cb_holder: Rc<RefCell<Option<Closure<dyn FnMut(web_sys::Event)>>>> = Rc::new(RefCell::new(None));
                    let key_cb_holder: Rc<RefCell<Option<Closure<dyn FnMut(web_sys::Event)>>>> = Rc::new(RefCell::new(None));

                    // Only run built-in scroll spy when:
                    // - there are ids to observe,
                    // - scroll spy is enabled via props,
                    // - and selection is uncontrolled (no external `selected`).
                    if !ids_vec.is_empty()
                        && let Some(cfg) = cfg_opt
                        && *uncontrolled
                        && let Some(win) = window()
                        && let Some(doc) = win.document()
                    {
                        // if !ids_vec.is_empty() && cfg_opt.is_some() && *uncontrolled {
                        //     if let Some(win) = window() {
                        //         if let Some(doc) = win.document() {
                        let selected_id_setter = selected_id_elements.clone();
                        let history: Option<History> = win.history().ok();
                        let scroll_lock_for_effect = scroll_lock_clone.clone();
                        let last_clicked_for_effect = last_clicked_id.clone();
                        let ids_vec_owned = ids_vec.clone();

                        // Shared updater used by both initial run and scroll listener
                        let do_update: Rc<dyn Fn()> = {
                            let doc = doc.clone();
                            let selected_id_setter = selected_id_setter.clone();
                            let history = history.clone();
                            let ids_vec_owned = ids_vec_owned.clone();
                            let cfg = cfg.clone();
                            let scroll_lock_for_effect = scroll_lock_for_effect.clone();
                            let last_clicked_for_effect = last_clicked_for_effect.clone();
                            Rc::new(move || {
                                let win = window();
                                let current = win.as_ref().and_then(|w| w.scroll_y().ok()).unwrap_or(0.0);
                                // Compute dynamic baseline (fixed/sticky at y=0)
                                let mut baseline = 0.0_f64;
                                if let Some(w) = win.clone() {
                                    if let Some(doc2) = w.document() {
                                        let inner_w = w.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1024.0);
                                        let list = doc2.elements_from_point((inner_w as f32) / 2.0, 0.0);
                                        let len = list.length();
                                        for i in 0..len {
                                            let js = list.get(i);
                                            if let Ok(elem) = js.dyn_into::<web_sys::Element>() {
                                                let rect = elem.get_bounding_client_rect();
                                                let mut include = false;
                                                if let Some(w) = window() {
                                                    if let Ok(style) = w.get_computed_style(&elem) {
                                                        if let Some(style) = style {
                                                            if let Ok(pos) = style.get_property_value("position") {
                                                                let p = pos.trim();
                                                                include = p == "fixed" || p == "sticky";
                                                            }
                                                        }
                                                    }
                                                }
                                                if include {
                                                    if rect.top() <= 0.0 && rect.bottom() > 0.0 {
                                                        baseline = baseline.max(rect.bottom());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                let threshold = current + baseline + (cfg.offset_px as f64);
                                // Choose last section whose absolute top <= threshold
                                let mut candidate: Option<(String, f64)> = None;
                                for id in &ids_vec_owned {
                                    if let Some(el) = doc.get_element_by_id(id) {
                                        let rect = el.get_bounding_client_rect();
                                        let top = rect.top() + current;
                                        if top <= threshold {
                                            match candidate {
                                                Some((_, best_top)) if best_top >= top => {}
                                                _ => candidate = Some((id.clone(), top)),
                                            }
                                        }
                                    }
                                }
                                let chosen_id = candidate.map(|(id, _)| id).or_else(|| ids_vec_owned.first().cloned());
                                if let Some(id_str) = chosen_id {
                                    if *scroll_lock_for_effect.borrow() {
                                        if let Some(clicked) = last_clicked_for_effect.borrow().clone() {
                                            if clicked == id_str {
                                                // Arrived at target: unlock and clear
                                                *scroll_lock_for_effect.borrow_mut() = false;
                                                last_clicked_for_effect.borrow_mut().take();
                                            } else {
                                                // Still scrolling to target: do not update selection yet
                                                return;
                                            }
                                        } else {
                                            // Defensive: if locked but no target, unlock
                                            *scroll_lock_for_effect.borrow_mut() = false;
                                        }
                                    }
                                    let already = selected_id_setter
                                        .as_ref()
                                        .as_ref()
                                        .map(|v| v.as_str().to_string())
                                        .unwrap_or_default();
                                    if already != id_str {
                                        selected_id_setter.set(Some(id_str.clone().into()));
                                        if cfg.update_hash {
                                            if let Some(h) = &history {
                                                let _ = h.replace_state_with_url(&JsValue::NULL, "", Some(&format!("#{id_str}")));
                                            }
                                        }
                                    }
                                }
                            })
                        };

                        // Scroll listener: update in real time using the shared updater
                        let do_update_cb = do_update.clone();
                        let cb: Closure<dyn FnMut(web_sys::Event)> = Closure::wrap(Box::new(move |_| {
                            (do_update_cb.as_ref())();
                        }));
                        win.add_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref())
                            .ok();
                        scroll_cb_holder.borrow_mut().replace(cb);

                        // Initial selection based on current viewport
                        (do_update.as_ref())();

                        // User input listeners: single unlock action reused for wheel/touch/key
                        let unlock_action: Rc<dyn Fn()> = {
                            let scroll_lock = scroll_lock_clone.clone();
                            let last_clicked = last_clicked_id.clone();
                            Rc::new(move || {
                                if *scroll_lock.borrow() {
                                    *scroll_lock.borrow_mut() = false;
                                    last_clicked.borrow_mut().take();
                                }
                            })
                        };
                        // Attach all input listeners via one compact loop
                        let input_listeners: Vec<(&'static str, Rc<RefCell<Option<Closure<dyn FnMut(web_sys::Event)>>>>)> = vec![
                            ("wheel", wheel_cb_holder.clone()),
                            ("touchmove", touch_cb_holder.clone()),
                            ("keydown", key_cb_holder.clone()),
                        ];
                        for (ev, holder) in &input_listeners {
                            let action = unlock_action.clone();
                            let closure = Closure::wrap(Box::new(move |_e: web_sys::Event| {
                                (action.as_ref())();
                            }) as Box<dyn FnMut(_)>);
                            let _ = win.add_event_listener_with_callback(ev, closure.as_ref().unchecked_ref());
                            holder.borrow_mut().replace(closure);
                        }

                        // (Initial selection handled via do_update above)
                        //     }
                        // }
                    }

                    let holder = scroll_cb_holder.clone();
                    let input_listeners: Vec<(&'static str, Rc<RefCell<Option<Closure<dyn FnMut(web_sys::Event)>>>>)> = vec![
                        ("wheel", wheel_cb_holder.clone()),
                        ("touchmove", touch_cb_holder.clone()),
                        ("keydown", key_cb_holder.clone()),
                    ];
                    move || {
                        if let Some(cb) = holder.borrow_mut().take() {
                            if let Some(win) = window() {
                                let _ = win.remove_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref());
                            }
                        }
                        for (ev, h) in input_listeners {
                            if let Some(cb) = h.borrow_mut().take() {
                                if let Some(win) = window() {
                                    let _ = win.remove_event_listener_with_callback(ev, cb.as_ref().unchecked_ref());
                                }
                            }
                        }
                    }
                },
            );
            // }

            // Create click behavior callback with scroll lock
            let scroll_lock_for_click = scroll_lock.clone();
            let last_clicked_for_click = last_clicked_id_clone_for_click.clone();
            let original_onselect = props.onselect.clone();
            Some(Callback::from(move |id: AttrValue| {
                // Always call original onselect first
                if let Some(cb) = &original_onselect {
                    cb.emit(id.clone());
                }

                // Lock observer updates; unlock when target reached or on user input
                *scroll_lock_for_click.borrow_mut() = true;
                last_clicked_for_click.borrow_mut().replace(id.to_string());
            }))
        }
        #[cfg(not(feature = "scroll-spy"))]
        {
            props.onselect.clone()
        }
    };

    #[allow(unused_variables)]
    fn render_items(
        items: &[MenuItem], effective_selected: &Option<AttrValue>, onselect: &Option<Callback<AttrValue>>,
        internal_selected: &UseStateHandle<Option<AttrValue>>, #[cfg(feature = "scroll-spy")] click_behavior: &Option<ClickBehavior>,
        #[cfg(feature = "scroll-spy")] spy_enabled: bool,
    ) -> Html {
        let nodes: Vec<Html> = items
            .iter()
            .map(|item| {
                let is_active = effective_selected.as_ref().map(|sel| sel == &item.id).unwrap_or(false);
                let a_classes = classes!(is_active.then_some("is-active"));
                let click_id = item.id.clone();
                let onselect_cb = onselect.as_ref().cloned();
                let internal_selected_for_click = internal_selected.clone();
                #[cfg(feature = "scroll-spy")]
                let behavior_for_onclick = click_behavior.clone();
                #[cfg(not(feature = "scroll-spy"))]
                let spy_enabled = false;
                let onclick = Callback::from(move |_e: web_sys::MouseEvent| {
                    // Immediately highlight clicked item; rAF-based lock prevents spy overrides until scroll settles
                    internal_selected_for_click.set(Some(click_id.clone()));
                    if let Some(cb) = onselect_cb.clone() {
                        cb.emit(click_id.clone());
                    }
                    #[cfg(feature = "scroll-spy")]
                    if let Some(b) = behavior_for_onclick.clone() {
                        match b {
                            ClickBehavior::Jump => { /* default anchor behavior */ }
                            ClickBehavior::Smooth => { /* rely on CSS scroll-behavior if set */ }
                            ClickBehavior::Timed { .. } => { /* treat like Smooth for now */ }
                        }
                    }
                });
                #[cfg(feature = "scroll-spy")]
                let behavior_for_children = click_behavior.clone();
                html! {
                    <li key={item.id.to_string()}>
                        <a class={a_classes} href={item.href.clone()} {onclick}>{ item.label.clone() }</a>
                        {
                            if let Some(children) = &item.children {
                                html! { <ul>{ render_items(
                                    children.as_slice(),
                                    effective_selected, onselect, internal_selected,
                                    #[cfg(feature = "scroll-spy")] &behavior_for_children,
                                    #[cfg(feature = "scroll-spy")] spy_enabled,
                                ) }</ul> }
                            } else { html!{} }
                        }
                    </li>
                }
            })
            .collect();
        html! { <>{ nodes }</> }
    }

    if let Some(items) = &props.items {
        html! {
            <ul class={class}>
                { render_items(
                    items.as_slice(),
                    &effective_selected,
                    &effective_onselect,
                    &internal_selected,
                    #[cfg(feature = "scroll-spy")] &props.click_behavior,
                    #[cfg(feature = "scroll-spy")] props.scroll_spy.is_some(),
                ) }
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

// Feature-gated configuration type for scroll-spy
#[cfg(feature = "scroll-spy")]
#[derive(Clone, Debug, PartialEq)]
pub struct ScrollSpyConfig {
    pub offset_px: i32,    // header bias
    pub throttle_ms: u32,  // scroll handler throttle interval
    pub update_hash: bool, // replaceState hash updates
}

#[cfg(feature = "scroll-spy")]
impl Default for ScrollSpyConfig {
    fn default() -> Self {
        Self { offset_px: 0, throttle_ms: 16, update_hash: true }
    }
}

#[cfg(feature = "scroll-spy")]
#[derive(Clone, Debug, PartialEq)]
pub enum ClickBehavior {
    Jump,
    Smooth,
    Timed { ms: u32 },
}

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
/// https://bulma.io/documentation/components/menu/
#[function_component(MenuLabel)]
pub fn menu_label(props: &MenuLabelProps) -> Html {
    html! {
        <p class={classes!("menu-label", props.classes.clone())}>
            {props.text.clone()}
        </p>
    }
}
