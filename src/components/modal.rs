use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{HtmlElement, KeyboardEvent};

use yew::prelude::*;

/// Handle for controlling a modal instance
#[derive(Clone, Debug, PartialEq)]
pub struct UseModalHandle {
    is_open: UseStateHandle<bool>,
}

impl UseModalHandle {
    /// Open the modal
    pub fn open(&self) {
        self.is_open.set(true);
    }

    /// Close the modal
    pub fn close(&self) {
        self.is_open.set(false);
    }

    /// Get whether the modal is currently open
    pub fn is_open(&self) -> bool {
        *self.is_open
    }

    /// Get a callback to open the modal
    pub fn open_callback(&self) -> Callback<MouseEvent> {
        let is_open = self.is_open.clone();
        Callback::from(move |_| is_open.set(true))
    }

    /// Get a callback to close the modal
    pub fn close_callback(&self) -> Callback<MouseEvent> {
        let is_open = self.is_open.clone();
        Callback::from(move |_| is_open.set(false))
    }
}

/// Hook to create a modal handle
#[hook]
pub fn use_modal() -> UseModalHandle {
    let is_open = use_state(|| false);

    UseModalHandle { is_open }
}

/// Context for managing modal state across the app
#[derive(Clone, Debug, PartialEq)]
pub struct ModalContext {
    modals: Rc<std::cell::RefCell<HashMap<String, UseModalHandle>>>,
}

impl ModalContext {
    pub fn new() -> Self {
        Self {
            modals: Rc::new(std::cell::RefCell::new(HashMap::new())),
        }
    }

    pub fn register(&self, id: String, handle: UseModalHandle) {
        self.modals.borrow_mut().insert(id, handle);
    }

    pub fn close(&self, id: &str) {
        if let Some(handle) = self.modals.borrow().get(id) {
            handle.close();
        }
    }

    pub fn open(&self, id: &str) {
        if let Some(handle) = self.modals.borrow().get(id) {
            handle.open();
        }
    }

    pub fn get_handle(&self, id: &str) -> Option<UseModalHandle> {
        self.modals.borrow().get(id).cloned()
    }
}

impl Default for ModalContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Modal actions.
pub enum ModalMsg {
    Open,
    Close,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ModalProps {
    /// The ID of this modal, used for triggering close events from other parts of the app.
    pub id: String,
    /// The modal handle that controls this modal's state
    pub handle: UseModalHandle,
    /// The content of the `"modal-content"` element.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

/// A classic modal overlay, in which you can include any content you want.
///
/// [https://bulma.io/documentation/components/modal/](https://bulma.io/documentation/components/modal/)
#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let modal_ref = use_node_ref();

    let mut class = Classes::from("modal");
    class.push(props.classes.clone());

    if props.handle.is_open() {
        class.push("is-active");
    }

    let close_modal = props.handle.close_callback();

    // Register with context if available
    {
        let id = props.id.clone();
        let handle = props.handle.clone();
        let context = use_context::<ModalContext>();

        use_effect_with(id.clone(), move |id| {
            if let Some(context) = context {
                context.register(id.clone(), handle);
            }
            || {}
        });
    }

    // Handle keyboard events for accessibility
    {
        let is_active_value = props.handle.is_open();
        let handle = props.handle.clone();
        let modal_ref = modal_ref.clone();

        use_effect_with(is_active_value, move |active| {
            if *active {
                let document = web_sys::window().unwrap().document().unwrap();

                // Set up keyboard event listener for Escape key
                let handle_for_keyboard = handle.clone();
                let keyboard_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                    if event.key() == "Escape" {
                        handle_for_keyboard.close();
                    }
                }) as Box<dyn FnMut(_)>);

                document
                    .add_event_listener_with_callback("keydown", keyboard_closure.as_ref().unchecked_ref())
                    .unwrap();

                // Focus the modal for accessibility
                if let Some(modal) = modal_ref.cast::<HtmlElement>() {
                    let _ = modal.focus();
                }

                Box::new(move || {
                    let _ = document.remove_event_listener_with_callback("keydown", keyboard_closure.as_ref().unchecked_ref());
                    drop(keyboard_closure);
                }) as Box<dyn FnOnce()>
            } else {
                Box::new(|| {}) as Box<dyn FnOnce()>
            }
        });
    }

    html! {
        <div id={props.id.clone()} {class} ref={modal_ref} tabindex="-1">
            <div class="modal-background" onclick={close_modal.clone()}></div>
            <div class="modal-content">
                {props.children.clone()}
            </div>
            <button class="modal-close is-large" aria-label="close" onclick={close_modal}></button>
        </div>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ModalCardProps {
    /// The ID of this modal, used for triggering close events from other parts of the app.
    pub id: String,
    /// The modal handle that controls this modal's state
    pub handle: UseModalHandle,
    /// The title of this modal.
    pub title: String,
    /// The content to be placed in the `modal-card-body` not including the modal-card-header /
    /// modal-card-title, which is handled by the `modal_title` prop.
    #[prop_or_default]
    pub body: Html,
    /// The content to be placed in the `modal-card-footer`.
    #[prop_or_default]
    pub footer: Html,
    #[prop_or_default]
    pub classes: Classes,
}

/// A classic modal with a header, body, and footer section.
///
/// [https://bulma.io/documentation/components/modal/](https://bulma.io/documentation/components/modal/)
#[function_component(ModalCard)]
pub fn modal_card(props: &ModalCardProps) -> Html {
    let modal_ref = use_node_ref();

    let mut class = Classes::from("modal");
    class.push(props.classes.clone());

    if props.handle.is_open() {
        class.push("is-active");
    }

    let close_modal = props.handle.close_callback();

    // Register with context if available
    {
        let id = props.id.clone();
        let handle = props.handle.clone();
        let context = use_context::<ModalContext>();

        use_effect_with(id.clone(), move |id| {
            if let Some(context) = context {
                context.register(id.clone(), handle);
            }
            || {}
        });
    }

    // Handle keyboard events for accessibility
    {
        let is_active_value = props.handle.is_open();
        let handle = props.handle.clone();
        let modal_ref = modal_ref.clone();

        use_effect_with(is_active_value, move |active| {
            if *active {
                let document = web_sys::window().unwrap().document().unwrap();

                // Set up keyboard event listener for Escape key
                let handle_for_keyboard = handle.clone();
                let keyboard_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                    if event.key() == "Escape" {
                        handle_for_keyboard.close();
                    }
                }) as Box<dyn FnMut(_)>);

                document
                    .add_event_listener_with_callback("keydown", keyboard_closure.as_ref().unchecked_ref())
                    .unwrap();

                // Focus the modal for accessibility
                if let Some(modal) = modal_ref.cast::<HtmlElement>() {
                    let _ = modal.focus();
                }

                Box::new(move || {
                    let _ = document.remove_event_listener_with_callback("keydown", keyboard_closure.as_ref().unchecked_ref());
                    drop(keyboard_closure);
                }) as Box<dyn FnOnce()>
            } else {
                Box::new(|| {}) as Box<dyn FnOnce()>
            }
        });
    }

    html! {
        <div id={props.id.clone()} {class} ref={modal_ref} tabindex="-1">
            <div class="modal-background" onclick={close_modal.clone()}></div>
            <div class="modal-card">
                <header class="modal-card-head">
                    <p class="modal-card-title">{props.title.clone()}</p>
                    <button class="delete" aria-label="close" onclick={close_modal.clone()}></button>
                </header>
                <section class="modal-card-body">
                    {props.body.clone()}
                </section>
                <footer class="modal-card-foot">
                    {props.footer.clone()}
                </footer>
            </div>
            <button class="modal-close is-large" aria-label="close" onclick={close_modal}></button>
        </div>
    }
}

//////////////////////////////////////////////////////////////////////////////

/// A request to close a modal instance by ID.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModalCloseMsg(pub String);

/// A context provider for modal management
#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ModalProviderProps {
    pub children: Children,
}

#[function_component(ModalProvider)]
pub fn modal_provider(props: &ModalProviderProps) -> Html {
    let context = use_state(|| ModalContext::new());

    html! {
        <ContextProvider<ModalContext> context={(*context).clone()}>
            {props.children.clone()}
        </ContextProvider<ModalContext>>
    }
}
