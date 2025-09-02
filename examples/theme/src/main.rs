mod components;
mod elements;
mod forms;
mod sections;

use console_error_panic_hook::set_once as set_panic_hook;
use sections::TabId;
use strum::IntoEnumIterator;
use ybc::*;
use yew::prelude::*;

// template: https://jenil.github.io/bulmaswatch/darkly/

#[function_component(App)]
pub fn app() -> Html {
    // Active top-level tab index: 0 Elements, 1 Components, 2 Form, 3 Columns, 4 Grid, 5 Layout
    let active_tab = use_state(|| TabId::default());
    let on_tab_select = {
        let active_tab = active_tab.clone();
        Callback::from(move |idx: usize| active_tab.set(TabId::iter().nth(idx).unwrap_or_default()))
    };

    html! {
        <>
            <Header on_tab_select={on_tab_select} />
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
                    initial={Some(0)} onselect={props.on_tab_select.clone()} />
                })}
            />
        </>
    }
}

fn main() {
    set_panic_hook();
    yew::Renderer::<App>::new().render();
}
