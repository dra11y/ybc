use yew::prelude::*;

use crate::elements::button::Button;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct DropdownProps {
    /// The content of the dropdown menu.
    ///
    /// This content will be placed directly within the `div.dropdown-content` container.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// Make this dropdown triggerable based on hover.
    #[prop_or_default]
    pub hoverable: bool,
    /// Any additional classes to use for the trigger button.
    #[prop_or_default]
    pub button_classes: Classes,
    /// The content of the trigger button.
    #[prop_or_default]
    pub button_html: Html,
    /// Force the dropdown open (adds `is-active`).
    #[prop_or_default]
    pub active: bool,
    /// Optional callback when open state changes due to user interaction.
    #[prop_or_default]
    pub on_active_change: Callback<bool>,
}

/// Dropdown actions.
pub enum DropdownMsg {
    Open,
    Close,
}

/// An interactive dropdown menu for discoverable content.
///
/// [https://bulma.io/documentation/components/dropdown/](https://bulma.io/documentation/components/dropdown/)
pub struct Dropdown {
    is_active: bool,
}

impl Component for Dropdown {
    type Message = DropdownMsg;
    type Properties = DropdownProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { is_active: ctx.props().active }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        if ctx.props().hoverable {
            return false;
        }
        match msg {
            DropdownMsg::Open => {
                self.is_active = true;
                ctx.props().on_active_change.emit(true);
            }
            DropdownMsg::Close => {
                self.is_active = false;
                ctx.props().on_active_change.emit(false);
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old: &Self::Properties) -> bool {
        self.is_active = ctx.props().active;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut class = Classes::from("dropdown");
        class.push(ctx.props().classes.clone());
        let is_active = ctx.props().active || self.is_active;
        let closecb = ctx.link().callback(|_| DropdownMsg::Close);
        let opencb = if ctx.props().hoverable {
            class.push("is-hoverable");
            Callback::noop()
        } else {
            if is_active {
                closecb
            } else {
                ctx.link().callback(|_| DropdownMsg::Open)
            }
        };
        let overlay = if is_active && !ctx.props().active {
            class.push("is-active");
            html! {<div onclick={ctx.link().callback(|_| DropdownMsg::Close)} style="z-index:10;background-color:rgba(0,0,0,0);position:fixed;top:0;bottom:0;left:0;right:0;"></div>}
        } else {
            if is_active {
                class.push("is-active");
            }
            html! {}
        };
        html! {
            <div {class}>
                {overlay}
                <div class="dropdown-trigger">
                    <Button classes={ctx.props().button_classes.clone()} onclick={opencb}>
                        {ctx.props().button_html.clone()}
                    </Button>
                </div>
                <div class="dropdown-menu" role="menu">
                    <div class="dropdown-content">
                        {ctx.props().children.clone()}
                    </div>
                </div>
            </div>
        }
    }
}
