//! Tab bar component for switching between Assembler and Rust views

use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Tab {
    pub id: String,
    pub label: String,
}

#[derive(Properties, PartialEq)]
pub struct TabBarProps {
    pub tabs: Vec<Tab>,
    pub active_tab: String,
    pub on_tab_change: Callback<String>,
}

#[function_component(TabBar)]
pub fn tab_bar(props: &TabBarProps) -> Html {
    html! {
        <div class="tab-bar">
            {for props.tabs.iter().map(|tab| {
                let tab_id = tab.id.clone();
                let is_active = props.active_tab == tab.id;
                let class = if is_active { "tab active" } else { "tab" };
                let on_click = {
                    let on_tab_change = props.on_tab_change.clone();
                    let tab_id = tab_id.clone();
                    Callback::from(move |_| on_tab_change.emit(tab_id.clone()))
                };
                html! {
                    <button class={class} onclick={on_click}>
                        {&tab.label}
                    </button>
                }
            })}
        </div>
    }
}
