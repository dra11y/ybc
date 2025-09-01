use console_error_panic_hook::set_once as set_panic_hook;
use ybc::{
    Box as YBox, Button, Buttons, Column, Columns, Container, Content, HeaderSize, Hero, Icon, Image, ImageSize, Level, LevelItem, LevelLeft, Media,
    MediaContent, MediaLeft, Navbar, NavbarItem, NavbarItemTag, Section, Subtitle, Title,
};
use yew::prelude::*;

#[function_component(Header)]
fn header() -> Html {
    // Navbar (primary) with brand "Bulma"
    let navbrand = html! {
        <>
            <NavbarItem tag={NavbarItemTag::A} href={"#".to_string()}>
                {"Bulma"}
            </NavbarItem>
        </>
    };

    html! {
        <>
            <Navbar classes="is-primary" padded=true navbrand={navbrand} />
            <Hero classes="is-primary" body={html!{
                <Container classes="has-text-centered">
                    <Title size={HeaderSize::Is1}>{"Bulma Yew"}</Title>
                    <Subtitle size={HeaderSize::Is3}>{"Subtitle"}</Subtitle>
                </Container>
            }} />
        </>
    }
}

#[function_component(SidebarMenu)]
fn sidebar_menu() -> Html {
    // Note: Pending confirmation whether raw li/a are acceptable inside MenuList; using placeholders
    // for now.
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

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Header />
            <Section>
                <Container>
                    <Columns>
                        <Column classes="is-2">
                            <SidebarMenu />
                        </Column>
                        <Column>
                            <TypographySection />
                            <BoxSection />
                            <ButtonSection />
                        </Column>
                    </Columns>
                </Container>
            </Section>
        </>
    }
}

#[function_component(TypographySection)]
fn typography_section() -> Html {
    html! {
        <>
            <div id="typography"></div>
            <Section>
                <Title tag="h1" size={HeaderSize::Is1}>{"Typography"}</Title>
                <hr />
                <Columns>
                    <Column>
                        <Title tag="h1" size={HeaderSize::Is1}>{"Title 1"}</Title>
                        <Title tag="h2" size={HeaderSize::Is2}>{"Title 2"}</Title>
                        <Title tag="h3" size={HeaderSize::Is3}>{"Title 3"}</Title>
                        <Title tag="h4" size={HeaderSize::Is4}>{"Title 4"}</Title>
                        <Title tag="h5" size={HeaderSize::Is5}>{"Title 5"}</Title>
                        <Title tag="h6" size={HeaderSize::Is6}>{"Title 6"}</Title>
                    </Column>
                    <Column>
                        <Subtitle tag="h1" size={HeaderSize::Is1}>{"Subtitle 1"}</Subtitle>
                        <Subtitle tag="h2" size={HeaderSize::Is2}>{"Subtitle 2"}</Subtitle>
                        <Subtitle tag="h3" size={HeaderSize::Is3}>{"Subtitle 3"}</Subtitle>
                        <Subtitle tag="h4" size={HeaderSize::Is4}>{"Subtitle 4"}</Subtitle>
                        <Subtitle tag="h5" size={HeaderSize::Is5}>{"Subtitle 5"}</Subtitle>
                        <Subtitle tag="h6" size={HeaderSize::Is6}>{"Subtitle 6"}</Subtitle>
                    </Column>
                    <Column>
                        <Title tag="h1">{"Title"}</Title>
                        <Subtitle tag="h2">{"Subtitle"}</Subtitle>
                        <Title tag="p" size={HeaderSize::Is1}>{"Title 1"}</Title>
                        <Subtitle tag="p" size={HeaderSize::Is3}>{"Subtitle 3"}</Subtitle>
                        <Title tag="p" size={HeaderSize::Is2}>{"Title 2"}</Title>
                        <Subtitle tag="p" size={HeaderSize::Is4}>{"Subtitle 4"}</Subtitle>
                        <Title tag="p" size={HeaderSize::Is3}>{"Title 3"}</Title>
                        <Subtitle tag="p" size={HeaderSize::Is5}>{"Subtitle 5"}</Subtitle>
                    </Column>
                </Columns>
            </Section>
        </>
    }
}

#[function_component(BoxSection)]
fn box_section() -> Html {
    html! {
        <>
            <div id="box"></div>
            <Section>
                <Title tag="h1" size={HeaderSize::Is1}>{"Box"}</Title>
                <hr />
                <YBox>
                    <Media>
                        <MediaLeft>
                            <Image size={Some(ImageSize::Is64x64)}>
                                <img alt="Image" src="https://s3.amazonaws.com/uifaces/faces/twitter/zeldman/128.jpg" />
                            </Image>
                        </MediaLeft>
                        <MediaContent>
                            <Content>
                                <p>
                                    <strong>{"John Smith"}</strong>
                                    {" "}
                                    <small>{"@johnsmith"}</small>
                                    {" "}
                                    <small>{"31m"}</small>
                                    <br />
                                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean efficitur sit amet massa fringilla egestas. Nullam condimentum luctus turpis."}
                                </p>
                            </Content>
                            <Level>
                                <LevelLeft>
                                    <LevelItem>
                                        <a class="level-item">
                                            <Icon size={Some(ybc::Size::Small)}>
                                                <i class="fa fa-reply"></i>
                                            </Icon>
                                        </a>
                                    </LevelItem>
                                    <LevelItem>
                                        <a class="level-item">
                                            <Icon size={Some(ybc::Size::Small)}>
                                                <i class="fa fa-retweet"></i>
                                            </Icon>
                                        </a>
                                    </LevelItem>
                                    <LevelItem>
                                        <a class="level-item">
                                            <Icon size={Some(ybc::Size::Small)}>
                                                <i class="fa fa-heart"></i>
                                            </Icon>
                                        </a>
                                    </LevelItem>
                                </LevelLeft>
                            </Level>
                        </MediaContent>
                    </Media>
                </YBox>
            </Section>
        </>
    }
}

#[function_component(ButtonSection)]
fn button_section() -> Html {
    html! {
        <>
            <div id="button"></div>
            <Section>
                <Title tag="h1" size={HeaderSize::Is1}>{"Button"}</Title>
                <hr />
                <Columns>
                    <Column>
                        <Buttons>
                            <Button>{"Button"}</Button>
                            <Button classes="is-white">{"White"}</Button>
                            <Button classes="is-light">{"Light"}</Button>
                            <Button classes="is-dark">{"Dark"}</Button>
                            <Button classes="is-black">{"Black"}</Button>
                            <Button classes="is-link">{"Link"}</Button>
                            <Button classes="is-text">{"Link"}</Button>
                        </Buttons>
                        <Buttons>
                            <Button classes="is-primary">{"Primary"}</Button>
                            <Button classes="is-info">{"Info"}</Button>
                            <Button classes="is-success">{"Success"}</Button>
                            <Button classes="is-warning">{"Warning"}</Button>
                            <Button classes="is-danger">{"Danger"}</Button>
                        </Buttons>
                        <Buttons>
                            <Button classes="is-focused">{"Focus"}</Button>
                            <Button classes="is-primary is-focused">{"Focus"}</Button>
                            <Button classes="is-info is-focused">{"Focus"}</Button>
                            <Button classes="is-success is-focused">{"Focus"}</Button>
                            <Button classes="is-warning is-focused">{"Focus"}</Button>
                            <Button classes="is-danger is-focused">{"Focus"}</Button>
                        </Buttons>
                        <Buttons>
                            <Button classes="is-hovered">{"Hover"}</Button>
                            <Button classes="is-primary is-hovered">{"Hover"}</Button>
                            <Button classes="is-info is-hovered">{"Hover"}</Button>
                            <Button classes="is-success is-hovered">{"Hover"}</Button>
                            <Button classes="is-warning is-hovered">{"Hover"}</Button>
                            <Button classes="is-danger is-hovered">{"Hover"}</Button>
                        </Buttons>
                        <Buttons>
                            <Button classes="is-active">{"Active"}</Button>
                            <Button classes="is-primary is-active">{"Active"}</Button>
                            <Button classes="is-info is-active">{"Active"}</Button>
                            <Button classes="is-success is-active">{"Active"}</Button>
                            <Button classes="is-warning is-active">{"Active"}</Button>
                            <Button classes="is-danger is-active">{"Active"}</Button>
                        </Buttons>
                        <Buttons>
                            <Button classes="" loading={true}>{"Loading"}</Button>
                            <Button classes="is-primary" loading={true}>{"Loading"}</Button>
                            <Button classes="is-info" loading={true}>{"Loading"}</Button>
                            <Button classes="is-success" loading={true}>{"Loading"}</Button>
                            <Button classes="is-warning" loading={true}>{"Loading"}</Button>
                            <Button classes="is-danger" loading={true}>{"Loading"}</Button>
                        </Buttons>
                        <Buttons>
                            <Button>
                                <Icon size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-bold"></i>
                                </Icon>
                            </Button>
                            <Button>
                                <Icon size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-italic"></i>
                                </Icon>
                            </Button>
                            <Button>
                                <Icon size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-underline"></i>
                                </Icon>
                            </Button>
                            <Button>
                                <Icon>
                                    <i class="fab fa-github"></i>
                                </Icon>
                                <span>{"GitHub"}</span>
                            </Button>
                            <Button classes="is-primary">
                                <Icon>
                                    <i class="fab fa-twitter"></i>
                                </Icon>
                                <span>{"Twitter"}</span>
                            </Button>
                            <Button classes="is-success">
                                <Icon size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                                <span>{"Save"}</span>
                            </Button>
                            <Button classes="is-danger is-outlined">
                                <span>{"Delete"}</span>
                                <Icon size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-times"></i>
                                </Icon>
                            </Button>
                        </Buttons>
                        <ybc::Field addons=true>
                            <ybc::Control>
                                <Button>
                                    <Icon size={Some(ybc::Size::Small)}>
                                        <i class="fa fa-bold"></i>
                                    </Icon>
                                    <span>{"Bold"}</span>
                                </Button>
                            </ybc::Control>
                            <ybc::Control>
                                <Button>
                                    <Icon size={Some(ybc::Size::Small)}>
                                        <i class="fa fa-italic"></i>
                                    </Icon>
                                    <span>{"Italic"}</span>
                                </Button>
                            </ybc::Control>
                            <ybc::Control>
                                <Button>
                                    <Icon size={Some(ybc::Size::Small)}>
                                        <i class="fa fa-underline"></i>
                                    </Icon>
                                    <span>{"Underline"}</span>
                                </Button>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field addons=true>
                            <ybc::Control>
                                <Button>
                                    <Icon size={Some(ybc::Size::Small)}>
                                        <i class="fa fa-align-left"></i>
                                    </Icon>
                                    <span>{"Left"}</span>
                                </Button>
                            </ybc::Control>
                            <ybc::Control>
                                <Button>
                                    <Icon size={Some(ybc::Size::Small)}>
                                        <i class="fa fa-align-center"></i>
                                    </Icon>
                                    <span>{"Center"}</span>
                                </Button>
                            </ybc::Control>
                            <ybc::Control>
                                <Button>
                                    <Icon size={Some(ybc::Size::Small)}>
                                        <i class="fa fa-align-right"></i>
                                    </Icon>
                                    <span>{"Right"}</span>
                                </Button>
                            </ybc::Control>
                        </ybc::Field>
                        <Buttons>
                            <Button classes="is-rounded">{"Rounded"}</Button>
                            <Button classes="is-primary is-rounded">{"Rounded"}</Button>
                            <Button classes="is-link is-rounded">{"Rounded"}</Button>
                            <Button classes="is-info is-rounded">{"Rounded"}</Button>
                            <Button classes="is-success is-rounded">{"Rounded"}</Button>
                            <Button classes="is-danger is-rounded">{"Rounded"}</Button>
                        </Buttons>
                    </Column>
                    <Column>
                        <Buttons>
                            <Button classes="is-small">{"Small"}</Button>
                            <Button>{"Normal"}</Button>
                            <Button classes="is-medium">{"Medium"}</Button>
                            <Button classes="is-large">{"Large"}</Button>
                        </Buttons>
                        <Buttons>
                            <Button classes="is-outlined">{"Outlined"}</Button>
                            <Button classes="is-primary is-outlined">{"Outlined"}</Button>
                            <Button classes="is-info is-outlined">{"Outlined"}</Button>
                            <Button classes="is-success is-outlined">{"Outlined"}</Button>
                            <Button classes="is-danger is-outlined">{"Outlined"}</Button>
                        </Buttons>
                        <Buttons classes="notification is-primary">
                            <Button classes="is-primary is-inverted is-outlined">{"Invert Outlined"}</Button>
                            <Button classes="is-info is-inverted is-outlined">{"Invert Outlined"}</Button>
                            <Button classes="is-success is-inverted is-outlined">{"Invert Outlined"}</Button>
                            <Button classes="is-danger is-inverted is-outlined">{"Invert Outlined"}</Button>
                        </Buttons>
                        <Buttons classes="notification is-primary">
                            <Button classes="is-primary is-inverted">{"Inverted"}</Button>
                            <Button classes="is-info is-inverted">{"Inverted"}</Button>
                            <Button classes="is-success is-inverted">{"Inverted"}</Button>
                            <Button classes="is-danger is-inverted">{"Inverted"}</Button>
                        </Buttons>
                        <Buttons>
                            <Button classes="is-small">
                                <Icon size={Some(ybc::Size::Small)}>
                                    <i class="fab fa-github"></i>
                                </Icon>
                                <span>{"GitHub"}</span>
                            </Button>
                            <Button>
                                <Icon>
                                    <i class="fab fa-github"></i>
                                </Icon>
                                <span>{"GitHub"}</span>
                            </Button>
                            <Button classes="is-medium">
                                <Icon>
                                    <i class="fab fa-github"></i>
                                </Icon>
                                <span>{"GitHub"}</span>
                            </Button>
                            <Button classes="is-large">
                                <Icon size={Some(ybc::Size::Medium)}>
                                    <i class="fab fa-github"></i>
                                </Icon>
                                <span>{"GitHub"}</span>
                            </Button>
                        </Buttons>
                        <Buttons>
                            <Button classes="is-small">
                                <Icon size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-heading"></i>
                                </Icon>
                            </Button>
                        </Buttons>
                        <Buttons>
                            <Button>
                                <Icon size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-heading"></i>
                                </Icon>
                            </Button>
                            <Button>
                                <Icon>
                                    <i class="fa fa-heading fa-lg"></i>
                                </Icon>
                            </Button>
                        </Buttons>
                        <Buttons>
                            <Button classes="is-medium">
                                <Icon size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-heading"></i>
                                </Icon>
                            </Button>
                            <Button classes="is-medium">
                                <Icon>
                                    <i class="fa fa-heading fa-lg"></i>
                                </Icon>
                            </Button>
                            <Button classes="is-medium">
                                <Icon size={Some(ybc::Size::Medium)}>
                                    <i class="fa fa-heading fa-2x"></i>
                                </Icon>
                            </Button>
                        </Buttons>
                        <Buttons>
                            <Button classes="is-large">
                                <Icon size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-heading"></i>
                                </Icon>
                            </Button>
                            <Button classes="is-large">
                                <Icon size={Some(ybc::Size::Medium)}>
                                    <i class="fa fa-heading fa-lg"></i>
                                </Icon>
                            </Button>
                            <Button classes="is-large">
                                <Icon size={Some(ybc::Size::Large)}>
                                    <i class="fa fa-heading fa-2x"></i>
                                </Icon>
                            </Button>
                        </Buttons>
                    </Column>
                </Columns>
            </Section>
        </>
    }
}

fn main() {
    set_panic_hook();
    yew::Renderer::<App>::new().render();
}
