//! Collapsible section component

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CollapsibleProps {
    pub title: String,
    #[prop_or(false)]
    pub initially_open: bool,
    #[prop_or_default]
    pub badge: Option<String>,
    pub children: Children,
}

#[function_component(Collapsible)]
pub fn collapsible(props: &CollapsibleProps) -> Html {
    let is_open = use_state(|| props.initially_open);

    let toggle = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    let arrow = if *is_open { "▼" } else { "▶" };
    let content_class = if *is_open {
        "collapsible-content open"
    } else {
        "collapsible-content"
    };

    html! {
        <div class="collapsible">
            <div class="collapsible-header" onclick={toggle}>
                <span class="collapsible-arrow">{arrow}</span>
                <span class="collapsible-title">{&props.title}</span>
                if let Some(badge) = &props.badge {
                    <span class="collapsible-badge">{badge}</span>
                }
            </div>
            <div class={content_class}>
                {props.children.clone()}
            </div>
        </div>
    }
}
