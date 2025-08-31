use console_error_panic_hook::set_once as set_panic_hook;
use ybc::{
    use_modal, Button, Card, CardContent, Container, Content, HeaderSize, Hero, HeroSize, Modal, ModalCard, ModalContext, ModalProvider, Section,
    Title,
};
use yew::prelude::*;

#[function_component(BasicModalExample)]
pub fn basic_modal_example() -> Html {
    let modal_handle = use_modal();
    let open_modal = modal_handle.open_callback();

    html! {
        <>
        <Button classes="is-primary is-large" onclick={open_modal}>
            {"Open Basic Modal"}
        </Button>

        <Modal
            id={"demo-modal".to_string()}
            handle={modal_handle}
        >
            <Card>
                <CardContent>
                    <Content>
                        <Title size={HeaderSize::Is4}>{"Modal Content"}</Title>
                        <p>{"This is a basic modal using the Modal component. It includes a card for proper spacing and visual hierarchy."}</p>
                        <p>{"You can click the background or the X button to close this modal."}</p>
                    </Content>
                </CardContent>
            </Card>
        </Modal>
        </>
    }
}

#[function_component(ModalCardExample)]
pub fn modal_card_example() -> Html {
    let modal_handle = use_modal();

    let open_modal = modal_handle.open_callback();
    let on_save = {
        let modal_handle = modal_handle.clone();
        Callback::from(move |_| {
            // Here you would typically save data
            modal_handle.close();
        })
    };

    let on_cancel = {
        let modal_handle = modal_handle.clone();
        Callback::from(move |_| {
            modal_handle.close();
        })
    };

    html! {
        <>
        <Button classes="is-info is-large" onclick={open_modal}>
            {"Open Modal Card"}
        </Button>

        <ModalCard
            id={"card-modal".to_string()}
            handle={modal_handle}
            title={"Modal title".to_string()}
            body={html! {
                <Content>
                    <Title size={HeaderSize::Is1}>{"Hello World"}</Title>
                    <p>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla accumsan, metus ultrices eleifend gravida, nulla nunc varius lectus, nec rutrum justo nibh eu lectus. Ut vulputate semper dui. Fusce erat odio, sollicitudin vel erat vel, interdum mattis neque."}</p>

                    <Title size={HeaderSize::Is2}>{"Second level"}</Title>
                    <p>{"Curabitur accumsan turpis pharetra "}<strong>{"augue tincidunt"}</strong>{" blandit. Quisque condimentum maximus mi, sit amet commodo arcu rutrum id. Proin pretium urna vel cursus venenatis. Suspendisse potenti. Etiam mattis sem rhoncus lacus dapibus facilisis. Donec at dignissim dui. Ut et neque nisl."}</p>

                    <ul>
                        <li>{"In fermentum leo eu lectus mollis, quis dictum mi aliquet."}</li>
                        <li>{"Morbi eu nulla lobortis, lobortis est in, fringilla felis."}</li>
                        <li>{"Aliquam nec felis in sapien venenatis viverra fermentum nec lectus."}</li>
                        <li>{"Ut non enim metus."}</li>
                    </ul>

                    <Title size={HeaderSize::Is3}>{"Third level"}</Title>
                    <p>{"Quisque ante lacus, malesuada ac auctor vitae, congue "}<a href="#">{"non ante"}</a>{". Phasellus lacus ex, semper ac tortor nec, fringilla condimentum orci. Fusce eu rutrum tellus."}</p>

                    <ol>
                        <li>{"Donec blandit a lorem id convallis."}</li>
                        <li>{"Cras gravida arcu at diam gravida gravida."}</li>
                        <li>{"Integer in volutpat libero."}</li>
                        <li>{"Donec a diam tellus."}</li>
                        <li>{"Aenean nec tortor orci."}</li>
                        <li>{"Quisque aliquam cursus urna, non bibendum massa viverra eget."}</li>
                        <li>{"Vivamus maximus ultricies pulvinar."}</li>
                    </ol>

                    <blockquote>{"Ut venenatis, nisl scelerisque sollicitudin fermentum, quam libero hendrerit ipsum, ut blandit est tellus sit amet turpis."}</blockquote>

                    <p>{"Quisque at semper enim, eu hendrerit odio. Etiam auctor nisl et "}<em>{"justo sodales"}</em>{" elementum. Maecenas ultrices lacus quis neque consectetur, et lobortis nisi molestie."}</p>

                    <p>{"Sed sagittis enim ac tortor maximus rutrum. Nulla facilisi. Donec mattis vulputate risus in luctus. Maecenas vestibulum interdum commodo."}</p>

                    <p>{"Suspendisse egestas sapien non felis placerat elementum. Morbi tortor nisl, suscipit sed mi sit amet, mollis malesuada nulla. Nulla facilisi. Nullam ac erat ante."}</p>

                    <Title size={HeaderSize::Is4}>{"Fourth level"}</Title>
                    <p>{"Nulla efficitur eleifend nisi, sit amet bibendum sapien fringilla ac. Mauris euismod metus a tellus laoreet, at elementum ex efficitur."}</p>

                    <p>{"Maecenas eleifend sollicitudin dui, faucibus sollicitudin augue cursus non. Ut finibus eleifend arcu ut vehicula. Mauris eu est maximus est porta condimentum in eu justo. Nulla id iaculis sapien."}</p>

                    <p>{"Phasellus porttitor enim id metus volutpat ultricies. Ut nisi nunc, blandit sed dapibus at, vestibulum in felis. Etiam iaculis lorem ac nibh bibendum rhoncus. Nam interdum efficitur ligula sit amet ullamcorper. Etiam tristique, leo vitae porta faucibus, mi lacus laoreet metus, at cursus leo est vel tellus. Sed ac posuere est. Nunc ultricies nunc neque, vitae ultricies ex sodales quis. Aliquam eu nibh in libero accumsan pulvinar. Nullam nec nisl placerat, pretium metus vel, euismod ipsum. Proin tempor cursus nisl vel condimentum. Nam pharetra varius metus non pellentesque."}</p>

                    <Title size={HeaderSize::Is5}>{"Fifth level"}</Title>
                    <p>{"Aliquam sagittis rhoncus vulputate. Cras non luctus sem, sed tincidunt ligula. Vestibulum at nunc elit. Praesent aliquet ligula mi, in luctus elit volutpat porta. Phasellus molestie diam vel nisi sodales, a eleifend augue laoreet. Sed nec eleifend justo. Nam et sollicitudin odio."}</p>

                    <Title size={HeaderSize::Is6}>{"Sixth level"}</Title>
                    <p>{"Cras in nibh lacinia, venenatis nisi et, auctor urna. Donec pulvinar lacus sed diam dignissim, ut eleifend eros accumsan. Phasellus non tortor eros. Ut sed rutrum lacus. Etiam purus nunc, scelerisque quis enim vitae, malesuada ultrices turpis. Nunc vitae maximus purus, nec consectetur dui. Suspendisse euismod, elit vel rutrum commodo, ipsum tortor maximus dui, sed varius sapien odio vitae est. Etiam at cursus metus."}</p>
                </Content>
            }}
            footer={html! {
                <div class="buttons">
                    <Button classes="is-success" onclick={on_save}>{"Save changes"}</Button>
                    <Button onclick={on_cancel}>{"Cancel"}</Button>
                </div>
            }}
        />
        </>
    }
}
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
        <ModalProvider>
            <Hero
                size={HeroSize::Medium}
                classes="is-primary"
                body={html! {
                    <Container>
                        <Title size={HeaderSize::Is1}>{"YBC Modal Example"}</Title>
                        <Content>
                            {"Demonstrating modal components with proper Bulma styling"}
                        </Content>
                    </Container>
                }}
            />

            <Section>
                <Container>
                    <Content>
                        <Title size={HeaderSize::Is3}>{"Basic Modal"}</Title>
                        <p>{"Click the button below to open a basic modal with content."}</p>
                    </Content>

                    <BasicModalExample />                    <div class="mt-6">
                        <Content>
                            <Title size={HeaderSize::Is3}>{"Modal Card"}</Title>
                            <p>{"The ModalCard component provides a structured modal with header, body, and footer sections."}</p>
                            <p>{"Click the Save or Cancel buttons in the modal to see how to programmatically close modals."}</p>
                        </Content>

                        <ModalCardExample />
                    </div>
                </Container>
            </Section>
        </ModalProvider>
        </>
    }
}

fn main() {
    set_panic_hook();

    yew::Renderer::<App>::new().render();
}
