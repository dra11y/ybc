use crate::{
    components::{
        BreadcrumbSection, CardSection, DropdownSection, HeroSection, MediaSection, MenuSection, MessageSection, ModalSection, NavbarSection,
        PaginationSection, PanelSection, TabsSection,
    },
    elements::{
        BoxSection, ButtonSection, ContentSection, DeleteSection, IconSection, ImagesSection, NotificationsSection, ProgressSection, TableSection,
        TagSection, TypographySection,
    },
    forms::FormSection,
};
use strum::{Display, EnumIter, IntoEnumIterator};
use ybc::*;
use yew::prelude::*;

#[derive(Default, Clone, Copy, Debug, Display, Hash, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
#[strum(serialize_all = "title_case")]
pub enum TabId {
    #[default]
    Elements,
    Components,
    Forms,
    Layout,
}

impl TabId {
    pub fn tabs() -> Vec<Tab> {
        Self::iter().map(|t| t.tab()).collect()
    }

    pub fn tab(&self) -> Tab {
        Tab {
            id: self.id().into(),
            label: self.title().into(),
            icon_class: None,
        }
    }

    /// Returns the TabId from the location hash.
    /// If not found, checks for a SectionId that matches, then returns
    /// that Section's TabId.
    pub fn from_id(id: Option<&str>) -> Option<Self> {
        id.and_then(|id| Self::iter().find(|t| t.id() == id))
            .or_else(|| Some(SectionId::from_id(id)?.tab_id()))
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

    pub fn section_ids(&self) -> Vec<SectionId> {
        SectionId::iter().filter(|s| s.tab_id() == *self).collect()
    }

    pub fn default_section_id(&self) -> Option<SectionId> {
        self.section_ids().first().cloned()
    }

    pub fn menu_items(&self) -> Vec<MenuItem> {
        SectionId::iter()
            .filter(|s| s.tab_id() == *self)
            .map(|s| s.menu_item())
            .collect()
    }

    pub fn tab_content(&self) -> Html {
        html! {
            <TabContent tab={*self}>{match self {
                TabId::Elements => html! {<>
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
                </>},
                TabId::Components => html! {<>
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
                </>},
                TabId::Forms => html! {<>
                    <FormSection />
                </>},
                TabId::Layout => html! {<>
                    <HeroSection />
                    <MediaSection />
                </>},
            }}</TabContent>
        }
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

    pub fn from_id(id: Option<&str>) -> Option<Self> {
        id.and_then(|id| Self::iter().find(|t| t.id() == id))
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
                <Menu sticky="top: 1rem">
                    <MenuLabel text={props.tab.title()} />
                    <MenuList
                        items={Some(props.tab.menu_items())}
                        scroll_spy={Some(ScrollSpyConfig { update_hash: false, ..ScrollSpyConfig::default() })}
                        click_behavior={ClickBehavior::Smooth}
                    />
                </Menu>
            </Column>
            <Column>
                {props.children.clone()}
            </Column>
        </Columns>
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SectionContentProps {
    pub section: SectionId,
    pub children: Children,
}

#[function_component(SectionContent)]
pub fn section_content(props: &SectionContentProps) -> Html {
    html! {
        <>
            <Section id={props.section.id()}>
                <Title tag="h1" size={HeaderSize::Is1}>{props.section.title()}</Title>
                <hr />
                {props.children.clone()}
            </Section>
        </>
    }
}
