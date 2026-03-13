//! Global tooltip component using position:fixed div
//! Listens for mouseover/mouseout on [data-tooltip] elements

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[function_component(Tooltip)]
pub fn tooltip() -> Html {
    let text = use_state(String::new);
    let visible = use_state(|| false);
    let x = use_state(|| 0.0_f64);
    let y = use_state(|| 0.0_f64);
    // Bumped after each show to force a second render for width measurement
    let layout_pass = use_state(|| 0_u32);

    {
        let text = text.clone();
        let visible = visible.clone();
        let x = x.clone();
        let y = y.clone();
        let layout_pass = layout_pass.clone();

        use_effect_with((), move |_| {
            let document = web_sys::window().unwrap().document().unwrap();

            let text2 = text.clone();
            let visible2 = visible.clone();
            let x2 = x.clone();
            let y2 = y.clone();
            let lp2 = layout_pass.clone();
            let mouseover = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
                if let Some(target) = e.target() {
                    let el: &web_sys::Element = target.unchecked_ref();
                    if let Some(tooltip_el) = el.closest("[data-tooltip]").ok().flatten()
                        && let Some(tip) = tooltip_el.get_attribute("data-tooltip")
                    {
                        text2.set(tip);
                        visible2.set(true);
                        let rect = tooltip_el.get_bounding_client_rect();
                        x2.set(rect.left() + rect.width() / 2.0);
                        y2.set(rect.bottom() + 6.0);
                        // Trigger a second render so width clamping works
                        lp2.set(*lp2 + 1);
                    }
                }
            });

            let visible3 = visible.clone();
            let mouseout = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
                if let Some(target) = e.target() {
                    let el: &web_sys::Element = target.unchecked_ref();
                    if el.closest("[data-tooltip]").ok().flatten().is_some() {
                        visible3.set(false);
                    }
                }
            });

            document.add_event_listener_with_callback("mouseover", mouseover.as_ref().unchecked_ref()).ok();
            document.add_event_listener_with_callback("mouseout", mouseout.as_ref().unchecked_ref()).ok();

            // Leak closures — they live for the lifetime of the app
            mouseover.forget();
            mouseout.forget();

            || ()
        });
    }

    // Use layout_pass to suppress unused warning
    let _ = *layout_pass;

    let tip_ref = use_node_ref();

    // Adjust position to keep tooltip on screen
    let mut left = *x;
    let top = *y;
    if let Some(el) = tip_ref.cast::<web_sys::HtmlElement>() {
        let w = el.offset_width() as f64;
        let window = web_sys::window().unwrap();
        let vw = window.inner_width().unwrap().as_f64().unwrap_or(1280.0);
        left -= w / 2.0;
        if left + w > vw - 8.0 { left = vw - w - 8.0; }
        if left < 8.0 { left = 8.0; }
    } else {
        left -= 50.0; // rough center before first render
    }

    let style = if *visible {
        format!(
            "position:fixed;pointer-events:none;background:#1a1a2e;color:#eee;\
             padding:8px 14px;border-radius:5px;border:1px solid #555;\
             font-size:14px;white-space:nowrap;z-index:99999;\
             opacity:1;transition:opacity 0.12s;\
             left:{left:.0}px;top:{top:.0}px;"
        )
    } else {
        "position:fixed;pointer-events:none;opacity:0;transition:opacity 0.12s;left:-9999px;".to_string()
    };

    html! {
        <div ref={tip_ref} id="tooltip" style={style}>{&*text}</div>
    }
}
