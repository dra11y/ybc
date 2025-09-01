use ybc::*;
use yew::prelude::*;

#[function_component(BreadcrumbSection)]
pub fn breadcrumb_section() -> Html {
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

#[function_component(HeroSection)]
pub fn hero_section() -> Html {
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

#[function_component(CardSection)]
pub fn card_section() -> Html {
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
pub fn dropdown_section() -> Html {
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
pub fn media_section() -> Html {
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
                        <Delete />
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
pub fn menu_section() -> Html {
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
pub fn message_section() -> Html {
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
                                    <Delete />
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
pub fn modal_section() -> Html {
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
pub fn navbar_section() -> Html {
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
pub fn pagination_section() -> Html {
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
pub fn panel_section() -> Html {
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
pub fn tabs_section() -> Html {
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
