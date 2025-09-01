mod components;
mod general;

use components::*;
use general::*;

use console_error_panic_hook::set_once as set_panic_hook;
use ybc::*;
use yew::prelude::*;

// template: https://jenil.github.io/bulmaswatch/darkly/

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Header />
            <Section>
                <Container classes="is-fluid">
                    <Columns>
                        <Column classes="is-2">
                            <SidebarMenu />
                        </Column>
                        <Column>
                            <TypographySection />
                            <BoxSection />
                            <ButtonSection />
                            <ContentSection />
                            <DeleteSection />
                            <FormSection />
                            <IconSection />
                            <ImagesSection />
                            <NotificationsSection />
                            <ProgressSection />
                            <TableSection />
                            <TagSection />
                            <BreadcrumbSection />
                            <HeroSection />
                            <CardSection />
                            <DropdownSection />
                            <MediaSection />
                            <MenuSection />
                            <MessageSection />
                            <ModalSection />
                            <NavbarSection />
                            <PaginationSection />
                            <PanelSection />
                            <TabsSection />
                        </Column>
                    </Columns>
                </Container>
            </Section>
        </>
    }
}

#[function_component(Header)]
fn header() -> Html {
    html! {
        <>
            <Navbar classes="is-primary" padded=true navbrand={html! {
                <NavbarItem tag={NavbarItemTag::A} href={"#".to_string()}>
                    {"Bulma"}
                </NavbarItem>
            }} />
            <Hero classes="is-primary" body={html!{
                <Container classes="has-text-centered">
                    <Title size={HeaderSize::Is1}>{"Bulma Yew"}</Title>
                    <Subtitle size={HeaderSize::Is2}>{"Subtitle"}</Subtitle>
                </Container>
            }} />
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
