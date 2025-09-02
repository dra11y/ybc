mod components;
mod general;

use components::*;
use general::*;

use console_error_panic_hook::set_once as set_panic_hook;
use gloo_timers::callback::Timeout;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{History, HtmlElement, window};
use ybc::*;
use yew::prelude::*;

// template: https://jenil.github.io/bulmaswatch/darkly/

#[function_component(App)]
pub fn app() -> Html {
    // Active top-level tab index: 0 Elements, 1 Components, 2 Form, 3 Columns, 4 Grid, 5 Layout
    let active_tab = use_state(|| 0usize);
    let on_tab_select = {
        let active_tab = active_tab.clone();
        Callback::from(move |idx: usize| active_tab.set(idx))
    };

    // Elements tab: selected sidebar item id (Option)
    let selected_id_elements = use_state(|| None::<AttrValue>);
    // Debounce observer updates while programmatic smooth scroll is running
    let scroll_lock = use_mut_ref(|| false);
    let scroll_lock_effect = scroll_lock.clone();
    // Timer handle to unlock scroll lock after smooth scroll duration
    let scroll_unlock_timer = use_mut_ref(|| None::<gloo_timers::callback::Timeout>);

    // Scroll spy: only when Elements tab is active
    {
        let selected_id_elements = selected_id_elements.clone();
        let active_tab = active_tab.clone();
        let scroll_lock = scroll_lock_effect.clone();
        use_effect_with(active_tab, move |active_tab| {
            let scroll_cb_holder: Rc<RefCell<Option<Closure<dyn FnMut(web_sys::Event)>>>> = Rc::new(RefCell::new(None));
            let throttle_timer: Rc<RefCell<Option<gloo_timers::callback::Timeout>>> = Rc::new(RefCell::new(None));

            if **active_tab == 0 {
                if let Some(win) = window() {
                    if let Some(doc) = win.document() {
                        let ids = [
                            "typography",
                            "box",
                            "button",
                            "content",
                            "delete",
                            "icon",
                            "images",
                            "notifications",
                            "progress",
                            "table",
                            "tag",
                        ];
                        let selected_id_setter = selected_id_elements.clone();
                        let history: Option<History> = win.history().ok();
                        let scroll_lock_for_effect = scroll_lock.clone();
                        let throttle_timer_c = throttle_timer.clone();
                        let cb: Closure<dyn FnMut(web_sys::Event)> = Closure::wrap(Box::new(move |_| {
                            if *scroll_lock_for_effect.borrow() {
                                return;
                            }
                            // throttle to ~60fps using a short timeout
                            if throttle_timer_c.borrow().is_some() {
                                return;
                            }
                            let doc = doc.clone();
                            let selected_id_setter = selected_id_setter.clone();
                            let history = history.clone();
                            let ids_vec = ids.map(|s| s.to_string()).to_vec();
                            let timer_holder = throttle_timer_c.clone();
                            let handle = gloo_timers::callback::Timeout::new(16, move || {
                                // Stable rule: pick last section whose offsetTop <= current scroll + header bias
                                let current = window().and_then(|w| w.scroll_y().ok()).unwrap_or(0.0);
                                let threshold = current + 80.0; // header bias
                                let mut candidate: Option<(String, f64)> = None;
                                for id in &ids_vec {
                                    if let Some(el) = doc.get_element_by_id(id) {
                                        if let Ok(hel) = el.dyn_into::<HtmlElement>() {
                                            let top = hel.offset_top() as f64;
                                            if top <= threshold {
                                                match candidate {
                                                    Some((_, best_top)) if best_top >= top => {}
                                                    _ => {
                                                        candidate = Some((id.clone(), top));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                let chosen_id = candidate.map(|(id, _)| id).or_else(|| ids_vec.first().cloned());
                                if let Some(id_str) = chosen_id {
                                    let already = selected_id_setter
                                        .as_ref()
                                        .as_ref()
                                        .map(|v| v.as_str().to_string())
                                        .unwrap_or_default();
                                    if already != id_str {
                                        selected_id_setter.set(Some(id_str.clone().into()));
                                        if let Some(h) = &history {
                                            let _ = h.replace_state_with_url(&JsValue::NULL, "", Some(&format!("#{id_str}")));
                                        }
                                    }
                                }
                                // clear throttle
                                let _ = timer_holder.borrow_mut().take();
                            });
                            throttle_timer_c.borrow_mut().replace(handle);
                        }));
                        win.add_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref())
                            .ok();
                        scroll_cb_holder.borrow_mut().replace(cb);
                    }
                }
            }

            let holder = scroll_cb_holder.clone();
            let throttle = throttle_timer.clone();
            move || {
                if let Some(cb) = holder.borrow_mut().take() {
                    if let Some(win) = window() {
                        let _ = win.remove_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref());
                    }
                }
                if let Some(t) = throttle.borrow_mut().take() {
                    t.cancel();
                }
            }
        });
    }
    html! {
        <>
            <Header on_tab_select={on_tab_select} />
            <Section>
                <Container classes="is-fluid">
                    {
                        match *active_tab {
                            // Elements
                            0 => {
                                // Elements tab: list of anchors and ids
                                let items = vec![
                                    MenuItem { id: "typography".into(), label: "Typography".into(), href: "#typography".into(), children: None },
                                    MenuItem { id: "box".into(), label: "Box".into(), href: "#box".into(), children: None },
                                    MenuItem { id: "button".into(), label: "Button".into(), href: "#button".into(), children: None },
                                    MenuItem { id: "content".into(), label: "Content".into(), href: "#content".into(), children: None },
                                    MenuItem { id: "delete".into(), label: "Delete".into(), href: "#delete".into(), children: None },
                                    MenuItem { id: "icon".into(), label: "Icon".into(), href: "#icon".into(), children: None },
                                    MenuItem { id: "images".into(), label: "Image".into(), href: "#images".into(), children: None },
                                    MenuItem { id: "notifications".into(), label: "Notification".into(), href: "#notifications".into(), children: None },
                                    MenuItem { id: "progress".into(), label: "Progress".into(), href: "#progress".into(), children: None },
                                    MenuItem { id: "table".into(), label: "Table".into(), href: "#table".into(), children: None },
                                    MenuItem { id: "tag".into(), label: "Tag".into(), href: "#tag".into(), children: None },
                                ];

                                html!{
                                    <Columns>
                                        <Column classes="is-2">
                                            <div style="position: sticky; top: 1rem;">
                                                <ybc::Menu>
                                                    <ybc::MenuLabel text="Elements" />
                            <ybc::MenuList
                                                        items={Some(items)}
                                                        selected={(*selected_id_elements).clone()}
                                                        onselect={{
                                let scroll_lock = scroll_lock.clone();
                                                            let scroll_unlock_timer = scroll_unlock_timer.clone();
                                                            Callback::from(move |_| {
                                                                // Lock observer updates for duration of CSS smooth scroll (~400ms default)
                                                                *scroll_lock.borrow_mut() = true;
                                                                // Clear any previous timer and start a new one
                                                                if let Some(t) = scroll_unlock_timer.borrow_mut().take() {
                                                                    t.cancel();
                                                                }
                                                                let lock_ref = scroll_lock.clone();
                                                                let handle = Timeout::new(450, move || {
                                                                    *lock_ref.borrow_mut() = false;
                                                                });
                                                                scroll_unlock_timer.borrow_mut().replace(handle);
                                                            })
                                                        }}
                                                    />
                                                </ybc::Menu>
                                            </div>
                                        </Column>
                                        <Column>
                                            <TypographySection />
                                            <BoxSection />
                                            <ButtonSection />
                                            <ContentSection />
                                            <DeleteSection />
                                            <IconSection />
                                            <ImagesSection />
                                            <NotificationsSection />
                                            <ProgressSection />
                                            <TableSection />
                                            <TagSection />
                                        </Column>
                                    </Columns>
                                }
                            },
                            // Components (using library scroll-spy feature)
                            1 => {
                                let components_items = vec![
                                    MenuItem { id: "breadcrumb".into(), label: "Breadcrumb".into(), href: "#breadcrumb".into(), children: None },
                                    MenuItem { id: "card".into(), label: "Card".into(), href: "#card".into(), children: None },
                                    MenuItem { id: "dropdown".into(), label: "Dropdown".into(), href: "#dropdown".into(), children: None },
                                    MenuItem { id: "menu".into(), label: "Menu".into(), href: "#menu".into(), children: None },
                                    MenuItem { id: "message".into(), label: "Message".into(), href: "#message".into(), children: None },
                                    MenuItem { id: "modal".into(), label: "Modal".into(), href: "#modal".into(), children: None },
                                    MenuItem { id: "navbar".into(), label: "Navbar".into(), href: "#navbar".into(), children: None },
                                    MenuItem { id: "pagination".into(), label: "Pagination".into(), href: "#pagination".into(), children: None },
                                    MenuItem { id: "panel".into(), label: "Panel".into(), href: "#panel".into(), children: None },
                                    MenuItem { id: "tabs".into(), label: "Tabs".into(), href: "#tabs".into(), children: None },
                                ];

                                html!{
                                    <Columns>
                                        <Column classes="is-2">
                                            <div style="position: sticky; top: 1rem;">
                                                <ybc::Menu>
                                                    <ybc::MenuLabel text="Components" />
                                                    <ybc::MenuList
                                                        items={Some(components_items)}
                                                        scroll_spy={Some(ybc::ScrollSpyConfig::default())}
                                                        click_behavior={ybc::ClickBehavior::Smooth}
                                                    />
                                                </ybc::Menu>
                                            </div>
                                        </Column>
                                        <Column>
                                            <BreadcrumbSection />
                                            <CardSection />
                                            <DropdownSection />
                                            <MenuSection />
                                            <MessageSection />
                                            <ModalSection />
                                            <NavbarSection />
                                            <PaginationSection />
                                            <PanelSection />
                                            <TabsSection />
                                        </Column>
                                    </Columns>
                                }
                            },
                            // Form
                            2 => html!{
                                <Columns>
                                    <Column classes="is-2">
                                        <div style="position: sticky; top: 1rem;">
                                            <ybc::Menu>
                                                <ybc::MenuLabel text="Form" />
                                                <ybc::MenuList>
                                                    <li><a href="#form">{"General"}</a></li>
                                                </ybc::MenuList>
                                            </ybc::Menu>
                                        </div>
                                    </Column>
                                    <Column>
                                        <FormSection />
                                    </Column>
                                </Columns>
                            },
                            // Columns (no example components yet)
                            3 => html!{
                                <Columns>
                                    <Column classes="is-2">
                                        <div style="position: sticky; top: 1rem;">
                                            <ybc::Menu>
                                                <ybc::MenuLabel text="Columns" />
                                            </ybc::Menu>
                                        </div>
                                    </Column>
                                    <Column>
                                        // Intentionally left blank: no columns examples in this theme yet
                                    </Column>
                                </Columns>
                            },
                            // Grid (no example components yet)
                            4 => html!{
                                <Columns>
                                    <Column classes="is-2">
                                        <div style="position: sticky; top: 1rem;">
                                            <ybc::Menu>
                                                <ybc::MenuLabel text="Grid" />
                                            </ybc::Menu>
                                        </div>
                                    </Column>
                                    <Column>
                                        // Intentionally left blank: no grid examples in this theme yet
                                    </Column>
                                </Columns>
                            },
                            // Layout
                            _ => html!{
                                <Columns>
                                    <Column classes="is-2">
                                        <div style="position: sticky; top: 1rem;">
                                            <ybc::Menu>
                                                <ybc::MenuLabel text="Layout" />
                                                <ybc::MenuList>
                                                    <li><a href="#hero">{"Hero"}</a></li>
                                                    <li><a href="#media">{"Media Object"}</a></li>
                                                </ybc::MenuList>
                                            </ybc::Menu>
                                        </div>
                                    </Column>
                                    <Column>
                                        <HeroSection />
                                        <MediaSection />
                                    </Column>
                                </Columns>
                            },
                        }
                    }
                </Container>
            </Section>
        </>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct HeaderProps {
    pub on_tab_select: Callback<usize>,
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
                    <Tabs boxed={true} alignment={Some(Alignment::Centered)} tabs={vec![
                        Tab {
                            id: "elements".into(),
                            label: "Elements".into(),
                            icon_class: None,
                        },
                        Tab {
                            id: "components".into(),
                            label: "Components".into(),
                            icon_class: None,
                        },
                        Tab {
                            id: "form".into(),
                            label: "Form".into(),
                            icon_class: None,
                        },
                        Tab {
                            id: "columns".into(),
                            label: "Columns".into(),
                            icon_class: None,
                        },
                        Tab {
                            id: "grid".into(),
                            label: "Grid".into(),
                            icon_class: None,
                        },
                        Tab {
                            id: "layout".into(),
                            label: "Layout".into(),
                            icon_class: None,
                        },
                    ]} initial={Some(0)} onselect={props.on_tab_select.clone()} />
                })}
            />
        </>
    }
}

#[function_component(SidebarMenu)]
fn sidebar_menu() -> Html {
    html! {
        <ybc::Menu>
            <ybc::MenuLabel text="General" />
            <ybc::MenuList>
                <li><a href="#typography">{"Typography"}</a></li>
                <li><a href="#box">{"Box"}</a></li>
                <li><a href="#button">{"Button"}</a></li>
                <li><a href="#content">{"Content"}</a></li>
                <li><a href="#delete">{"Delete"}</a></li>
                <li><a href="#form">{"Form"}</a></li>
                <li><a href="#icon">{"Icons"}</a></li>
                <li><a href="#images">{"Images"}</a></li>
                <li><a href="#notifications">{"Notifications"}</a></li>
                <li><a href="#progress">{"Progress"}</a></li>
                <li><a href="#table">{"Table"}</a></li>
                <li><a href="#tag">{"Tag"}</a></li>
            </ybc::MenuList>
            <ybc::MenuLabel text="Components" />
            <ybc::MenuList>
                <li><a href="#breadcrumb">{"Breadcrumb"}</a></li>
                <li><a href="#hero">{"Hero"}</a></li>
                <li><a href="#card">{"Card"}</a></li>
                <li><a href="#dropdown">{"Dropdown"}</a></li>
                <li><a href="#level">{"Level"}</a></li>
                <li><a href="#media">{"Media"}</a></li>
                <li><a href="#menu">{"Menu"}</a></li>
                <li><a href="#message">{"Message"}</a></li>
                <li><a href="#modal">{"Modal"}</a></li>
                <li><a href="#navbar">{"Navbar"}</a></li>
                <li><a href="#pagination">{"Pagination"}</a></li>
                <li><a href="#panel">{"Panel"}</a></li>
                <li><a href="#tabs">{"Tabs"}</a></li>
                <li><a href="#footer">{"Footer"}</a></li>
            </ybc::MenuList>
        </ybc::Menu>
    }
}

fn main() {
    set_panic_hook();
    yew::Renderer::<App>::new().render();
}
