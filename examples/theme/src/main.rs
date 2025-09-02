mod components;
mod general;

use components::{
    BreadcrumbSection, CardSection, DropdownSection, HeroSection, MediaSection, MenuSection, MessageSection, ModalSection, NavbarSection,
    PaginationSection, PanelSection, TabsSection,
};
use console_error_panic_hook::set_once as set_panic_hook;
use general::{
    BoxSection, ButtonSection, ContentSection, DeleteSection, FormSection, IconSection, ImagesSection, NotificationsSection, ProgressSection,
    TableSection, TagSection, TypographySection,
};
use strum::{Display, EnumIter, IntoEnumIterator};
use ybc::*;
use yew::prelude::*;

// template: https://jenil.github.io/bulmaswatch/darkly/

#[derive(Clone, Copy, Debug, Display, Hash, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
#[strum(serialize_all = "title_case")]
pub enum TabId {
    Elements,
    Components,
    Forms,
    Columns,
    Grid,
    Layout,
}

impl TabId {
    pub fn title(&self) -> String {
        self.to_string()
    }

    pub fn section_ids(&self) -> Vec<SectionId> {
        SectionId::iter().filter(|s| s.tab_id() == *self).collect()
    }

    pub fn menu_items(&self) -> Vec<MenuItem> {
        SectionId::iter()
            .filter(|s| s.tab_id() == *self)
            .map(|s| s.menu_item())
            .collect()
    }
}

#[derive(Clone, Copy, Debug, Display, Hash, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
#[strum(serialize_all = "title_case")]
pub enum SectionId {
    // Elements
    Typography,
    Box,
    Button,
    Content,
    Delete,
    Icon,
    Image,
    Notification,
    Progress,
    Table,
    Tag,

    // Components
    Breadcrumb,
    Card,
    Dropdown,
    Menu,
    Message,
    Modal,
    Navbar,
    Pagination,
    Panel,
    Tabs,

    // Form
    VerticalForm,
    HorizontalForm,

    // Layout
    Hero,
    MediaObject,
}

impl SectionId {
    pub fn tab_id(&self) -> TabId {
        match self {
            SectionId::Typography
            | SectionId::Box
            | SectionId::Button
            | SectionId::Content
            | SectionId::Delete
            | SectionId::Icon
            | SectionId::Image
            | SectionId::Notification
            | SectionId::Progress
            | SectionId::Table
            | SectionId::Tag => TabId::Elements,

            SectionId::Breadcrumb
            | SectionId::Card
            | SectionId::Dropdown
            | SectionId::Menu
            | SectionId::Message
            | SectionId::Modal
            | SectionId::Navbar
            | SectionId::Pagination
            | SectionId::Panel
            | SectionId::Tabs => TabId::Components,

            SectionId::VerticalForm | SectionId::HorizontalForm => TabId::Forms,

            SectionId::Hero | SectionId::MediaObject => TabId::Layout,
        }
    }

    pub fn id(&self) -> String {
        self.title().to_lowercase().replace(" ", "-")
    }

    pub fn link(&self) -> String {
        format!("#{}", self.id())
    }

    pub fn title(&self) -> String {
        self.to_string()
    }

    pub fn menu_item(&self) -> MenuItem {
        MenuItem {
            id: self.id().into(),
            label: self.title().into(),
            href: self.link().into(),
            children: None,
        }
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TabContentProps {
    pub tab: TabId,
    pub children: Children,
}

#[function_component(TabContent)]
pub fn content(props: &TabContentProps) -> Html {
    html! {
        <Columns>
            <Column classes="is-2">
                // <div style="position: sticky; top: 1rem;">
                    <ybc::Menu>
                        <ybc::MenuLabel text={props.tab.title()} />
                        <ybc::MenuList
                            items={Some(props.tab.menu_items())}
                            scroll_spy={Some(ybc::ScrollSpyConfig::default())}
                            click_behavior={ybc::ClickBehavior::Smooth}
                        />
                    </ybc::Menu>
                // </div>
            </Column>
            <Column>
                {props.children.clone()}
            </Column>
        </Columns>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    // Active top-level tab index: 0 Elements, 1 Components, 2 Form, 3 Columns, 4 Grid, 5 Layout
    let active_tab = use_state(|| 0usize);
    let on_tab_select = {
        let active_tab = active_tab.clone();
        Callback::from(move |idx: usize| active_tab.set(idx))
    };

    html! {
        <>
            <Header on_tab_select={on_tab_select} />
            <Section>
                <Container classes="is-fluid">
                    {
                        match *active_tab {
                            // Elements
                            0 => html!{
                                <TabContent tab={TabId::Elements}>
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
                                </TabContent>
                            },
                            // Components
                            1 => html! {
                                <TabContent tab={TabId::Components}>
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
                                </TabContent>
                            },
                            // Form
                            2 => html! {
                                <TabContent tab={TabId::Forms}>
                                    <FormSection />
                                </TabContent>
                            },
                            // Columns
                            3 => html! {
                                <TabContent tab={TabId::Columns}>
                                    <></>
                                </TabContent>
                            },
                            // Grid (no example components yet)
                            4 => html! {
                                <TabContent tab={TabId::Grid}>
                                    <></>
                                </TabContent>
                            },
                            // Layout
                            _ => html! {
                                <TabContent tab={TabId::Layout}>
                                    <HeroSection />
                                    <MediaSection />
                                </TabContent>
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

fn main() {
    set_panic_hook();
    yew::Renderer::<App>::new().render();
}
