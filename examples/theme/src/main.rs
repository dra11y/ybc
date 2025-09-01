use console_error_panic_hook::set_once as set_panic_hook;
use ybc::{
    Block, Box as YBox, Button, Buttons, Column, Columns, Container, Content, Delete as YDelete, HeaderSize, Hero, Icon, Image, ImageSize, Level,
    LevelItem, LevelLeft, Media, MediaContent, MediaLeft, Message, MessageBody, MessageHeader, Navbar, NavbarItem, NavbarItemTag, Notification,
    Progress, Section, Subtitle, Table, Tag, Title,
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
                <Container>
                    <Title size={HeaderSize::Is1}>{"Bulma Yew"}</Title>
                    <Subtitle size={HeaderSize::Is3}>{"Subtitle"}</Subtitle>
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

#[function_component(HeroSection)]
fn hero_section() -> Html {
    fn hero_block(classes: &'static str, navbar_classes: &'static str) -> Html {
        html! {<>
            <Navbar
                classes={navbar_classes}
                padded=true
                spaced=true
                navbrand={html!{
                    <>
                        <NavbarItem>
                            <img src="https://bulma.io/images/bulma-type-white.png" alt="Logo" />
                        </NavbarItem>
                    </>
                }}
                navend={html!{
                    <>
                        <NavbarItem classes="is-active" tag={NavbarItemTag::A} href={"#".to_string()}> {"Home"} </NavbarItem>
                        <NavbarItem tag={NavbarItemTag::A} href={"#".to_string()}> {"Examples"} </NavbarItem>
                        <NavbarItem tag={NavbarItemTag::A} href={"#".to_string()}> {"Documentation"} </NavbarItem>
                        <ybc::NavbarDropdown hoverable=true navlink={html!{"More"}}>
                            <NavbarItem>
                                <div class="level is-mobile">
                                    <div class="level-left">
                                        <div class="level-item">
                                            <p>
                                                <strong>{"Extensions"}</strong><br/>
                                                <small>{"Side projects to enhance Bulma"}</small>
                                            </p>
                                        </div>
                                    </div>
                                </div>
                            </NavbarItem>
                        </ybc::NavbarDropdown>
                        <NavbarItem>
                            <Button classes="is-primary is-inverted">
                                <span class="icon"><i class="fab fa-github"></i></span>
                                <span>{"Download"}</span>
                            </Button>
                        </NavbarItem>
                    </>
                }}
            />
            <Hero classes={classes} body={html!{
                    <div class="container has-text-centered">
                        <Title>{"Title"}</Title>
                        <Subtitle>{"Subtitle"}</Subtitle>
                    </div>
                }} foot={Some(html!{
                    <nav class="tabs">
                        <div class="container">
                            <ul>
                                <li class="is-active"><a>{"Overview"}</a></li>
                                <li><a>{"Modifiers"}</a></li>
                                <li><a>{"Grid"}</a></li>
                                <li><a>{"Elements"}</a></li>
                                <li><a>{"Components"}</a></li>
                                <li><a>{"Layout"}</a></li>
                            </ul>
                        </div>
                    </nav>
                })} />
                <br />
        </>}
    }

    html! {
        <>
            <div id="hero"></div>
            <Section>
                <Title tag="h1">{"Hero"}</Title>
                <hr />
                {hero_block("hero", "navbar")}
                {hero_block("hero is-primary", "navbar is-primary")}
                {hero_block("hero is-link", "navbar is-link")}
                {hero_block("hero is-info", "navbar is-info")}
                {hero_block("hero is-success", "navbar is-success")}
                {hero_block("hero is-warning", "navbar is-warning")}
                {hero_block("hero is-danger", "navbar is-danger")}
                {hero_block("hero is-white", "navbar is-white")}
                {hero_block("hero is-black", "navbar is-black")}
                {hero_block("hero is-light", "navbar is-light")}
                {hero_block("hero is-dark", "navbar is-dark")}
            </Section>
        </>
    }
}
#[function_component(BreadcrumbSection)]
fn breadcrumb_section() -> Html {
    html! {
        <>
            <div id="breadcrumb"></div>
            <Section>
                <Title tag="h1">{"Breadcrumb"}</Title>
                <hr />
                <ybc::Breadcrumb>
                    <li><a>{"Bulma"}</a></li>
                    <li><a>{"Documentation"}</a></li>
                    <li><a>{"Components"}</a></li>
                    <li class="is-active"><a>{"Breadcrumb"}</a></li>
                </ybc::Breadcrumb>
            </Section>
        </>
    }
}

#[function_component(CardSection)]
fn card_section() -> Html {
    html! {
        <>
            <div id="card"></div>
            <Section>
                <Title tag="h1">{"Cards"}</Title>
                <hr />
                <Columns>
                    <Column>
                        <ybc::Card>
                            <ybc::CardImage>
                                <Image classes="is-4by3">
                                    <img alt="" src="https://picsum.photos/800/600" />
                                </Image>
                            </ybc::CardImage>
                            <ybc::CardContent>
                                <Media>
                                    <MediaLeft>
                                        <figure class="image" style="height: 40px; width: 40px;">
                                            <img src="https://source.unsplash.com/random/96x96" alt="Image" />
                                        </figure>
                                    </MediaLeft>
                                    <MediaContent>
                                        <Title tag="p" size={HeaderSize::Is4}>{"John Smith"}</Title>
                                        <Subtitle tag="p" size={HeaderSize::Is6}>{"@johnsmith"}</Subtitle>
                                    </MediaContent>
                                </Media>
                                <Content>
                                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus nec iaculis mauris."}
                                    <br />
                                    <a>{"@bulmaio"}</a>{"."}
                                    <a>{"#css"}</a>
                                    <a>{"#responsive"}</a>
                                    <br />
                                    <small>{"11:09 PM - 1 Jan 2016"}</small>
                                </Content>
                            </ybc::CardContent>
                        </ybc::Card>
                    </Column>
                    <Column>
                        <ybc::Card>
                            <ybc::CardHeader>
                                <p class="card-header-title">{"Component"}</p>
                                <a class="card-header-icon">
                                    <span class="icon"><i class="fa fa-angle-down"></i></span>
                                </a>
                            </ybc::CardHeader>
                            <ybc::CardContent>
                                <Content>
                                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus nec iaculis mauris."}
                                    <br />
                                    <a>{"@bulmaio"}</a>{"."}
                                    <a>{"#css"}</a>
                                    <a>{"#responsive"}</a>
                                    <br />
                                    <small>{"11:09 PM - 1 Jan 2016"}</small>
                                </Content>
                            </ybc::CardContent>
                            <ybc::CardFooter>
                                <a class="card-footer-item">{"Save"}</a>
                                <a class="card-footer-item">{"Edit"}</a>
                                <a class="card-footer-item">{"Delete"}</a>
                            </ybc::CardFooter>
                        </ybc::Card>
                    </Column>
                </Columns>
            </Section>
        </>
    }
}

#[function_component(DropdownSection)]
fn dropdown_section() -> Html {
    let open1 = use_state(|| false);
    let open2 = use_state(|| false);
    html! {
        <>
            <div id="dropdown"></div>
            <Section>
                <Title tag="h1">{"Dropdown"}</Title>
                <hr />
                <Columns>
                    <Column>
                        <ybc::Dropdown active={*open1} on_active_change={{let open1 = open1.clone(); Callback::from(move |v| open1.set(v))}} button_html={html!{
                            <>
                                <span>{"Dropdown button"}</span>
                                <span class="icon is-small"><i class="fa fa-angle-down" aria-hidden="true"></i></span>
                            </>
                        }}>
                            <a href="#" class="dropdown-item">{"Dropdown item"}</a>
                            <a class="dropdown-item">{"Other dropdown item"}</a>
                            <a href="#" class="dropdown-item is-active">{"Active dropdown item"}</a>
                            <a href="#" class="dropdown-item">{"Other dropdown item"}</a>
                            <hr class="dropdown-divider" />
                            <a href="#" class="dropdown-item">{"With a divider"}</a>
                        </ybc::Dropdown>
                    </Column>
                    <Column>
                        <ybc::Dropdown active={*open2} on_active_change={{let open2 = open2.clone(); Callback::from(move |v| open2.set(v))}} button_classes="is-info" button_html={html!{
                            <>
                                <span>{"Content"}</span>
                                <span class="icon is-small"><i class="fa fa-angle-down" aria-hidden="true"></i></span>
                            </>
                        }}>
                            <div class="dropdown-item">
                                <p>{"You can insert "}<strong>{"any type of content"}</strong>{" within the dropdown menu."}</p>
                            </div>
                            <hr class="dropdown-divider" />
                            <div class="dropdown-item">
                                <p>{"You simply need to use a "}<code>{"<div>"}</code>{" instead."}</p>
                            </div>
                            <hr class="dropdown-divider" />
                            <a href="#" class="dropdown-item">{"This is a link"}</a>
                        </ybc::Dropdown>
                    </Column>
                </Columns>
            </Section>
        </>
    }
}

#[function_component(MediaSection)]
fn media_section() -> Html {
    html! {
        <>
            <div id="media"></div>
            <Section>
                <Title tag="h1">{"Media Object"}</Title>
                <hr />
                <Media>
                    <MediaLeft>
                        <Image size={Some(ImageSize::Is64x64)}>
                            <img alt="Image" src="https://placehold.net/avatar-2.svg" />
                        </Image>
                    </MediaLeft>
                    <MediaContent>
                        <Content>
                            <p>
                                <strong>{"John Smith"}</strong>{" "}
                                <small>{"@johnsmith"}</small>{" "}
                                <small>{"31m"}</small>
                                <br/>
                                {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin ornare magna eros, eu pellentesque tortor vestibulum ut. Maecenas non massa sem. Etiam finibus odio quis feugiat facilisis."}
                            </p>
                        </Content>
                        <Level>
                            <LevelLeft>
                                <a class="level-item"><span class="icon is-small"><i class="fa fa-reply"></i></span></a>
                                <a class="level-item"><span class="icon is-small"><i class="fa fa-retweet"></i></span></a>
                                <a class="level-item"><span class="icon is-small"><i class="fa fa-heart"></i></span></a>
                            </LevelLeft>
                        </Level>
                    </MediaContent>
                    <ybc::MediaRight>
                        <YDelete />
                    </ybc::MediaRight>
                </Media>
                <hr />
                <Media>
                    <MediaLeft>
                        <Image size={Some(ImageSize::Is64x64)}>
                            <img alt="Image" src="https://placehold.net/avatar-5.svg" />
                        </Image>
                    </MediaLeft>
                    <MediaContent>
                        <div class="field">
                            <p class="control">
                                <textarea class="textarea" placeholder="Add a comment..."></textarea>
                            </p>
                        </div>
                        <Level>
                            <LevelLeft>
                                <div class="level-item">
                                    <a class="button is-info">{"Post comment"}</a>
                                </div>
                            </LevelLeft>
                            <ybc::LevelRight>
                                <div class="level-item">
                                    <label class="checkbox"><input type="checkbox" /> {" Press enter to submit"}</label>
                                </div>
                            </ybc::LevelRight>
                        </Level>
                    </MediaContent>
                </Media>
                <hr />
                <Subtitle tag="h4">{"Nesting"}</Subtitle>
                <Media>
                    <MediaLeft>
                        <Image size={Some(ImageSize::Is64x64)}>
                            <img alt="Image" src="https://placehold.net/avatar-3.svg" />
                        </Image>
                    </MediaLeft>
                    <MediaContent>
                        <Content>
                            <p>
                                <strong>{"Barbara Middleton"}</strong>
                                <br/>
                                {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis porta eros lacus, nec ultricies elit blandit non. Suspendisse pellentesque mauris sit amet dolor blandit rutrum. Nunc in tempus turpis."}
                                <br/>
                                <small><a>{"Like"}</a>{" · "}<a>{"Reply"}</a>{" · 3 hrs"}</small>
                            </p>
                        </Content>
                        <Media>
                            <MediaLeft>
                                <Image size={Some(ImageSize::Is64x64)}>
                                    <img alt="Image" src="https://placehold.net/avatar-4.svg" />
                                </Image>
                            </MediaLeft>
                            <MediaContent>
                                <Content>
                                    <p>
                                        <strong>{"Sean Brown"}</strong>
                                        <br/>
                                        {"Donec sollicitudin urna eget eros malesuada sagittis. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Aliquam blandit nisl a nulla sagittis, a lobortis leo feugiat."}
                                        <br/>
                                        <small><a>{"Like"}</a>{" · "}<a>{"Reply"}</a>{" · 2 hrs"}</small>
                                    </p>
                                </Content>
                                <Media>{"Vivamus quis semper metus, non tincidunt dolor. Vivamus in mi eu lorem cursus ullamcorper sit amet nec massa."}</Media>
                                <Media>{"Morbi vitae diam et purus tincidunt porttitor vel vitae augue. Praesent malesuada metus sed pharetra euismod. Cras tellus odio, tincidunt iaculis diam non, porta aliquet tortor."}</Media>
                            </MediaContent>
                        </Media>
                        <Media>
                            <MediaLeft>
                                <Image size={Some(ImageSize::Is64x64)}>
                                    <img alt="Image" src="https://placehold.net/avatar-1.svg" />
                                </Image>
                            </MediaLeft>
                            <MediaContent>
                                <Content>
                                    <p>
                                        <strong>{"Kayli Eunice "}</strong>
                                        <br/>
                                        {"Sed convallis scelerisque mauris, non pulvinar nunc mattis vel. Maecenas varius felis sit amet magna vestibulum euismod malesuada cursus libero. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Phasellus lacinia non nisl id feugiat."}
                                        <br/>
                                        <small><a>{"Like"}</a>{" · "}<a>{"Reply"}</a>{" · 2 hrs"}</small>
                                    </p>
                                </Content>
                            </MediaContent>
                        </Media>
                    </MediaContent>
                </Media>
            </Section>
        </>
    }
}

#[function_component(MenuSection)]
fn menu_section() -> Html {
    html! {
        <>
            <div id="menu"></div>
            <Section>
                <Title tag="h1">{"Menu"}</Title>
                <hr />
                <div class="column is-3">
                    <ybc::Menu>
                        <ybc::MenuLabel text="General" />
                        <ybc::MenuList>
                            <li><a>{"Dashboard"}</a></li>
                            <li><a>{"Customers"}</a></li>
                        </ybc::MenuList>
                        <ybc::MenuLabel text="Administration" />
                        <ybc::MenuList>
                            <li><a>{"Team Settings"}</a></li>
                            <li>
                                <a class="is-active">{"Manage Your Team"}</a>
                                <ul>
                                    <li><a>{"Members"}</a></li>
                                    <li><a>{"Plugins"}</a></li>
                                    <li><a>{"Add a member"}</a></li>
                                </ul>
                            </li>
                            <li><a>{"Invitations"}</a></li>
                            <li><a>{"Cloud Storage Environment Settings"}</a></li>
                            <li><a>{"Authentication"}</a></li>
                        </ybc::MenuList>
                        <ybc::MenuLabel text="Transactions" />
                        <ybc::MenuList>
                            <li><a>{"Payments"}</a></li>
                            <li><a>{"Transfers"}</a></li>
                            <li><a>{"Balance"}</a></li>
                        </ybc::MenuList>
                    </ybc::Menu>
                </div>
            </Section>
        </>
    }
}

#[function_component(MessageSection)]
fn message_section() -> Html {
    let variants = vec![
        ("", "Message"),
        ("is-primary", "Primary"),
        ("is-link", "Link"),
        ("is-info", "Info"),
        ("is-success", "Success"),
        ("is-warning", "Warning"),
        ("is-danger", "Danger"),
        ("is-white", "White"),
        ("is-black", "Black"),
        ("is-light", "Light"),
        ("is-dark", "Dark"),
    ];
    html! {
        <>
            <div id="message"></div>
            <Section>
                <Title tag="h1">{"Message"}</Title>
                <hr />
                <Columns classes="is-multiline">
                    { for variants.into_iter().map(|(class, title)| html!{
                        <Column classes="is-half">
                            <Message classes={class}>
                                <MessageHeader>
                                    <p>{title}</p>
                                    <YDelete />
                                </MessageHeader>
                                <MessageBody>
                                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. "}
                                    <strong>{"Pellentesque risus mi"}</strong>
                                    {", tempus quis placerat ut, porta nec nulla. Vestibulum rhoncus ac ex sit amet fringilla. Nullam gravida purus diam, et dictum "}
                                    <a>{"felis venenatis"}</a>
                                    {" efficitur. Aenean ac "}
                                    <em>{"eleifend lacus"}</em>
                                    {"."}
                                </MessageBody>
                            </Message>
                        </Column>
                    }) }
                </Columns>
            </Section>
        </>
    }
}

#[function_component(ModalSection)]
fn modal_section() -> Html {
    let handle_basic = ybc::use_modal();
    let handle_card = ybc::use_modal();
    html! {
        <>
            <div id="modal"></div>
            <Section>
                <Title tag="h1">{"Modal"}</Title>
                <hr />
                <div class="buttons">
                    <Button classes="is-primary is-large" onclick={handle_basic.open_callback()}>{"Launch basic modal"}</Button>
                    <Button classes="is-info is-large" onclick={handle_card.open_callback()}>{"Launch modal card"}</Button>
                </div>

                <ybc::Modal id={"basicModal".to_string()} handle={handle_basic.clone()}>
                    <ybc::Card>
                        <ybc::CardContent>
                            <Content>
                                <Title size={HeaderSize::Is4}>{"Modal Content"}</Title>
                                <p>{"This is a basic modal using the Modal component. It includes a card for proper spacing and visual hierarchy."}</p>
                                <p>{"You can click the background or the X button to close this modal."}</p>
                            </Content>
                        </ybc::CardContent>
                    </ybc::Card>
                </ybc::Modal>

                <ybc::ModalCard
                    id={"myModal".to_string()}
                    handle={handle_card.clone()}
                    title={"Modal title".to_string()}
                    body={html!{<>
                        {"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. "}
                        {"Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. "}
                        {"Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. "}
                        {"Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                    </>}}
                    footer={html!{
                        <>
                            <Button classes="is-primary" onclick={handle_card.close_callback()}>{"Save changes"}</Button>
                            <Button onclick={handle_card.close_callback()}>{"Cancel"}</Button>
                        </>
                    }}
                />
            </Section>
        </>
    }
}

#[function_component(NavbarSection)]
fn navbar_section() -> Html {
    let brand = html! {
        <NavbarItem tag={NavbarItemTag::A} href={"https://bulma.io".to_string()}>
            <img src="https://bulma.io/images/bulma-logo.png" alt="Bulma" width="112" height="28" />
        </NavbarItem>
    };
    let navstart = html! {
        <>
            <NavbarItem tag={NavbarItemTag::A} href={"#".to_string()}>{"Home"}</NavbarItem>
            <ybc::NavbarDropdown hoverable=true navlink={html!{<span class="is-active">{"Docs"}</span>}}>
                <NavbarItem tag={NavbarItemTag::A} href={"#".to_string()}>{"Overview"}</NavbarItem>
                <NavbarItem tag={NavbarItemTag::A} href={"#".to_string()}>{"Modifiers"}</NavbarItem>
                <NavbarItem tag={NavbarItemTag::A} href={"#".to_string()}>{"Grid"}</NavbarItem>
                <NavbarItem tag={NavbarItemTag::A} href={"#".to_string()}>{"Form"}</NavbarItem>
                <NavbarItem tag={NavbarItemTag::A} href={"#".to_string()}>{"Elements"}</NavbarItem>
                <NavbarItem classes="is-active" tag={NavbarItemTag::A} href={"#".to_string()}>{"Components"}</NavbarItem>
                <NavbarItem tag={NavbarItemTag::A} href={"#".to_string()}>{"Layout"}</NavbarItem>
                <ybc::NavbarDivider />
                <NavbarItem>
                    <div>{"version"}
                        <p class="has-text-info is-size-6-desktop">{"0.4.3"}</p>
                    </div>
                </NavbarItem>
            </ybc::NavbarDropdown>
            <ybc::NavbarDropdown hoverable=true navlink={html!{"Blog"}}>
                <NavbarItem>
                    <div class="navbar-content">
                        <p><small class="has-text-info">{"10 Mar 2017"}</small></p>
                        <p>{"New field element (for better controls)"}</p>
                    </div>
                </NavbarItem>
                <NavbarItem>
                    <div class="navbar-content">
                        <p><small class="has-text-info">{"11 Apr 2016"}</small></p>
                        <p>{"Metro UI CSS grid with Bulma tiles"}</p>
                    </div>
                </NavbarItem>
                <NavbarItem>
                    <div class="navbar-content">
                        <p><small class="has-text-info">{"09 Feb 2016"}</small></p>
                        <p>{"Blog launched, new responsive columns, new helpers"}</p>
                    </div>
                </NavbarItem>
                <NavbarItem tag={NavbarItemTag::A} href={"#blog/".to_string()}>{"More posts"}</NavbarItem>
                <ybc::NavbarDivider />
                <NavbarItem>
                    <div class="navbar-content">
                        <div class="level is-mobile">
                            <div class="level-left">
                                <div class="level-item"><strong>{"Stay up to date!"}</strong></div>
                            </div>
                            <div class="level-right">
                                <div class="level-item">
                                    <a class="button is-rss is-small" href="#atom.xml">
                                        <span class="icon is-small"><i class="fas fa-rss"></i></span>
                                        <span>{"Subscribe"}</span>
                                    </a>
                                </div>
                            </div>
                        </div>
                    </div>
                </NavbarItem>
            </ybc::NavbarDropdown>
            <ybc::NavbarDropdown hoverable=true navlink={html!{"More"}}>
                <NavbarItem>
                    <div class="level is-mobile">
                        <div class="level-left">
                            <div class="level-item">
                                <p><strong>{"Extensions"}</strong><br/><small>{"Side projects to enhance Bulma"}</small></p>
                            </div>
                        </div>
                    </div>
                </NavbarItem>
            </ybc::NavbarDropdown>
        </>
    };
    let navend = html! {
        <>
            <NavbarItem tag={NavbarItemTag::A} href={"https://github.com/jgthms/bulma".to_string()} target={Some("_blank".to_string())}>{"Github"}</NavbarItem>
            <NavbarItem tag={NavbarItemTag::A} href={"https://twitter.com/jgthms".to_string()} target={Some("_blank".to_string())}>{"Twitter"}</NavbarItem>
            <NavbarItem>
                <div class="field is-grouped">
                    <p class="control"><a id="twitter" class="button"><span>{"Tweet"}</span></a></p>
                    <p class="control"><a class="button is-primary" href="https://github.com/jgthms/bulma/archive/0.4.3.zip">
                        <span class="icon"><i class="fas fa-download"></i></span>
                        <span>{"Download"}</span>
                    </a></p>
                </div>
            </NavbarItem>
        </>
    };
    html! {
        <>
            <div id="navbar"></div>
            <Section>
                <Title tag="h1">{"Navbar"}</Title>
                <hr />
                <Navbar spaced=true padded=true navbrand={brand.clone()} navstart={navstart.clone()} navend={navend.clone()} />
                <br />
                <Navbar classes="is-primary" spaced=true padded=true navbrand={brand.clone()} navstart={navstart.clone()} navend={navend.clone()} />
                <br />
                <Navbar classes="is-link" spaced=true padded=true navbrand={brand.clone()} navstart={navstart.clone()} navend={navend.clone()} />
                <br />
                <Navbar classes="is-info" spaced=true padded=true navbrand={brand.clone()} navstart={navstart.clone()} navend={navend.clone()} />
                <br />
                <Navbar classes="is-success" spaced=true padded=true navbrand={brand.clone()} navstart={navstart.clone()} navend={navend.clone()} />
                <br />
                <Navbar classes="is-warning" spaced=true padded=true navbrand={brand.clone()} navstart={navstart.clone()} navend={navend.clone()} />
                <br />
                <Navbar classes="is-danger" spaced=true padded=true navbrand={brand.clone()} navstart={navstart.clone()} navend={navend.clone()} />
                <br />
                <Navbar classes="is-white" spaced=true padded=true navbrand={brand.clone()} navstart={navstart.clone()} navend={navend.clone()} />
                <br />
                <Navbar classes="is-black" spaced=true padded=true navbrand={brand.clone()} navstart={navstart.clone()} navend={navend.clone()} />
                <br />
                <Navbar classes="is-light" spaced=true padded=true navbrand={brand.clone()} navstart={navstart.clone()} navend={navend.clone()} />
                <br />
                <Navbar classes="is-dark" spaced=true padded=true navbrand={brand.clone()} navstart={navstart.clone()} navend={navend.clone()} />
                <br />
                <Navbar classes="is-transparent" spaced=true padded=true navbrand={brand.clone()} navstart={navstart.clone()} navend={navend.clone()} />
            </Section>
        </>
    }
}

#[function_component(PaginationSection)]
fn pagination_section() -> Html {
    html! {
        <>
            <div id="pagination"></div>
            <Section>
                <Title tag="h1">{"Pagination"}</Title>
                <hr />
                <ybc::Pagination
                    previous={html!{<ybc::PaginationItem item_type={ybc::PaginationItemType::Previous}>{"Previous"}</ybc::PaginationItem>}}
                    next={html!{<ybc::PaginationItem item_type={ybc::PaginationItemType::Next}>{"Next page"}</ybc::PaginationItem>}}
                >
                    <li><ybc::PaginationItem item_type={ybc::PaginationItemType::Link}>{"1"}</ybc::PaginationItem></li>
                    <li><ybc::PaginationEllipsis /></li>
                    <li><ybc::PaginationItem item_type={ybc::PaginationItemType::Link}>{"45"}</ybc::PaginationItem></li>
                    <li><ybc::PaginationItem item_type={ybc::PaginationItemType::Link}>{"46"}</ybc::PaginationItem></li>
                    <li><a class="pagination-link is-current" aria-label="Page 47" aria-current="page">{"47"}</a></li>
                    <li><ybc::PaginationItem item_type={ybc::PaginationItemType::Link}>{"48"}</ybc::PaginationItem></li>
                    <li><ybc::PaginationItem item_type={ybc::PaginationItemType::Link}>{"49"}</ybc::PaginationItem></li>
                    <li><ybc::PaginationEllipsis /></li>
                    <li><ybc::PaginationItem item_type={ybc::PaginationItemType::Link}>{"86"}</ybc::PaginationItem></li>
                </ybc::Pagination>
                <br />
                <ybc::Pagination
                    rounded=true
                    previous={html!{<ybc::PaginationItem item_type={ybc::PaginationItemType::Previous}>{"Previous"}</ybc::PaginationItem>}}
                    next={html!{<ybc::PaginationItem item_type={ybc::PaginationItemType::Next}>{"Next page"}</ybc::PaginationItem>}}
                >
                    <li><ybc::PaginationItem item_type={ybc::PaginationItemType::Link}>{"1"}</ybc::PaginationItem></li>
                    <li><ybc::PaginationEllipsis /></li>
                    <li><ybc::PaginationItem item_type={ybc::PaginationItemType::Link}>{"45"}</ybc::PaginationItem></li>
                    <li><ybc::PaginationItem item_type={ybc::PaginationItemType::Link}>{"46"}</ybc::PaginationItem></li>
                    <li><a class="pagination-link is-current" aria-label="Page 47" aria-current="page">{"47"}</a></li>
                    <li><ybc::PaginationItem item_type={ybc::PaginationItemType::Link}>{"48"}</ybc::PaginationItem></li>
                    <li><ybc::PaginationItem item_type={ybc::PaginationItemType::Link}>{"49"}</ybc::PaginationItem></li>
                    <li><ybc::PaginationEllipsis /></li>
                    <li><ybc::PaginationItem item_type={ybc::PaginationItemType::Link}>{"86"}</ybc::PaginationItem></li>
                </ybc::Pagination>
            </Section>
        </>
    }
}

#[function_component(PanelSection)]
fn panel_section() -> Html {
    let active = use_state(|| "all".to_string());
    let on_all = {
        let active = active.clone();
        Callback::from(move |_| active.set("all".to_string()))
    };
    let on_public = {
        let active = active.clone();
        Callback::from(move |_| active.set("public".to_string()))
    };
    let on_private = {
        let active = active.clone();
        Callback::from(move |_| active.set("private".to_string()))
    };
    let on_sources = {
        let active = active.clone();
        Callback::from(move |_| active.set("sources".to_string()))
    };
    let on_forks = {
        let active = active.clone();
        Callback::from(move |_| active.set("forks".to_string()))
    };

    html! {
        <>
            <div id="panel"></div>
            <Section>
                <Title tag="h1">{"Panel"}</Title>
                <hr />
                <ybc::Panel heading={html!{"Repositories"}}>
                    <ybc::PanelTabs>
                        <a class={classes!((*active == "all").then_some("is-active"))} onclick={on_all}>{"All"}</a>
                        <a class={classes!((*active == "public").then_some("is-active"))} onclick={on_public}>{"Public"}</a>
                        <a class={classes!((*active == "private").then_some("is-active"))} onclick={on_private}>{"Private"}</a>
                        <a class={classes!((*active == "sources").then_some("is-active"))} onclick={on_sources}>{"Sources"}</a>
                        <a class={classes!((*active == "forks").then_some("is-active"))} onclick={on_forks}>{"Forks"}</a>
                    </ybc::PanelTabs>

                    <ybc::PanelBlock>
                        <p class="control has-icons-left">
                            <input class="input" type="text" placeholder="Search" />
                            <span class="icon is-left"><i class="fas fa-search" aria-hidden="true"></i></span>
                        </p>
                    </ybc::PanelBlock>

                    <ybc::PanelBlock active=true>
                        <span class="panel-icon"><i class="fas fa-book" aria-hidden="true"></i></span>
                        <span>{"bulma"}</span>
                    </ybc::PanelBlock>
                    <ybc::PanelBlock>
                        <span class="panel-icon"><i class="fas fa-book" aria-hidden="true"></i></span>
                        <span>{"marksheet"}</span>
                    </ybc::PanelBlock>
                    <ybc::PanelBlock>
                        <span class="panel-icon"><i class="fas fa-book" aria-hidden="true"></i></span>
                        <span>{"minireset.css"}</span>
                    </ybc::PanelBlock>
                    <ybc::PanelBlock>
                        <span class="panel-icon"><i class="fas fa-book" aria-hidden="true"></i></span>
                        <span>{"jgthms.github.io"}</span>
                    </ybc::PanelBlock>
                    <ybc::PanelBlock>
                        <span class="panel-icon"><i class="fas fa-code-branch" aria-hidden="true"></i></span>
                        <span>{"danielroseman/streaming"}</span>
                    </ybc::PanelBlock>
                    <ybc::PanelBlock>
                        <span class="panel-icon"><i class="fas fa-code-branch" aria-hidden="true"></i></span>
                        <span>{"mojs"}</span>
                    </ybc::PanelBlock>

                    <ybc::PanelBlock tag={"label".to_string()}>
                        <input type="checkbox" />
                        <span>{"Remember me"}</span>
                    </ybc::PanelBlock>

                    <ybc::PanelBlock>
                        <Button classes="is-link is-outlined is-fullwidth">{"Reset all filters"}</Button>
                    </ybc::PanelBlock>
                </ybc::Panel>
            </Section>
        </>
    }
}

#[function_component(TabsSection)]
fn tabs_section() -> Html {
    let tabs_vec = vec![
        ybc::Tab {
            id: "pictures".into(),
            label: "Pictures".into(),
            icon_class: Some("fas fa-image".into()),
        },
        ybc::Tab {
            id: "music".into(),
            label: "Music".into(),
            icon_class: Some("fas fa-music".into()),
        },
        ybc::Tab {
            id: "videos".into(),
            label: "Videos".into(),
            icon_class: Some("fas fa-film".into()),
        },
        ybc::Tab {
            id: "documents".into(),
            label: "Documents".into(),
            icon_class: Some("far fa-file-alt".into()),
        },
    ];
    html! {
        <>
            <div id="tabs"></div>
            <Section>
                <Title tag="h1">{"Tabs"}</Title>
                <hr />
                <ybc::Tabs tabs={tabs_vec.clone()} />
                <br />
                <ybc::Tabs alignment={Some(ybc::Alignment::Centered)} tabs={tabs_vec.clone()} />
                <br />
                <ybc::Tabs size={Some(ybc::Size::Small)} tabs={tabs_vec.clone()} />
                <br />
                <ybc::Tabs size={Some(ybc::Size::Medium)} tabs={tabs_vec.clone()} />
                <br />
                <ybc::Tabs size={Some(ybc::Size::Large)} tabs={tabs_vec.clone()} />
                <br />
                <ybc::Tabs boxed=true tabs={tabs_vec.clone()} />
                <br />
                <ybc::Tabs toggle=true tabs={tabs_vec.clone()} />
                <br />
                <ybc::Tabs toggle=true rounded=true tabs={tabs_vec.clone()} />
                <br />
                <ybc::Tabs fullwidth=true tabs={tabs_vec.clone()} />
                <br />
                <ybc::Tabs alignment={Some(ybc::Alignment::Centered)} boxed=true tabs={tabs_vec.clone()} />
                <br />
                <ybc::Tabs toggle=true fullwidth=true size={Some(ybc::Size::Large)} tabs={tabs_vec.clone()} />
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

#[function_component(IconSection)]
fn icon_section() -> Html {
    html! {
        <>
            <div id="icon"></div>
            <Section>
                <Title tag="h1" size={HeaderSize::Is1}>{"Icon"}</Title>
                <hr />
                <Columns>
                    <Column>
                        <Icon size={Some(ybc::Size::Small)}>
                            <i class="fas fa-home fa-sm"></i>
                        </Icon>
                        {" "}
                        <Icon>
                            <i class="fas fa-home"></i>
                        </Icon>
                        {" "}
                        <Icon size={Some(ybc::Size::Medium)}>
                            <i class="fas fa-home fa-lg"></i>
                        </Icon>
                        {" "}
                        <Icon size={Some(ybc::Size::Large)}>
                            <i class="fas fa-home fa-2x"></i>
                        </Icon>
                    </Column>
                </Columns>

                <Title tag="h2" size={HeaderSize::Is4}>{"Icon text"}</Title>
                <Block>
                    <span class="icon-text">
                        <span class="icon"><i class="fas fa-home"></i></span>
                        <span>{"Home"}</span>
                    </span>
                </Block>

                <Block>
                    <span class="icon-text">
                        <span class="icon"><i class="fas fa-train"></i></span>
                        <span>{"Paris"}</span>
                        <span class="icon"><i class="fas fa-arrow-right"></i></span>
                        <span>{"Budapest"}</span>
                        <span class="icon"><i class="fas fa-arrow-right"></i></span>
                        <span>{"Bucharest"}</span>
                        <span class="icon"><i class="fas fa-arrow-right"></i></span>
                        <span>{"Istanbul"}</span>
                        <span class="icon"><i class="fas fa-flag-checkered"></i></span>
                    </span>
                </Block>

                <Content>
                    <p>
                        {"An invitation to "}
                        <span class="icon-text">
                            <span class="icon"><i class="fas fa-utensils"></i></span>
                            <span>{"dinner"}</span>
                        </span>
                        {" was soon afterwards dispatched; and already had Mrs. Bennet planned the courses that were to do credit to her housekeeping, when an answer arrived which deferred it all. Mr. Bingley was obliged to be in "}
                        <span class="icon-text">
                            <span class="icon"><i class="fas fa-city"></i></span>
                            <span>{"town"}</span>
                        </span>
                        {", the following day, and, consequently, unable to accept the honour of their "}
                        <span class="icon-text">
                            <span class="icon"><i class="fas fa-envelope-open-text"></i></span>
                            <span>{"invitation"}</span>
                        </span>
                        {", etc."}
                    </p>

                    <p>
                        {"Mrs. Bennet was quite disconcerted. She could not imagine what business he could have in "}
                        <span class="icon-text">
                            <span class="icon"><i class="fas fa-flag-checkered"></i></span>
                            <span>{"arrival"}</span>
                        </span>
                        {" in Hertfordshire; and she began to fear that he might be always "}
                        <span class="icon-text">
                            <span class="icon"><i class="fas fa-plane-departure"></i></span>
                            <span>{"flying"}</span>
                        </span>
                        {" about from one place to another, and never settled at Netherfield as he ought to be."}
                    </p>
                </Content>

                <Block>
                    <div class="icon-text">
                        <span class="icon has-text-info"><i class="fas fa-info-circle"></i></span>
                        <span>{"Information"}</span>
                    </div>
                    <p class="block">{"Your package will be delivered on "}<strong>{"Tuesday at 08:00"}</strong>{"."}</p>

                    <div class="icon-text">
                        <span class="icon has-text-success"><i class="fas fa-check-square"></i></span>
                        <span>{"Success"}</span>
                    </div>
                    <p class="block">{"Your image has been successfully uploaded."}</p>

                    <div class="icon-text">
                        <span class="icon has-text-warning"><i class="fas fa-exclamation-triangle"></i></span>
                        <span>{"Warning"}</span>
                    </div>
                    <p class="block">{"Some information is missing from your "}<a href="#">{"profile"}</a>{" details."}</p>

                    <div class="icon-text">
                        <span class="icon has-text-danger"><i class="fas fa-ban"></i></span>
                        <span>{"Danger"}</span>
                    </div>
                    <p class="block">{"There was an error in your submission. "}<a href="#">{"Please try again"}</a>{"."}</p>
                </Block>

                <Title tag="h2" size={HeaderSize::Is5}>{"Colors"}</Title>
                <Block>
                    <span class="icon has-text-info"><i class="fas fa-info-circle"></i></span>
                    <span class="icon has-text-success"><i class="fas fa-check-square"></i></span>
                    <span class="icon has-text-warning"><i class="fas fa-exclamation-triangle"></i></span>
                    <span class="icon has-text-danger"><i class="fas fa-ban"></i></span>
                </Block>

                <Block>
                    <span class="icon-text has-text-info">
                        <span class="icon"><i class="fas fa-info-circle"></i></span>
                        <span>{"Info"}</span>
                    </span>
                    {" "}
                    <span class="icon-text has-text-success">
                        <span class="icon"><i class="fas fa-check-square"></i></span>
                        <span>{"Success"}</span>
                    </span>
                    {" "}
                    <span class="icon-text has-text-warning">
                        <span class="icon"><i class="fas fa-exclamation-triangle"></i></span>
                        <span>{"Warning"}</span>
                    </span>
                    {" "}
                    <span class="icon-text has-text-danger">
                        <span class="icon"><i class="fas fa-ban"></i></span>
                        <span>{"Danger"}</span>
                    </span>
                </Block>
            </Section>
        </>
    }
}

#[function_component(ImagesSection)]
fn images_section() -> Html {
    html! {
        <>
            <div id="images"></div>
            <Section>
                <Title tag="h1" size={HeaderSize::Is1}>{"Images"}</Title>
                <hr />
                <div class="fixed-grid has-7-cols is-flex">
                    <div class="grid is-align-items-center is-justify-content-center">
                        // https://images.pexels.com/photos/33562244/pexels-photo-33562244.jpeg

                        <div class="cell">
                            <Image size={Some(ImageSize::Is16x16)}>
                                <img alt="" src="https://placehold.net/1.png" />
                            </Image>
                        </div>
                        <div class="cell">
                            <Image size={Some(ImageSize::Is24x24)}>
                                <img alt="" src="https://placehold.net/2.png" />
                            </Image>
                        </div>
                        <div class="cell">
                            <Image size={Some(ImageSize::Is32x32)}>
                                <img alt="" src="https://placehold.net/5.png" />
                            </Image>
                        </div>
                        <div class="cell">
                            <Image size={Some(ImageSize::Is48x48)}>
                                <img alt="" src="https://placehold.net/6.png" />
                            </Image>
                        </div>
                        <div class="cell">
                            <Image size={Some(ImageSize::Is64x64)}>
                                <img alt="" src="https://placehold.net/7.png" />
                            </Image>
                        </div>
                        <div class="cell">
                            <Image size={Some(ImageSize::Is96x96)}>
                                <img alt="" src="https://placehold.net/4.png" />
                            </Image>
                        </div>
                        <div class="cell">
                            <Image size={Some(ImageSize::Is128x128)}>
                                <img alt="" src="https://placehold.net/3.png" />
                            </Image>
                        </div>

                        <div class="cell">
                            <Image size={Some(ImageSize::Is128x128)}>
                                <img class="is-rounded" alt="" src="https://placehold.net/3.png" />
                            </Image>
                        </div>
                        <div class="cell">
                            <Image size={Some(ImageSize::Is96x96)}>
                                <img class="is-rounded" alt="" src="https://placehold.net/4.png" />
                            </Image>
                        </div>
                        <div class="cell">
                            <Image size={Some(ImageSize::Is64x64)}>
                                <img class="is-rounded" alt="" src="https://placehold.net/7.png" />
                            </Image>
                        </div>
                        <div class="cell">
                            <Image size={Some(ImageSize::Is48x48)}>
                                <img class="is-rounded" alt="" src="https://placehold.net/6.png" />
                            </Image>
                        </div>
                        <div class="cell">
                            <Image size={Some(ImageSize::Is32x32)}>
                                <img class="is-rounded" alt="" src="https://placehold.net/5.png" />
                            </Image>
                        </div>
                        <div class="cell">
                            <Image size={Some(ImageSize::Is24x24)}>
                                <img class="is-rounded" alt="" src="https://placehold.net/2.png" />
                            </Image>
                        </div>
                        <div class="cell">
                            <Image size={Some(ImageSize::Is16x16)}>
                                <img class="is-rounded" alt="" src="https://placehold.net/1.png" />
                            </Image>
                        </div>
                    </div>
                </div>
            </Section>
        </>
    }
}

#[function_component(NotificationsSection)]
fn notifications_section() -> Html {
    html! {
        <>
            <div id="notifications"></div>
            <Section>
                <Title tag="h1" size={HeaderSize::Is1}>{"Notifications"}</Title>
                <hr />
                <Columns classes="is-multiline">
                    <Column classes="is-half">
                        <Notification>
                            <YDelete />
                            {"Lorem ipsum dolor sit amet, "}
                            <a href="#">{"consectetur"}</a>
                            {" adipiscing elit lorem ipsum dolor sit amet, consectetur adipiscing elit"}
                        </Notification>
                    </Column>
                    <Column classes="is-half">
                        <Notification classes="is-primary">
                            <YDelete />
                            {"Lorem ipsum dolor sit amet, "}
                            <a href="#">{"consectetur"}</a>
                            {" adipiscing elit lorem ipsum dolor sit amet, consectetur adipiscing elit"}
                        </Notification>
                    </Column>
                    <Column classes="is-half">
                        <Notification classes="is-link">
                            <YDelete />
                            {"Lorem ipsum dolor sit amet, "}
                            <a href="#">{"consectetur"}</a>
                            {" adipiscing elit lorem ipsum dolor sit amet, consectetur adipiscing elit"}
                        </Notification>
                    </Column>
                    <Column classes="is-half">
                        <Notification classes="is-info">
                            <YDelete />
                            {"Lorem ipsum dolor sit amet, "}
                            <a href="#">{"consectetur"}</a>
                            {" adipiscing elit lorem ipsum dolor sit amet, consectetur adipiscing elit"}
                        </Notification>
                    </Column>
                    <Column classes="is-half">
                        <Notification classes="is-success">
                            <YDelete />
                            {"Lorem ipsum dolor sit amet, "}
                            <a href="#">{"consectetur"}</a>
                            {" adipiscing elit lorem ipsum dolor sit amet, consectetur adipiscing elit"}
                        </Notification>
                    </Column>
                    <Column classes="is-half">
                        <Notification classes="is-warning">
                            <YDelete />
                            {"Lorem ipsum dolor sit amet, "}
                            <a href="#">{"consectetur"}</a>
                            {" adipiscing elit lorem ipsum dolor sit amet, consectetur adipiscing elit"}
                        </Notification>
                    </Column>
                    <Column classes="is-half">
                        <Notification classes="is-danger">
                            <YDelete />
                            {"Lorem ipsum dolor sit amet, "}
                            <a href="#">{"consectetur"}</a>
                            {" adipiscing elit lorem ipsum dolor sit amet, consectetur adipiscing elit"}
                        </Notification>
                    </Column>
                </Columns>
            </Section>
        </>
    }
}

#[function_component(ProgressSection)]
fn progress_section() -> Html {
    html! {
        <>
            <div id="progress"></div>
            <Section>
                <Title tag="h1" size={HeaderSize::Is1}>{"Progress"}</Title>
                <hr />
                <Progress classes="is-small" max={100.0} value={Some(14.3)} />
                <Progress classes="is-small is-primary" max={100.0} value={Some(28.6)} />
                <Progress classes="is-link" max={100.0} value={Some(42.9)} />
                <Progress classes="is-info" max={100.0} value={Some(57.1)} />
                <Progress classes="is-medium is-success" max={100.0} value={Some(71.4)} />
                <Progress classes="is-medium is-warning" max={100.0} value={Some(85.7)} />
                <Progress classes="is-large is-danger" max={100.0} value={Some(100.0)} />

                <Title tag="h2" size={HeaderSize::Is2}>{"Indeterminate"}</Title>
                <Progress classes="is-small is-primary" max={100.0} />
                <Progress classes="is-danger" max={100.0} />
                <Progress classes="is-medium is-dark" max={100.0} />
                <Progress classes="is-large is-info" max={100.0} />
            </Section>
        </>
    }
}

#[function_component(TableSection)]
fn table_section() -> Html {
    html! {
        <>
            <div id="table"></div>
            <Section>
                <Title tag="h1" size={HeaderSize::Is1}>{"Table"}</Title>
                <hr />
                <Table>
                    <thead>
                        <tr>
                            <th><abbr title="Position">{"Pos"}</abbr></th>
                            <th>{"Team"}</th>
                            <th><abbr title="Played">{"Pld"}</abbr></th>
                            <th><abbr title="Won">{"W"}</abbr></th>
                            <th><abbr title="Drawn">{"D"}</abbr></th>
                            <th><abbr title="Lost">{"L"}</abbr></th>
                            <th><abbr title="Goals for">{"GF"}</abbr></th>
                            <th><abbr title="Goals against">{"GA"}</abbr></th>
                            <th><abbr title="Goal difference">{"GD"}</abbr></th>
                            <th><abbr title="Points">{"Pts"}</abbr></th>
                            <th>{"Qualification or relegation"}</th>
                        </tr>
                    </thead>
                    <tfoot>
                        <tr>
                            <th><abbr title="Position">{"Pos"}</abbr></th>
                            <th>{"Team"}</th>
                            <th><abbr title="Played">{"Pld"}</abbr></th>
                            <th><abbr title="Won">{"W"}</abbr></th>
                            <th><abbr title="Drawn">{"D"}</abbr></th>
                            <th><abbr title="Lost">{"L"}</abbr></th>
                            <th><abbr title="Goals for">{"GF"}</abbr></th>
                            <th><abbr title="Goals against">{"GA"}</abbr></th>
                            <th><abbr title="Goal difference">{"GD"}</abbr></th>
                            <th><abbr title="Points">{"Pts"}</abbr></th>
                            <th>{"Qualification or relegation"}</th>
                        </tr>
                    </tfoot>
                    <tbody>
                        <tr>
                            <th>{"1"}</th>
                            <td>{"Leicester City "}<strong>{"(C)"}</strong></td>
                            <td>{"38"}</td><td>{"23"}</td><td>{"12"}</td><td>{"3"}</td>
                            <td>{"68"}</td><td>{"36"}</td><td>{"+32"}</td><td>{"81"}</td>
                            <td>{"Qualification for the Champions League group stage"}</td>
                        </tr>
                        <tr>
                            <th>{"2"}</th>
                            <td>{"Arsenal"}</td>
                            <td>{"38"}</td><td>{"20"}</td><td>{"11"}</td><td>{"7"}</td>
                            <td>{"65"}</td><td>{"36"}</td><td>{"+29"}</td><td>{"71"}</td>
                            <td>{"Qualification for the Champions League group stage"}</td>
                        </tr>
                        <tr>
                            <th>{"3"}</th>
                            <td>{"Tottenham Hotspur"}</td>
                            <td>{"38"}</td><td>{"19"}</td><td>{"13"}</td><td>{"6"}</td>
                            <td>{"69"}</td><td>{"35"}</td><td>{"+34"}</td><td>{"70"}</td>
                            <td>{"Qualification for the Champions League group stage"}</td>
                        </tr>
                        <tr>
                            <th>{"4"}</th>
                            <td>{"Manchester City"}</td>
                            <td>{"38"}</td><td>{"19"}</td><td>{"9"}</td><td>{"10"}</td>
                            <td>{"71"}</td><td>{"41"}</td><td>{"+30"}</td><td>{"66"}</td>
                            <td>{"Qualification for the Champions League play-off round"}</td>
                        </tr>
                        <tr class="is-selected">
                            <th>{"5"}</th>
                            <td>{"Manchester United"}</td>
                            <td>{"38"}</td><td>{"19"}</td><td>{"9"}</td><td>{"10"}</td>
                            <td>{"49"}</td><td>{"35"}</td><td>{"+14"}</td><td>{"66"}</td>
                            <td>{"Qualification for the Europa League group stage"}</td>
                        </tr>
                        <tr>
                            <th>{"6"}</th>
                            <td>{"Southampton"}</td>
                            <td>{"38"}</td><td>{"18"}</td><td>{"9"}</td><td>{"11"}</td>
                            <td>{"59"}</td><td>{"41"}</td><td>{"+18"}</td><td>{"63"}</td>
                            <td>{"Qualification for the Europa League group stage"}</td>
                        </tr>
                        <tr>
                            <th>{"7"}</th>
                            <td>{"West Ham United"}</td>
                            <td>{"38"}</td><td>{"16"}</td><td>{"14"}</td><td>{"8"}</td>
                            <td>{"65"}</td><td>{"51"}</td><td>{"+14"}</td><td>{"62"}</td>
                            <td>{"Qualification for the Europa League third qualifying round"}</td>
                        </tr>
                        <tr>
                            <th>{"8"}</th>
                            <td>{"Liverpool"}</td>
                            <td>{"38"}</td><td>{"16"}</td><td>{"12"}</td><td>{"10"}</td>
                            <td>{"63"}</td><td>{"50"}</td><td>{"+13"}</td><td>{"60"}</td>
                            <td></td>
                        </tr>
                        <tr>
                            <th>{"9"}</th>
                            <td>{"Stoke City"}</td>
                            <td>{"38"}</td><td>{"14"}</td><td>{"9"}</td><td>{"15"}</td>
                            <td>{"41"}</td><td>{"55"}</td><td>{"−14"}</td><td>{"51"}</td>
                            <td></td>
                        </tr>
                        <tr>
                            <th>{"10"}</th>
                            <td>{"Chelsea"}</td>
                            <td>{"38"}</td><td>{"12"}</td><td>{"14"}</td><td>{"12"}</td>
                            <td>{"59"}</td><td>{"53"}</td><td>{"+6"}</td><td>{"50"}</td>
                            <td></td>
                        </tr>
                        <tr>
                            <th>{"11"}</th>
                            <td>{"Everton"}</td>
                            <td>{"38"}</td><td>{"11"}</td><td>{"14"}</td><td>{"13"}</td>
                            <td>{"59"}</td><td>{"55"}</td><td>{"+4"}</td><td>{"47"}</td>
                            <td></td>
                        </tr>
                        <tr>
                            <th>{"12"}</th>
                            <td>{"Swansea City"}</td>
                            <td>{"38"}</td><td>{"12"}</td><td>{"11"}</td><td>{"15"}</td>
                            <td>{"42"}</td><td>{"52"}</td><td>{"−10"}</td><td>{"47"}</td>
                            <td></td>
                        </tr>
                        <tr>
                            <th>{"13"}</th>
                            <td>{"Watford"}</td>
                            <td>{"38"}</td><td>{"12"}</td><td>{"9"}</td><td>{"17"}</td>
                            <td>{"40"}</td><td>{"50"}</td><td>{"−10"}</td><td>{"45"}</td>
                            <td></td>
                        </tr>
                        <tr>
                            <th>{"14"}</th>
                            <td>{"West Bromwich Albion"}</td>
                            <td>{"38"}</td><td>{"10"}</td><td>{"13"}</td><td>{"15"}</td>
                            <td>{"34"}</td><td>{"48"}</td><td>{"−14"}</td><td>{"43"}</td>
                            <td></td>
                        </tr>
                        <tr>
                            <th>{"15"}</th>
                            <td>{"Crystal Palace"}</td>
                            <td>{"38"}</td><td>{"11"}</td><td>{"9"}</td><td>{"18"}</td>
                            <td>{"39"}</td><td>{"51"}</td><td>{"−12"}</td><td>{"42"}</td>
                            <td></td>
                        </tr>
                        <tr>
                            <th>{"16"}</th>
                            <td>{"AFC Bournemouth"}</td>
                            <td>{"38"}</td><td>{"11"}</td><td>{"9"}</td><td>{"18"}</td>
                            <td>{"45"}</td><td>{"67"}</td><td>{"−22"}</td><td>{"42"}</td>
                            <td></td>
                        </tr>
                        <tr>
                            <th>{"17"}</th>
                            <td>{"Sunderland"}</td>
                            <td>{"38"}</td><td>{"9"}</td><td>{"12"}</td><td>{"17"}</td>
                            <td>{"48"}</td><td>{"62"}</td><td>{"−14"}</td><td>{"39"}</td>
                            <td></td>
                        </tr>
                        <tr>
                            <th>{"18"}</th>
                            <td>{"Newcastle United "}<strong>{"(R)"}</strong></td>
                            <td>{"38"}</td><td>{"9"}</td><td>{"10"}</td><td>{"19"}</td>
                            <td>{"44"}</td><td>{"65"}</td><td>{"−21"}</td><td>{"37"}</td>
                            <td>{"Relegation to the Football League Championship"}</td>
                        </tr>
                        <tr>
                            <th>{"19"}</th>
                            <td>{"Norwich City "}<strong>{"(R)"}</strong></td>
                            <td>{"38"}</td><td>{"9"}</td><td>{"7"}</td><td>{"22"}</td>
                            <td>{"39"}</td><td>{"67"}</td><td>{"−28"}</td><td>{"34"}</td>
                            <td>{"Relegation to the Football League Championship"}</td>
                        </tr>
                        <tr>
                            <th>{"20"}</th>
                            <td>{"Aston Villa "}<strong>{"(R)"}</strong></td>
                            <td>{"38"}</td><td>{"3"}</td><td>{"8"}</td><td>{"27"}</td>
                            <td>{"27"}</td><td>{"76"}</td><td>{"−49"}</td><td>{"17"}</td>
                            <td>{"Relegation to the Football League Championship"}</td>
                        </tr>
                    </tbody>
                </Table>
                <br />
                <Columns>
                <Column>
                <Table striped=true>
                    <thead><tr><th>{"One"}</th><th>{"Two"}</th></tr></thead>
                    <tbody>
                        <tr><td>{"Three"}</td><td>{"Four"}</td></tr>
                        <tr><td>{"Five"}</td><td>{"Six"}</td></tr>
                        <tr><td>{"Seven"}</td><td>{"Eight"}</td></tr>
                        <tr><td>{"Nine"}</td><td>{"Ten"}</td></tr>
                        <tr><td>{"Eleven"}</td><td>{"Twelve"}</td></tr>
                    </tbody>
                </Table>
                </Column>
                <Column>
                <Table bordered=true>
                    <thead><tr><th>{"One"}</th><th>{"Two"}</th></tr></thead>
                    <tbody><tr><td>{"Three"}</td><td>{"Four"}</td></tr></tbody>
                </Table>
                </Column>
                <Column>
                <Table narrow=true>
                    <thead><tr><th>{"One"}</th><th>{"Two"}</th></tr></thead>
                    <tbody>
                        <tr><td>{"Three"}</td><td>{"Four"}</td></tr>
                        <tr><td>{"Five"}</td><td>{"Six"}</td></tr>
                        <tr><td>{"Seven"}</td><td>{"Eight"}</td></tr>
                        <tr><td>{"Nine"}</td><td>{"Ten"}</td></tr>
                        <tr><td>{"Eleven"}</td><td>{"Twelve"}</td></tr>
                    </tbody>
                </Table>
                </Column>
                <Column>
                <Table bordered=true striped=true narrow=true>
                    <thead><tr><th>{"One"}</th><th>{"Two"}</th></tr></thead>
                    <tbody>
                        <tr><td>{"Three"}</td><td>{"Four"}</td></tr>
                        <tr><td>{"Five"}</td><td>{"Six"}</td></tr>
                        <tr><td>{"Seven"}</td><td>{"Eight"}</td></tr>
                        <tr><td>{"Nine"}</td><td>{"Ten"}</td></tr>
                        <tr><td>{"Eleven"}</td><td>{"Twelve"}</td></tr>
                    </tbody>
                </Table>
                </Column>
                </Columns>
            </Section>
        </>
    }
}

#[function_component(TagSection)]
fn tag_section() -> Html {
    html! {
        <>
            <div id="tag"></div>
            <Section>
                <Title tag="h1" size={HeaderSize::Is1}>{"Tag"}</Title>
                <hr />
                <div class="tags">
                    <Tag classes="is-primary">{"Primary"}</Tag>
                    <Tag classes="is-link">{"Link"}</Tag>
                    <Tag classes="is-info">{"Info"}</Tag>
                    <Tag classes="is-success">{"Success"}</Tag>
                    <Tag classes="is-warning">{"Warning"}</Tag>
                    <Tag classes="is-danger">{"Danger"}</Tag>
                    <Tag classes="is-white">{"White"}</Tag>
                    <Tag classes="is-black">{"Black"}</Tag>
                    <Tag classes="is-light">{"Light"}</Tag>
                    <Tag classes="is-dark">{"Dark"}</Tag>
                    <Tag classes="is-primary is-medium">{"Medium"}</Tag>
                    <Tag classes="is-info is-large">{"Large"}</Tag>
                    <Tag classes="is-success">{"Bar"}<YDelete classes="is-small" /></Tag>
                    <Tag classes="is-warning is-medium">{"Hello"}<YDelete classes="is-small" /></Tag>
                    <Tag classes="is-danger is-large">{"World"}<YDelete /></Tag>
                </div>
                <ybc::Field grouped=true multiline=true>
                    <ybc::Control>
                        <ybc::Tags has_addons=true>
                            <Tag classes="is-dark">{"npm"}</Tag>
                            <Tag classes="is-info">{"0.5.0"}</Tag>
                        </ybc::Tags>
                    </ybc::Control>
                    <ybc::Control>
                        <ybc::Tags has_addons=true>
                            <Tag classes="is-dark">{"build"}</Tag>
                            <Tag classes="is-success">{"passing"}</Tag>
                        </ybc::Tags>
                    </ybc::Control>
                    <ybc::Control>
                        <ybc::Tags has_addons=true>
                            <Tag classes="is-dark">{"chat"}</Tag>
                            <Tag classes="is-primary">{"on gitter"}</Tag>
                        </ybc::Tags>
                    </ybc::Control>
                </ybc::Field>
                <ybc::Field grouped=true multiline=true>
                    <ybc::Control>
                        <ybc::Tags has_addons=true>
                            <Tag tag={"a".to_string()} classes="is-link">{"Technology"}</Tag>
                            <Tag tag={"a".to_string()} delete=true classes="is-delete"></Tag>
                        </ybc::Tags>
                    </ybc::Control>
                    <ybc::Control>
                        <ybc::Tags has_addons=true>
                            <Tag tag={"a".to_string()} classes="is-link">{"CSS"}</Tag>
                            <Tag tag={"a".to_string()} delete=true classes="is-delete"></Tag>
                        </ybc::Tags>
                    </ybc::Control>
                    <ybc::Control>
                        <ybc::Tags has_addons=true>
                            <Tag tag={"a".to_string()} classes="is-link">{"Flexbox"}</Tag>
                            <Tag tag={"a".to_string()} delete=true classes="is-delete"></Tag>
                        </ybc::Tags>
                    </ybc::Control>
                    <ybc::Control>
                        <ybc::Tags has_addons=true>
                            <Tag tag={"a".to_string()} classes="is-link">{"Web Design"}</Tag>
                            <Tag tag={"a".to_string()} delete=true classes="is-delete"></Tag>
                        </ybc::Tags>
                    </ybc::Control>
                    <ybc::Control>
                        <ybc::Tags has_addons=true>
                            <Tag tag={"a".to_string()} classes="is-link">{"Open Source"}</Tag>
                            <Tag tag={"a".to_string()} delete=true classes="is-delete"></Tag>
                        </ybc::Tags>
                    </ybc::Control>
                    <ybc::Control>
                        <ybc::Tags has_addons=true>
                            <Tag tag={"a".to_string()} classes="is-link">{"Community"}</Tag>
                            <Tag tag={"a".to_string()} delete=true classes="is-delete"></Tag>
                        </ybc::Tags>
                    </ybc::Control>
                    <ybc::Control>
                        <ybc::Tags has_addons=true>
                            <Tag tag={"a".to_string()} classes="is-link">{"Documentation"}</Tag>
                            <Tag tag={"a".to_string()} delete=true classes="is-delete"></Tag>
                        </ybc::Tags>
                    </ybc::Control>
                </ybc::Field>
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
                                <img alt="Image" src="https://placehold.net/avatar-2.svg" />
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

#[function_component(ContentSection)]
fn content_section() -> Html {
    html! {
        <>
            <div id="content"></div>
            <Section>
                <Title tag="h1" size={HeaderSize::Is1}>{"Content"}</Title>
                <hr />
                <Content>
                    <h1>{"Hello World"}</h1>
                    <p>
                        {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla accumsan, metus ultrices eleifend gravida, nulla nunc varius lectus, nec rutrum justo nibh eu lectus. Ut vulputate semper dui. Fusce erat odio, sollicitudin vel erat vel, interdum mattis neque."}
                    </p>
                    <h2>{"Second level"}</h2>
                    <p>
                        {"Curabitur accumsan turpis pharetra "}
                        <strong>{"augue tincidunt"}</strong>
                        {" blandit. Quisque condimentum maximus mi, sit amet commodo arcu rutrum id. Proin pretium urna vel cursus venenatis. Suspendisse potenti. Etiam mattis sem rhoncus lacus dapibus facilisis. Donec at dignissim dui. Ut et neque nisl."}
                    </p>
                    <ul>
                        <li>{"In fermentum leo eu lectus mollis, quis dictum mi aliquet."}</li>
                        <li>{"Morbi eu nulla lobortis, lobortis est in, fringilla felis."}</li>
                        <li>{"Aliquam nec felis in sapien venenatis viverra fermentum nec lectus."}</li>
                        <li>{"Ut non enim metus."}</li>
                    </ul>
                    <h3>{"Third level"}</h3>
                    <p>
                        {"Quisque ante lacus, malesuada ac auctor vitae, congue "}
                        <a href="#">{"non ante"}</a>
                        {". Phasellus lacus ex, semper ac tortor nec, fringilla condimentum orci. Fusce eu rutrum tellus."}
                    </p>
                    <ol>
                        <li>{"Donec blandit a lorem id convallis."}</li>
                        <li>{"Cras gravida arcu at diam gravida gravida."}</li>
                        <li>{"Integer in volutpat libero."}</li>
                        <li>{"Donec a diam tellus."}</li>
                        <li>{"Aenean nec tortor orci."}</li>
                        <li>{"Quisque aliquam cursus urna, non bibendum massa viverra eget."}</li>
                        <li>{"Vivamus maximus ultricies pulvinar."}</li>
                    </ol>
                    <blockquote>
                        {"Ut venenatis, nisl scelerisque sollicitudin fermentum, quam libero hendrerit ipsum, ut blandit est tellus sit amet turpis."}
                    </blockquote>
                    <p>
                        {"Quisque at semper enim, eu hendrerit odio. Etiam auctor nisl et "}
                        <em>{"justo sodales"}</em>
                        {" elementum. Maecenas ultrices lacus quis neque consectetur, et lobortis nisi molestie."}
                    </p>
                    <p>{"Sed sagittis enim ac tortor maximus rutrum. Nulla facilisi. Donec mattis vulputate risus in luctus. Maecenas vestibulum interdum commodo."}</p>
                    <p>{"Suspendisse egestas sapien non felis placerat elementum. Morbi tortor nisl, suscipit sed mi sit amet, mollis malesuada nulla. Nulla facilisi. Nullam ac erat ante."}</p>
                    <h4>{"Fourth level"}</h4>
                    <p>{"Nulla efficitur eleifend nisi, sit amet bibendum sapien fringilla ac. Mauris euismod metus a tellus laoreet, at elementum ex efficitur."}</p>
                    <p>{"Maecenas eleifend sollicitudin dui faucibus sollicitudin augue cursus non. Ut finibus eleifend arcu ut vehicula. Mauris eu est maximus est porta condimentum in eu justo. Nulla id iaculis sapien."}</p>
                    <Table fullwidth={true}>
                        <thead>
                            <tr>
                                <th>{"One"}</th>
                                <th>{"Two"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>{"Three"}</td>
                                <td>{"Four"}</td>
                            </tr>
                            <tr>
                                <td>{"Five"}</td>
                                <td>{"Six"}</td>
                            </tr>
                            <tr>
                                <td>{"Seven"}</td>
                                <td>{"Eight"}</td>
                            </tr>
                            <tr>
                                <td>{"Nine"}</td>
                                <td>{"Ten"}</td>
                            </tr>
                            <tr>
                                <td>{"Eleven"}</td>
                                <td>{"Twelve"}</td>
                            </tr>
                        </tbody>
                    </Table>
                    <p>{"Phasellus porttitor enim id metus volutpat ultricies. Ut nisi nunc, blandit sed dapibus at, vestibulum in felis. Etiam iaculis lorem ac nibh bibendum rhoncus. Nam interdum efficitur ligula sit amet ullamcorper. Etiam tristique, leo vitae porta faucibus, mi lacus laoreet metus, at cursus leo est vel tellus. Sed ac posuere est. Nunc ultricies nunc neque, vitae ultricies ex sodales quis. Aliquam eu nibh in libero accumsan pulvinar. Nullam nec nisl placerat, pretium metus vel, euismod ipsum. Proin tempor cursus nisl vel condimentum. Nam pharetra varius metus non pellentesque."}</p>
                    <h5>{"Fifth level"}</h5>
                    <p>{"Aliquam sagittis rhoncus vulputate. Cras non luctus sem, sed tincidunt ligula. Vestibulum at nunc elit. Praesent aliquet ligula mi, in luctus elit volutpat porta. Phasellus molestie diam vel nisi sodales, a eleifend augue laoreet. Sed nec eleifend justo. Nam et sollicitudin odio."}</p>
                    <h6>{"Sixth level"}</h6>
                    <p>{"Cras in nibh lacinia, venenatis nisi et, auctor urna. Donec pulvinar lacus sed diam dignissim, ut eleifend eros accumsan. Phasellus non tortor eros. Ut sed rutrum lacus. Etiam purus nunc, scelerisque quis enim vitae, malesuada ultrices turpis. Nunc vitae maximus purus, nec consectetur dui. Suspendisse euismod, elit vel rutrum commodo, ipsum tortor maximus dui, sed varius sapien odio vitae est. Etiam at cursus metus."}</p>
                </Content>
            </Section>
        </>
    }
}

#[function_component(DeleteSection)]
fn delete_section() -> Html {
    html! {
        <>
            <div id="delete"></div>
            <Section>
                <Title tag="h1" size={HeaderSize::Is1}>{"Delete"}</Title>
                <hr />
                <Block>
                    <Tag classes="is-success">
                        {"Hello World"}
                        <YDelete classes="is-small" />
                    </Tag>
                    {" "}
                    <YDelete tag={"a".to_string()} classes="is-small" />
                    {" "}
                    <YDelete tag={"a".to_string()} />
                    {" "}
                    <YDelete tag={"a".to_string()} classes="is-medium" />
                    {" "}
                    <YDelete tag={"a".to_string()} classes="is-large" />
                </Block>
                <Notification classes="is-danger">
                    <YDelete />
                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit lorem ipsum dolor sit amet, consectetur adipiscing elit"}
                </Notification>
                <Message classes="is-info">
                    <MessageHeader>
                        {"Info"}
                        <YDelete />
                    </MessageHeader>
                    <MessageBody>
                        {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque risus mi, tempus quis placerat ut, porta nec nulla. Vestibulum rhoncus ac ex sit amet fringilla. Nullam gravida purus diam, et dictum felis venenatis efficitur. Aenean ac eleifend lacus, in mollis lectus. Donec sodales, arcu et sollicitudin porttitor, tortor urna tempor ligula, id porttitor mi magna a neque. Donec dui urna, vehicula et sem eget, facilisis sodales sem."}
                    </MessageBody>
                </Message>
            </Section>
        </>
    }
}

#[function_component(FormSection)]
fn form_section() -> Html {
    // Local state only for file inputs so filenames update when selecting files
    let file1 = use_state(Vec::new);
    let file2 = use_state(Vec::new);
    let file3 = use_state(Vec::new);
    let file4 = use_state(Vec::new);
    let file5 = use_state(Vec::new);
    let file6 = use_state(Vec::new);
    html! {
        <>
            <div id="form"></div>
            <Section>
                <Title tag="h1" size={HeaderSize::Is1}>{"Form"}</Title>
                <hr />
                <Columns>
                    <Column>
            <ybc::Field label={Some("Name".to_string())}>
                            <ybc::Control>
                <ybc::Input name="name" value="" update={Callback::from(|_: String| {})} placeholder="Text input" />
                            </ybc::Control>
                        </ybc::Field>

                        <ybc::Field label={Some(String::from("Username"))} help={Some(String::from("This username is available"))} help_classes={classes!("is-success")}>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="username" value="bulma" update={Callback::from(|_: String| {})} classes="is-success" placeholder="Text input" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-user"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>

                        <ybc::Field label={Some(String::from("Email"))} help={Some(String::from("This email is invalid"))} help_classes={classes!("is-danger")}>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="email" value="hello@" update={Callback::from(|_: String| {})} classes="is-danger" placeholder="Email input" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-warning"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>

                        <ybc::Field label={Some(String::from("Subject"))}>
                            <ybc::Control>
                                <ybc::Select name="subject" value="" update={Callback::from(|_: String| {})}>
                                    <option>{"Select dropdown"}</option>
                                    <option>{"With options"}</option>
                                </ybc::Select>
                            </ybc::Control>
                        </ybc::Field>

                        <ybc::Field label={Some(String::from("Files to join"))}>
                            <ybc::Control>
                                <ybc::MultiSelect name="files_to_join" value={vec![]} update={Callback::from(|_: Vec<String>| {})}>
                                    <option>{"Select dropdown"}</option>
                                    <option>{"With options"}</option>
                                </ybc::MultiSelect>
                            </ybc::Control>
                        </ybc::Field>

                        <ybc::Field label={Some(String::from("Message"))}>
                            <ybc::Control>
                                <ybc::TextArea name="message" value="" update={Callback::from(|_: String| {})} placeholder="Textarea" />
                            </ybc::Control>
                        </ybc::Field>

                        <ybc::Field>
                            <ybc::Control>
                                <ybc::Checkbox name="agree" checked={false} update={Callback::from(|_: bool| {})}>
                                    {" I agree to the "}
                                    <a href="#">{"terms and conditions"}</a>
                                </ybc::Checkbox>
                            </ybc::Control>
                        </ybc::Field>

                        <ybc::Field>
                            <ybc::Control>
                                <ybc::Radio name="question" value="Yes" checked_value={None::<String>} update={Callback::from(|_: String| {})}>{" Yes"}</ybc::Radio>
                                {" "}
                                <ybc::Radio name="question" value="No" checked_value={None::<String>} update={Callback::from(|_: String| {})}>{" No"}</ybc::Radio>
                            </ybc::Control>
                        </ybc::Field>

                        <ybc::Field grouped=true>
                            <ybc::Control>
                                <Button classes="is-primary">{"Submit"}</Button>
                            </ybc::Control>
                            <ybc::Control>
                                <Button classes="is-link">{"Cancel"}</Button>
                            </ybc::Control>
                        </ybc::Field>

                        <br />
                        <Subtitle tag="h4">{"Disabled"}</Subtitle>
                        <hr />

                        <ybc::Field>
                            <ybc::Control>
                                <ybc::Input name="disabled_input" value="" update={Callback::from(|_| {})} placeholder="Disabled input" disabled=true />
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control>
                                <ybc::TextArea name="disabled_textarea" value="" update={Callback::from(|_| {})} placeholder="Disabled textarea" disabled=true />
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control>
                                <ybc::Checkbox name="remember" checked={false} update={Callback::from(|_| {})} classes="is-disabled" disabled=true>
                                    {" Remember me"}
                                </ybc::Checkbox>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control>
                                <ybc::Radio name="question_disabled" value="Yes" checked_value={None::<String>} update={Callback::from(|_| {})} classes="is-disabled" disabled=true>
                                    {" Yes"}
                                </ybc::Radio>
                                {" "}
                                <ybc::Radio name="question_disabled" value="No" checked_value={None::<String>} update={Callback::from(|_| {})} classes="is-disabled" disabled=true>
                                    {" No"}
                                </ybc::Radio>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field grouped=true>
                            <ybc::Control>
                                <Button classes="is-primary" disabled=true>{"Submit"}</Button>
                            </ybc::Control>
                            <ybc::Control>
                                <Button disabled=true>{"Cancel"}</Button>
                            </ybc::Control>
                        </ybc::Field>

                        <br />
                        <Title tag="h3">{"Horizontal Form"}</Title>
                        <hr />

                        <ybc::Field horizontal=true label={Some(String::from("From"))} label_classes={classes!("is-normal", "has-text-left")}>
                            <ybc::Field grouped=true>
                                <ybc::Control classes="is-expanded" icons_left=true>
                                    <ybc::Input name="from_name" value="" update={Callback::from(|_| {})} placeholder="Name" />
                                    <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Small)}>
                                        <i class="fa fa-user"></i>
                                    </Icon>
                                </ybc::Control>
                            </ybc::Field>
                            <ybc::Field help={Some(String::from("This email is correct"))} help_classes={classes!("is-success")}>
                                <ybc::Control classes="is-expanded" icons_left=true icons_right=true>
                                    <ybc::Input name="from_email" value="alex@smith.com" update={Callback::from(|_: String| {})} classes="is-success" placeholder="Email" />
                                    <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Small)}>
                                        <i class="fa fa-envelope"></i>
                                    </Icon>
                                    <Icon alignment={Some(ybc::Alignment::Right)} size={Some(ybc::Size::Small)}>
                                        <i class="fa fa-check"></i>
                                    </Icon>
                                </ybc::Control>
                            </ybc::Field>
                        </ybc::Field>

                        <ybc::Field horizontal=true label={Some(String::from("Department"))} label_classes={classes!("is-normal", "has-text-left")}>
                            <ybc::Field classes="is-narrow">
                                <ybc::Control>
                                    <ybc::Select name="department" value="Business development" update={Callback::from(|_: String| {})} classes="is-fullwidth">
                                        <option>{"Business development"}</option>
                                        <option>{"Marketing"}</option>
                                        <option>{"Sales"}</option>
                                    </ybc::Select>
                                </ybc::Control>
                            </ybc::Field>
                        </ybc::Field>

                        <ybc::Field horizontal=true label={Some(String::from("Skills"))} label_classes={classes!("is-normal", "has-text-left")}>
                            <ybc::Control>
                                <ybc::MultiSelect name="skills" value={vec!["Science computer".to_string()]} update={Callback::from(|_: Vec<String>| {})}>
                                    <option>{"Science computer"}</option>
                                    <option>{"Development"}</option>
                                    <option>{"Management"}</option>
                                    <option>{"Relationship"}</option>
                                </ybc::MultiSelect>
                            </ybc::Control>
                        </ybc::Field>

                        <ybc::Field horizontal=true label={Some(String::from("Already a member?"))} label_classes={classes!("is-normal", "has-text-left")}>
                            <ybc::Field classes="is-narrow">
                                <ybc::Control>
                                    <ybc::Radio name="member" value="Yes" checked_value={None::<String>} update={Callback::from(|_| {})}>{" Yes"}</ybc::Radio>
                                    {" "}
                                    <ybc::Radio name="member" value="No" checked_value={None::<String>} update={Callback::from(|_| {})}>{" No"}</ybc::Radio>
                                </ybc::Control>
                            </ybc::Field>
                        </ybc::Field>

                        <ybc::Field horizontal=true label={Some(String::from("Subject"))} label_classes={classes!("is-normal", "has-text-left")}>
                            <ybc::Field help={Some(String::from("This field is required"))} help_classes={classes!("is-danger")}>
                                <ybc::Control>
                                    <ybc::Input name="subject_error" value="" update={Callback::from(|_: String| {})} classes="is-danger" placeholder="e.g. Partnership opportunity" />
                                </ybc::Control>
                            </ybc::Field>
                        </ybc::Field>

                        <ybc::Field horizontal=true label={Some(String::from("Question"))} label_classes={classes!("is-normal", "has-text-left")}>
                            <ybc::Field>
                                <ybc::Control>
                                    <ybc::TextArea name="question" value="" update={Callback::from(|_: String| {})} placeholder="Explain how we can help you" />
                                </ybc::Control>
                            </ybc::Field>
                        </ybc::Field>

                        <ybc::Field horizontal=true label={None::<String>} label_classes={classes!("is-normal", "has-text-left")}>
                            <ybc::Field>
                                <ybc::Control>
                                    <Button classes="is-primary">{"Send message"}</Button>
                                </ybc::Control>
                            </ybc::Field>
                        </ybc::Field>

                        <br />
                        <Title tag="h3">{"File"}</Title>
                        <hr />
                        <ybc::Field>
                            <ybc::File name="resume1" files={(*file1).clone()} update={{ let s=file1.clone(); Callback::from(move |v| s.set(v)) }} selector_icon={html!{<i class="fa fa-upload"></i>}} />
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::File name="resume2" files={(*file2).clone()} update={{ let s=file2.clone(); Callback::from(move |v| s.set(v)) }} has_name={Some("Screen Shot 2017-07-29 at 15.54.25.png".to_string())} selector_icon={html!{<i class="fa fa-upload"></i>}} />
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::File name="resume3" files={(*file3).clone()} update={{ let s=file3.clone(); Callback::from(move |v| s.set(v)) }} classes="is-primary" selector_icon={html!{<i class="fa fa-upload"></i>}} selector_label="Primary file…" />
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::File name="resume4" files={(*file4).clone()} update={{ let s=file4.clone(); Callback::from(move |v| s.set(v)) }} classes="is-info" has_name={Some("Screen Shot 2017-07-29 at 15.54.25.png".to_string())} selector_icon={html!{<i class="fa fa-upload"></i>}} selector_label="Info file…" />
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::File name="resume5" files={(*file5).clone()} update={{ let s=file5.clone(); Callback::from(move |v| s.set(v)) }} classes="is-warning is-boxed" selector_icon={html!{<i class="fa fa-cloud-upload-alt"></i>}} selector_label="Warning file…" />
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::File name="resume6" files={(*file6).clone()} update={{ let s=file6.clone(); Callback::from(move |v| s.set(v)) }} classes="is-danger has-name is-boxed" selector_icon={html!{<i class="fa fa-cloud-upload-alt"></i>}} selector_label="Danger file…" has_name={Some("Screen Shot 2017-07-29 at 15.54.25.png".to_string())} />
                        </ybc::Field>
                    </Column>
                    <Column>
                        <br />
                        <Subtitle tag="h3">{"Styles"}</Subtitle>
                        <hr />
                        <ybc::Field>
                            <ybc::Control>
                                <ybc::Input name="rounded" value="" update={Callback::from(|_| {})} placeholder="Rounded input" rounded=true />
                            </ybc::Control>
                        </ybc::Field>
                        <br />
                        <Subtitle tag="h3">{"Colors"}</Subtitle>
                        <hr />
                        <ybc::Field><ybc::Control><ybc::Input name="c1" value="" update={Callback::from(|_: String| {})} classes="is-primary" placeholder="Primary input" /></ybc::Control></ybc::Field>
                        <ybc::Field><ybc::Control><ybc::Input name="c2" value="" update={Callback::from(|_: String| {})} classes="is-info" placeholder="Info input" /></ybc::Control></ybc::Field>
                        <ybc::Field><ybc::Control><ybc::Input name="c3" value="" update={Callback::from(|_: String| {})} classes="is-success" placeholder="Success input" /></ybc::Control></ybc::Field>
                        <ybc::Field><ybc::Control><ybc::Input name="c4" value="" update={Callback::from(|_: String| {})} classes="is-warning" placeholder="Warning input" /></ybc::Control></ybc::Field>
                        <ybc::Field><ybc::Control><ybc::Input name="c5" value="" update={Callback::from(|_: String| {})} classes="is-danger" placeholder="Danger input" /></ybc::Control></ybc::Field>
                        <br />
                        <Subtitle tag="h3">{"Sizes"}</Subtitle>
                        <hr />
                        <ybc::Field><ybc::Control><ybc::Input name="s1" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Small)} placeholder="Small input" /></ybc::Control></ybc::Field>
                        <ybc::Field><ybc::Control><ybc::Input name="s2" value="" update={Callback::from(|_| {})} placeholder="Normal input" /></ybc::Control></ybc::Field>
                        <ybc::Field><ybc::Control><ybc::Input name="s3" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Medium)} placeholder="Medium input" /></ybc::Control></ybc::Field>
                        <ybc::Field><ybc::Control><ybc::Input name="s4" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Large)} placeholder="Large input" /></ybc::Control></ybc::Field>

                        <ybc::Field><ybc::Control><ybc::Select name="ss1" value="" update={Callback::from(|_| {})} classes="is-small"><option>{"Select dropdown"}</option><option>{"With options"}</option></ybc::Select></ybc::Control></ybc::Field>
                        <ybc::Field><ybc::Control><ybc::Select name="ss2" value="" update={Callback::from(|_| {})}><option>{"Select dropdown"}</option><option>{"With options"}</option></ybc::Select></ybc::Control></ybc::Field>
                        <ybc::Field><ybc::Control><ybc::Select name="ss3" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Medium)}><option>{"Select dropdown"}</option><option>{"With options"}</option></ybc::Select></ybc::Control></ybc::Field>
                        <ybc::Field><ybc::Control><ybc::Select name="ss4" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Large)}><option>{"Select dropdown"}</option><option>{"With options"}</option></ybc::Select></ybc::Control></ybc::Field>

                        <ybc::Field label={Some("Small input".to_string())} label_classes={classes!("is-small")}>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="se1" value="" update={Callback::from(|_: String| {})} size={Some(ybc::Size::Small)} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field label={Some("Normal input".to_string())}>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="ne1" value="" update={Callback::from(|_: String| {})} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="ne2" value="" update={Callback::from(|_| {})} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field label={Some("Medium input".to_string())} label_classes={classes!("is-medium")}>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="me1" value="" update={Callback::from(|_: String| {})} size={Some(ybc::Size::Medium)} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="me2" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Medium)} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="me3" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Medium)} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Medium)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)} size={Some(ybc::Size::Medium)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field label={Some("Large input".to_string())} label_classes={classes!("is-large")}>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="le1" value="" update={Callback::from(|_: String| {})} size={Some(ybc::Size::Large)} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="le2" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Large)} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="le3" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Large)} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Medium)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)} size={Some(ybc::Size::Medium)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control icons_left=true icons_right=true>
                                <ybc::Input name="le4" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Large)} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Large)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                                <Icon alignment={Some(ybc::Alignment::Right)} size={Some(ybc::Size::Large)}>
                                    <i class="fa fa-check"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>

                        <br />
                        <Subtitle tag="h4">{"With Font Awesome icons"}</Subtitle>
                        <hr />
                        <ybc::Field>
                            <ybc::Control icons_left=true>
                                <ybc::Input name="fa_email" value="" update={Callback::from(|_| {})} r#type={ybc::InputType::Email} placeholder="Email" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-envelope"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control icons_left=true>
                                <ybc::Input name="fa_pass" value="" update={Callback::from(|_| {})} r#type={ybc::InputType::Password} placeholder="Password" />
                                <Icon alignment={Some(ybc::Alignment::Left)} size={Some(ybc::Size::Small)}>
                                    <i class="fa fa-lock"></i>
                                </Icon>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Control>
                                <Button classes="is-success">{"Login"}</Button>
                            </ybc::Control>
                        </ybc::Field>

                        <br />
                        <Title tag="h3">{"Form addons"}</Title>
                        <hr />
                        <ybc::Field addons=true>
                            <ybc::Control>
                                <ybc::Input name="addon1" value="" update={Callback::from(|_| {})} placeholder="Find a repository" />
                            </ybc::Control>
                            <ybc::Control>
                                <Button classes="is-info">{"Search"}</Button>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field addons=true>
                            <ybc::Control>
                                <ybc::Input name="addon2" value="" update={Callback::from(|_| {})} size={Some(ybc::Size::Large)} placeholder="Find a repository" />
                            </ybc::Control>
                            <ybc::Control>
                                <Button classes="is-info is-large">{"Search"}</Button>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field addons=true>
                            <ybc::Control>
                                <ybc::Select name="currency1" value="" update={Callback::from(|_: String| {})}>
                                    <option>{"$"}</option>
                                    <option>{"£"}</option>
                                    <option>{"€"}</option>
                                </ybc::Select>
                            </ybc::Control>
                            <ybc::Control>
                                <ybc::Input name="amount1" value="" update={Callback::from(|_: String| {})} placeholder="Amount of money" />
                            </ybc::Control>
                            <ybc::Control>
                                <Button>{"Transfer"}</Button>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field addons=true>
                            <ybc::Control>
                                <ybc::Select name="currency2" value="" update={Callback::from(|_: String| {})}>
                                    <option>{"$"}</option>
                                    <option>{"£"}</option>
                                    <option>{"€"}</option>
                                </ybc::Select>
                            </ybc::Control>
                            <ybc::Control classes="is-expanded">
                                <ybc::Input name="amount2" value="" update={Callback::from(|_: String| {})} placeholder="Amount of money" />
                            </ybc::Control>
                            <ybc::Control>
                                <Button>{"Transfer"}</Button>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field addons=true>
                            <ybc::Control classes="is-expanded">
                                <ybc::Select name="country" value="Argentina" update={Callback::from(|_: String| {})} classes="is-fullwidth">
                                    <option value="Argentina">{"Argentina"}</option>
                                    <option value="Bolivia">{"Bolivia"}</option>
                                    <option value="Brazil">{"Brazil"}</option>
                                    <option value="Chile">{"Chile"}</option>
                                    <option value="Colombia">{"Colombia"}</option>
                                    <option value="Ecuador">{"Ecuador"}</option>
                                    <option value="Guyana">{"Guyana"}</option>
                                    <option value="Paraguay">{"Paraguay"}</option>
                                    <option value="Peru">{"Peru"}</option>
                                    <option value="Suriname">{"Suriname"}</option>
                                    <option value="Uruguay">{"Uruguay"}</option>
                                    <option value="Venezuela">{"Venezuela"}</option>
                                </ybc::Select>
                            </ybc::Control>
                            <ybc::Control>
                                <Button classes="is-primary">{"Choose"}</Button>
                            </ybc::Control>
                        </ybc::Field>
                        <ybc::Field grouped=true>
                            <ybc::Control classes="is-expanded">
                                <ybc::Input name="repo" value="" update={Callback::from(|_: String| {})} placeholder="Find a repository" />
                            </ybc::Control>
                            <ybc::Control>
                                <Button classes="is-info">{"Search"}</Button>
                            </ybc::Control>
                        </ybc::Field>
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
