//! Rust Pipeline view component
//! Shows the compilation pipeline: Rust -> WASM -> COR24 Assembly -> Machine Code -> Execution

use yew::prelude::*;
use super::collapsible::Collapsible;

/// Pre-built example for the Rust pipeline demo
#[derive(Clone, PartialEq)]
pub struct RustExample {
    pub name: String,
    pub description: String,
    pub rust_source: String,
    pub wasm_hex: String,
    pub wasm_size: usize,
    pub cor24_assembly: String,
    pub machine_code_hex: String,
    pub machine_code_size: usize,
    pub listing: String,
}

#[derive(Properties, PartialEq)]
pub struct RustPipelineProps {
    pub examples: Vec<RustExample>,
    pub on_run: Callback<RustExample>,
    pub led_value: u8,
    pub cycle_count: u32,
    pub is_running: bool,
}

#[function_component(RustPipeline)]
pub fn rust_pipeline(props: &RustPipelineProps) -> Html {
    let selected_example = use_state(|| {
        props.examples.first().cloned()
    });

    let on_example_select = {
        let selected_example = selected_example.clone();
        let examples = props.examples.clone();
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<web_sys::HtmlSelectElement>();
            if let Some(select) = target {
                let idx = select.selected_index() as usize;
                if let Some(example) = examples.get(idx) {
                    selected_example.set(Some(example.clone()));
                }
            }
        })
    };

    let on_run_click = {
        let on_run = props.on_run.clone();
        let selected = selected_example.clone();
        Callback::from(move |_| {
            if let Some(example) = &*selected {
                on_run.emit(example.clone());
            }
        })
    };

    html! {
        <div class="rust-pipeline">
            // Example selector
            <div class="pipeline-header">
                <label>{"Example: "}</label>
                <select onchange={on_example_select}>
                    {for props.examples.iter().map(|ex| {
                        html! {
                            <option value={ex.name.clone()}>{&ex.name}{" - "}{&ex.description}</option>
                        }
                    })}
                </select>
                <button class="run-pipeline-btn" onclick={on_run_click} disabled={props.is_running}>
                    {if props.is_running { "Running..." } else { "▶ Run Pipeline" }}
                </button>
            </div>

            if let Some(example) = &*selected_example {
                // Rust Source
                <div class="pipeline-stage">
                    <h3>{"1. Rust Source"}</h3>
                    <pre class="code-block rust-code">{&example.rust_source}</pre>
                </div>

                // WASM Binary (collapsible)
                <Collapsible title="2. WASM Binary" badge={Some(format!("{} bytes", example.wasm_size))}>
                    <pre class="code-block hex-dump">{&example.wasm_hex}</pre>
                </Collapsible>

                // COR24 Assembly (collapsible, initially open)
                <Collapsible title="3. COR24 Assembly" initially_open={true}>
                    <pre class="code-block asm-code">{&example.cor24_assembly}</pre>
                </Collapsible>

                // Machine Code (collapsible)
                <Collapsible title="4. Machine Code" badge={Some(format!("{} bytes", example.machine_code_size))}>
                    <pre class="code-block hex-dump">{&example.machine_code_hex}</pre>
                    <h4>{"Listing:"}</h4>
                    <pre class="code-block listing">{&example.listing}</pre>
                </Collapsible>

                // Execution panel
                <div class="pipeline-stage execution-panel">
                    <h3>{"5. Execution"}</h3>
                    <div class="execution-status">
                        <div class="led-display">
                            <span class="led-label">{"LEDs: "}</span>
                            <div class="led-row">
                                {for (0..8).rev().map(|i| {
                                    let led_on = (props.led_value >> i) & 1 == 1;
                                    let class = if led_on { "led led-on" } else { "led led-off" };
                                    html! {
                                        <div class={class}>{i}</div>
                                    }
                                })}
                            </div>
                            <span class="led-value">{format!("0x{:02X}", props.led_value)}</span>
                        </div>
                        <div class="cycle-count">
                            <span>{"Cycles: "}{props.cycle_count}</span>
                        </div>
                    </div>
                </div>
            }

            // Future: Server-side compilation notice
            <div class="pipeline-note">
                <em>{"Note: Examples are pre-built. Server-side compilation coming soon."}</em>
            </div>
        </div>
    }
}
