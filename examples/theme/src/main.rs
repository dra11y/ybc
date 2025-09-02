mod components;
mod elements;
mod forms;
mod sections;

use std::rc::Rc;

use console_error_panic_hook::set_once as set_panic_hook;
use gloo_timers::callback::Timeout;
use sections::{SectionId, TabId};
use strum::IntoEnumIterator;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, console, window};
use ybc::*;
use yew::prelude::*;

// template: https://jenil.github.io/bulmaswatch/darkly/

pub fn location_hash() -> Option<String> {
    Some(window()?.location().hash().ok()?.trim_start_matches('#').to_string())
}

pub fn push_state(url: &str) -> Result<(), JsValue> {
    window()
        .ok_or(JsValue::NULL)?
        .history()?
        .push_state_with_url(&JsValue::NULL, "", Some(url))
}

pub fn get_element_by_id(id: &str) -> Option<Element> {
    window()?.document()?.get_element_by_id(id)
}

#[function_component(App)]
pub fn app() -> Html {
    // Active top-level tab index: 0 Elements, 1 Components, 2 Form, 3 Columns, 4 Grid, 5 Layout
    // Initialize from URL hash if it matches a tab or section; else default
    let tab_id = TabId::from_id(location_hash().as_deref()).unwrap_or_default();
    let active_tab = use_state(move || tab_id);
    let on_tab_select = {
        let active_tab = active_tab.clone();
        Callback::from(move |idx: usize| {
            let Some(tab) = TabId::iter().nth(idx) else {
                return;
            };
            // Update state
            console::log_2(&"TAB SELECTED".into(), &tab.id().into());
            active_tab.set(tab);
            _ = push_state(&tab.link());
        })
    };

    // Helper: scroll to current section hash when the element exists.
    // Schedule several attempts to handle DOM timing after tab switches.
    let scroll_to_hash: Rc<dyn Fn(u32)> = Rc::new(|retries: u32| {
        let Some(id) = location_hash() else {
            return;
        };
        if SectionId::from_id(Some(&id)).is_none() {
            return;
        }
        let id_now = id.clone();
        let try_once = move || {
            if let Some(el) = get_element_by_id(&id_now) {
                el.scroll_into_view();
            }
        };
        try_once();
        for i in 1..=retries {
            let idc = id.clone();
            Timeout::new(16 * i, move || {
                if let Some(el) = get_element_by_id(&idc) {
                    el.scroll_into_view();
                }
            })
            .forget();
        }
    });

    // On mount: set tab from hash if present
    {
        let active_tab = active_tab.clone();
        let scroll_to_hash = scroll_to_hash.clone();
        use_effect_with((), move |_| {
            if let Some(tab) = TabId::from_id(location_hash().as_deref()) {
                console::log_2(&"MOUNT set TAB".into(), &tab.id().into());
                if *active_tab != tab {
                    active_tab.set(tab);
                }
                // Schedule scroll after state change
                Timeout::new(0, move || (scroll_to_hash.as_ref())(10)).forget();
            }
            || {}
        });
    }

    // Listen to hashchange to switch tabs when navigating via URL/back/forward
    {
        let active_tab = active_tab.clone();
        let scroll_to_hash = scroll_to_hash.clone();
        use_effect_with((), move |_| {
            let cb = Closure::wrap(Box::new(move |_e: Event| {
                let hash = location_hash();
                let Some(tab) = TabId::from_id(hash.as_deref()) else {
                    console::log_2(&"hashchange NO MATCH".into(), &hash.into());
                    return;
                };

                console::log_3(&"hashchange TAB".into(), &tab.id().into(), &active_tab.id().into());
                if *active_tab != tab {
                    console::log_2(&"hashchange TAB".into(), &"CHANGED".into());
                    active_tab.set(tab);
                }
                // Scroll to the target on navigation
                (scroll_to_hash.as_ref())(10);
            }) as Box<dyn FnMut(_)>);

            if let Some(win) = window() {
                let _ = win.add_event_listener_with_callback("hashchange", cb.as_ref().unchecked_ref());
            }
            // cleanup
            move || {
                if let Some(win) = window() {
                    let _ = win.remove_event_listener_with_callback("hashchange", cb.as_ref().unchecked_ref());
                }
            }
        });
    }

    // // Also scroll on initial mount even if tab didn't change
    // {
    //     let scroll = scroll_to_hash.clone();
    //     use_effect_with((), move |_| {
    //         // Delay to ensure DOM is ready
    //         Timeout::new(0, move || (scroll.as_ref())(30)).forget();
    //         move || {}
    //     });
    // }

    // // After tab switches, scroll to the current section hash (if any)
    // {
    //     let scroll = scroll_to_hash.clone();
    //     let active = active_tab.clone();
    //     use_effect_with((*active).clone(), move |_| {
    //         // Schedule after render; allow retries for mount timing
    //         Timeout::new(0, move || (scroll.as_ref())(30)).forget();
    //         move || {}
    //     });
    // }

    html! {
        <>
            <Header on_tab_select={on_tab_select} active_idx={TabId::iter().position(|t| t == *active_tab).unwrap_or(0)} />
            <Section>
                <Container classes="is-fluid">{
                    active_tab.tab_content()
                }</Container>
            </Section>
        </>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct HeaderProps {
    pub on_tab_select: Callback<usize>,
    pub active_idx: usize,
}

#[function_component(Header)]
fn header(props: &HeaderProps) -> Html {
    html! {
        <>
            <Navbar classes="is-primary" padded=true navbrand={html! {
                <NavbarItem tag={NavbarItemTag::A} href={"#".to_string()}>
                    {"Bulma"}
                </NavbarItem>
            }} />
            <Hero classes="is-primary"
                body={html!{
                <Container classes="has-text-centered">
                    <Title size={HeaderSize::Is1}>{"Bulma Yew"}</Title>
                    <Subtitle size={HeaderSize::Is2}>{"Subtitle"}</Subtitle>
                </Container>
            }}
                foot={Some(html!{
                    <Tabs boxed={true} alignment={Some(Alignment::Centered)}
                    tabs={TabId::tabs()}
                    initial={Some(props.active_idx)}
                    onselect={props.on_tab_select.clone()}
                    key={props.active_idx} />
                })}
            />
        </>
    }
}

fn main() {
    set_panic_hook();
    yew::Renderer::<App>::new().render();
}
