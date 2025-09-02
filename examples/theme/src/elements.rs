use ybc::*;
use yew::prelude::*;

use crate::sections::{SectionContent, SectionId};

#[function_component(TypographySection)]
pub fn typography_section() -> Html {
    html! {
        <SectionContent section={SectionId::Typography}>
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
        </SectionContent>
    }
}

#[function_component(BoxSection)]
pub fn box_section() -> Html {
    html! {
        <SectionContent section={SectionId::Box}>
            <BBox>
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
            </BBox>
        </SectionContent>
    }
}

#[function_component(ButtonSection)]
pub fn button_section() -> Html {
    html! {
        <SectionContent section={SectionId::Button}>
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
                        <Button classes="is-primary is-rounded">{"Primary"}</Button>
                        <Button classes="is-link is-rounded">{"Link"}</Button>
                        <Button classes="is-info is-rounded">{"Info"}</Button>
                        <Button classes="is-success is-rounded">{"Success"}</Button>
                        <Button classes="is-danger is-rounded">{"Danger"}</Button>
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
        </SectionContent>
    }
}

#[function_component(ContentSection)]
pub fn content_section() -> Html {
    html! {
        <SectionContent section={SectionId::Content}>
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
        </SectionContent>
    }
}

#[function_component(DeleteSection)]
pub fn delete_section() -> Html {
    html! {
        <SectionContent section={SectionId::Delete}>
            <Block>
                <Tag classes="is-success">
                    {"Hello World"}
                    <Delete classes="is-small" />
                </Tag>
                {" "}
                <Delete tag={"a".to_string()} classes="is-small" />
                {" "}
                <Delete tag={"a".to_string()} />
                {" "}
                <Delete tag={"a".to_string()} classes="is-medium" />
                {" "}
                <Delete tag={"a".to_string()} classes="is-large" />
            </Block>
            <Notification classes="is-danger">
                <Delete />
                {"Lorem ipsum dolor sit amet, consectetur adipiscing elit lorem ipsum dolor sit amet, consectetur adipiscing elit"}
            </Notification>
            <Message classes="is-info">
                <MessageHeader>
                    {"Info"}
                    <Delete />
                </MessageHeader>
                <MessageBody>
                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque risus mi, tempus quis placerat ut, porta nec nulla. Vestibulum rhoncus ac ex sit amet fringilla. Nullam gravida purus diam, et dictum felis venenatis efficitur. Aenean ac eleifend lacus, in mollis lectus. Donec sodales, arcu et sollicitudin porttitor, tortor urna tempor ligula, id porttitor mi magna a neque. Donec dui urna, vehicula et sem eget, facilisis sodales sem."}
                </MessageBody>
            </Message>
        </SectionContent>
    }
}

#[function_component(IconSection)]
pub fn icon_section() -> Html {
    html! {
        <>
            <SectionContent section={SectionId::Icon}>
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
            </SectionContent>
        </>
    }
}

#[function_component(ImagesSection)]
pub fn images_section() -> Html {
    html! {
        <>
            <SectionContent section={SectionId::Image}>
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
            </SectionContent>
        </>
    }
}

#[function_component(NotificationsSection)]
pub fn notifications_section() -> Html {
    html! {
        <>
            <SectionContent section={SectionId::Notification}>
                <Columns classes="is-multiline">
                    <Column classes="is-half">
                        <Notification>
                            <Delete />
                            {"Lorem ipsum dolor sit amet, "}
                            <a href="#">{"consectetur"}</a>
                            {" adipiscing elit lorem ipsum dolor sit amet, consectetur adipiscing elit"}
                        </Notification>
                    </Column>
                    <Column classes="is-half">
                        <Notification classes="is-primary">
                            <Delete />
                            {"Lorem ipsum dolor sit amet, "}
                            <a href="#">{"consectetur"}</a>
                            {" adipiscing elit lorem ipsum dolor sit amet, consectetur adipiscing elit"}
                        </Notification>
                    </Column>
                    <Column classes="is-half">
                        <Notification classes="is-link">
                            <Delete />
                            {"Lorem ipsum dolor sit amet, "}
                            <a href="#">{"consectetur"}</a>
                            {" adipiscing elit lorem ipsum dolor sit amet, consectetur adipiscing elit"}
                        </Notification>
                    </Column>
                    <Column classes="is-half">
                        <Notification classes="is-info">
                            <Delete />
                            {"Lorem ipsum dolor sit amet, "}
                            <a href="#">{"consectetur"}</a>
                            {" adipiscing elit lorem ipsum dolor sit amet, consectetur adipiscing elit"}
                        </Notification>
                    </Column>
                    <Column classes="is-half">
                        <Notification classes="is-success">
                            <Delete />
                            {"Lorem ipsum dolor sit amet, "}
                            <a href="#">{"consectetur"}</a>
                            {" adipiscing elit lorem ipsum dolor sit amet, consectetur adipiscing elit"}
                        </Notification>
                    </Column>
                    <Column classes="is-half">
                        <Notification classes="is-warning">
                            <Delete />
                            {"Lorem ipsum dolor sit amet, "}
                            <a href="#">{"consectetur"}</a>
                            {" adipiscing elit lorem ipsum dolor sit amet, consectetur adipiscing elit"}
                        </Notification>
                    </Column>
                    <Column classes="is-half">
                        <Notification classes="is-danger">
                            <Delete />
                            {"Lorem ipsum dolor sit amet, "}
                            <a href="#">{"consectetur"}</a>
                            {" adipiscing elit lorem ipsum dolor sit amet, consectetur adipiscing elit"}
                        </Notification>
                    </Column>
                </Columns>
            </SectionContent>
        </>
    }
}

#[function_component(ProgressSection)]
pub fn progress_section() -> Html {
    html! {
        <>
            <SectionContent section={SectionId::Progress}>
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
            </SectionContent>
        </>
    }
}

#[function_component(TableSection)]
pub fn table_section() -> Html {
    html! {
        <>
            <SectionContent section={SectionId::Table}>
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
            </SectionContent>
        </>
    }
}

#[function_component(TagSection)]
pub fn tag_section() -> Html {
    html! {
        <>
            <SectionContent section={SectionId::Tag}>
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
                    <Tag classes="is-success">{"Bar"}<Delete classes="is-small" /></Tag>
                    <Tag classes="is-warning is-medium">{"Hello"}<Delete classes="is-small" /></Tag>
                    <Tag classes="is-danger is-large">{"World"}<Delete /></Tag>
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
            </SectionContent>
        </>
    }
}
