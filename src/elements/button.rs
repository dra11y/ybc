use derive_more::Display;
use yew::events::{Event, MouseEvent};
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The size for all buttons within this group.
    #[prop_or_default]
    pub size: Option<ButtonGroupSize>,
}

/// A container for a group of buttons.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
#[function_component(Buttons)]
pub fn buttons(props: &ButtonsProps) -> Html {
    let class = classes!("buttons", props.classes.clone(), props.size.as_ref().map(ToString::to_string));
    html! {
        <div {class}>
            {props.children.clone()}
        </div>
    }
}

/// The 3 sizes available for a button group.
///
/// https://bulma.io/documentation/elements/button/#sizes
#[derive(Clone, Debug, Display, PartialEq, Eq)]
#[display("are-{_variant}")]
pub enum ButtonGroupSize {
    #[display("small")]
    Small,
    #[display("medium")]
    Medium,
    #[display("large")]
    Large,
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The click handler to use for this component.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// A button element.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let class = classes!(
        "button",
        props.classes.clone(),
        props.loading.then_some("is-loading"),
        props.r#static.then_some("is-static")
    );
    html! {
        <button {class} onclick={props.onclick.clone()} disabled={props.disabled}>
            {props.children.clone()}
        </button>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "router")]
mod router {
    use super::*;
    use serde::Serialize;
    use yew_router::Routable;
    use yew_router::components::Link;

    #[derive(Clone, Properties, PartialEq)]
    pub struct ButtonRouterProps<R: Routable + Clone + PartialEq + 'static> {
        /// The Switched item representing the route.
        pub route: R,
        /// Html inside the component.
        #[prop_or_default]
        pub children: Children,
        /// Classes to be added to component.
        #[prop_or_default]
        pub classes: Classes,
        /// Render a loading spinner within this component.
        #[prop_or_default]
        pub loading: bool,
        /// Make this component static.
        #[prop_or_default]
        pub r#static: bool,
        /// Disable this component.
        #[prop_or_default]
        pub disabled: bool,
    }

    /// A Yew Router button element with Bulma styling.
    pub struct ButtonRouter<R: Routable + Clone + PartialEq + 'static, Q: Clone + PartialEq + Serialize + 'static = ()> {
        _route: std::marker::PhantomData<R>,
        _query: std::marker::PhantomData<Q>,
    }

    impl<R: Routable + Clone + PartialEq + 'static, Q: Clone + PartialEq + Serialize + 'static> Component for ButtonRouter<R, Q> {
        type Message = ();
        type Properties = ButtonRouterProps<R>;

        fn create(_ctx: &Context<Self>) -> Self {
            Self {
                _route: std::marker::PhantomData,
                _query: std::marker::PhantomData,
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let loading = ctx.props().loading.then_some("is-loading");
            let classes = classes!(ctx.props().classes.clone(), "button", loading);
            html! {
                <Link<R, Q>
                    to={ctx.props().route.clone()}
                    disabled={ctx.props().disabled}
                    {classes}
                    children={ctx.props().children.clone()}
                />
            }
        }
    }

    /// A Yew Router anchor button element with Bulma styling.
    pub struct ButtonAnchorRouter<R: Routable + Clone + PartialEq + 'static, Q: Clone + PartialEq + Serialize + 'static = ()> {
        _route: std::marker::PhantomData<R>,
        _query: std::marker::PhantomData<Q>,
    }

    impl<R: Routable + Clone + PartialEq + 'static, Q: Clone + PartialEq + Serialize + 'static> Component for ButtonAnchorRouter<R, Q> {
        type Message = ();
        type Properties = ButtonRouterProps<R>;

        fn create(_ctx: &Context<Self>) -> Self {
            Self {
                _route: std::marker::PhantomData,
                _query: std::marker::PhantomData,
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let loading = ctx.props().loading.then_some("is-loading");
            let classes = classes!(ctx.props().classes.clone(), "button", loading);
            html! {
                <Link<R, Q>
                    to={ctx.props().route.clone()}
                    disabled={ctx.props().disabled}
                    {classes}
                    children={ctx.props().children.clone()}
                />
            }
        }
    }
}

#[cfg(feature = "router")]
pub use router::{ButtonAnchorRouter, ButtonRouter, ButtonRouterProps};

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonAnchorProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The `href` attribute value to use for this component.
    #[prop_or_default]
    pub href: String,
    /// The click handler to use for this component.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// An optional `rel` for when this element is using the `a` tag.
    #[prop_or_default]
    pub rel: Option<String>,
    /// An optional `target` for when this element is using the `a` tag.
    #[prop_or_default]
    pub target: Option<String>,
}

/// An anchor element styled as a button.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
#[function_component(ButtonAnchor)]
pub fn button_anchor(props: &ButtonAnchorProps) -> Html {
    let class = classes!(
        "button",
        props.classes.clone(),
        props.loading.then_some("is-loading"),
        props.r#static.then_some("is-static")
    );
    html! {
        <a
            {class}
            onclick={props.onclick.clone()}
            href={props.href.clone()}
            rel={props.rel.clone().unwrap_or_default()}
            target={props.target.clone().unwrap_or_default()}
            disabled={props.disabled}
        >
            {props.children.clone()}
        </a>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Display)]
pub enum ButtonInputType {
    #[display("button")]
    Button(Callback<MouseEvent>),
    #[display("submit")]
    Submit(Callback<SubmitEvent>),
    #[display("color")]
    Color { onchange: Callback<Event>, hidden: bool },
    #[display("reset")]
    Reset(Callback<Event>),
}

impl ButtonInputType {
    pub fn onclick(&self) -> Option<Callback<MouseEvent>> {
        match self {
            ButtonInputType::Button(callback) => Some(callback.clone()),
            _ => None,
        }
    }

    pub fn onsubmit(&self) -> Option<Callback<SubmitEvent>> {
        match self {
            ButtonInputType::Submit(callback) => Some(callback.clone()),
            _ => None,
        }
    }

    pub fn onreset(&self) -> Option<Callback<Event>> {
        match self {
            ButtonInputType::Reset(callback) => Some(callback.clone()),
            _ => None,
        }
    }

    pub fn onchange(&self) -> Option<Callback<Event>> {
        match self {
            ButtonInputType::Color { onchange: callback, .. } => Some(callback.clone()),
            _ => None,
        }
    }
}

impl Default for ButtonInputType {
    fn default() -> Self {
        ButtonInputType::Button(Callback::noop())
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonInputProps {
    /// The button type and click handler to use for this component.
    #[prop_or_default]
    pub r#type: ButtonInputType,
    /// Optional button value
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub classes: Classes,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// An `<input type="...">` button of the given type.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
#[function_component(ButtonInput)]
pub fn button_input(props: &ButtonInputProps) -> Html {
    let class = classes!(
        "button",
        props.classes.clone(),
        props.loading.then_some("is-loading"),
        props.r#static.then_some("is-static"),
    );
    let style = match props.r#type {
        ButtonInputType::Color { hidden, .. } if hidden => Some(format!(
            "{}; opacity: 0; width: 100%; height: 100%; cursor: pointer;",
            props.style.clone().unwrap_or_default()
        )),
        _ => props.style.clone().map(|s| s.to_string()),
    };
    html! {
        <input
            type={props.r#type.to_string()}
            {class}
            onclick={props.r#type.onclick()}
            onsubmit={props.r#type.onsubmit()}
            onreset={props.r#type.onreset()}
            onchange={props.r#type.onchange()}
            value={props.value.clone()}
            disabled={props.disabled}
            style={style}
        />
    }
}
