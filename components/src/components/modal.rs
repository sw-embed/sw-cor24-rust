use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub id: String,
    pub title: String,
    pub active: bool,
    pub on_close: Callback<()>,
    pub children: Children,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let modal_class = classes!("modal", props.active.then_some("active"));

    // Escape key closes modal
    {
        let active = props.active;
        let on_close = props.on_close.clone();
        use_effect_with(active, move |&active| {
            let cleanup: Option<JsValue> = if active {
                let document = web_sys::window().unwrap().document().unwrap();
                let closure = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(
                    move |e: web_sys::KeyboardEvent| {
                        if e.key() == "Escape" {
                            on_close.emit(());
                        }
                    },
                );
                document
                    .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
                    .ok();
                Some(closure.into_js_value())
            } else {
                None
            };
            move || {
                if let Some(closure) = cleanup
                    && let Some(document) = web_sys::window().and_then(|w| w.document())
                {
                    document
                        .remove_event_listener_with_callback("keydown", closure.unchecked_ref())
                        .ok();
                }
            }
        });
    }

    let on_overlay_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |e: MouseEvent| {
            // Only close if clicking the overlay itself, not the content
            #[allow(clippy::collapsible_if)]
            if let Some(target) = e.target() {
                if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
                    if element.class_name().contains("modal") {
                        on_close.emit(());
                    }
                }
            }
        })
    };

    let on_close_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_: MouseEvent| on_close.emit(()))
    };

    html! {
        <div id={props.id.clone()} class={modal_class} onclick={on_overlay_click}>
            <div class="modal-content" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                <span class="modal-close" onclick={on_close_click}>
                    {"×"}
                </span>
                <h2 class="modal-title">{&props.title}</h2>
                <div class="modal-body">
                    {props.children.clone()}
                </div>
            </div>
        </div>
    }
}
