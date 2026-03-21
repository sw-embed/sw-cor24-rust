//! C Pipeline view component - Wizard-driven layout
//! Shows the C compilation pipeline: C Source -> COR24 Assembly -> Execution
//! Simpler than Rust pipeline (no MSP430 intermediate step).

use std::cell::Cell;
use std::rc::Rc;

use yew::prelude::*;

use crate::components::debug_panel::DebugPanel;
use crate::components::rust_pipeline::EmulatorState;

/// Pre-built example for the C pipeline demo
#[derive(Clone, PartialEq)]
pub struct CExample {
    pub name: String,
    pub description: String,
    pub c_source: String,
    pub cor24_assembly: String,
}

/// Wizard steps for C pipeline
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CWizardStep {
    Source,    // C source code
    Compile,  // COR24 assembly output
    Assemble, // Assembled + debugger
}

impl CWizardStep {
    fn label(&self) -> &'static str {
        match self {
            CWizardStep::Source => "Source",
            CWizardStep::Compile => "Compile",
            CWizardStep::Assemble => "Assemble",
        }
    }

    fn next(&self) -> Option<CWizardStep> {
        match self {
            CWizardStep::Source => Some(CWizardStep::Compile),
            CWizardStep::Compile => Some(CWizardStep::Assemble),
            CWizardStep::Assemble => None,
        }
    }

    fn action_label(&self) -> &'static str {
        match self {
            CWizardStep::Source => "Compile",
            CWizardStep::Compile => "Assemble",
            CWizardStep::Assemble => "",
        }
    }

    fn action_tooltip(&self) -> &'static str {
        match self {
            CWizardStep::Source => "Compile C to COR24 assembly",
            CWizardStep::Compile => "Assemble COR24 code into machine code",
            CWizardStep::Assemble => "",
        }
    }

    fn step_tooltip(&self) -> &'static str {
        match self {
            CWizardStep::Source => "C source code (from MakerLisp's cc24 compiler)",
            CWizardStep::Compile => "COR24 assembly (compiler output + runtime stubs)",
            CWizardStep::Assemble => "Machine code execution and debug",
        }
    }

    fn cell_id(&self) -> &'static str {
        match self {
            CWizardStep::Source => "c-cell-source",
            CWizardStep::Compile => "c-cell-asm",
            CWizardStep::Assemble => "c-cell-debug",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct CPipelineProps {
    pub examples: Vec<CExample>,
    #[prop_or_default]
    pub loaded_example: Option<CExample>,
    #[prop_or_default]
    pub load_generation: u32,
    pub on_load: Callback<CExample>,
    pub on_step: Callback<u32>,
    pub on_run: Callback<()>,
    pub on_stop: Callback<()>,
    pub on_reset: Callback<()>,
    pub cpu_state: EmulatorState,
    pub is_loaded: bool,
    pub is_running: bool,
    pub switch_value: u8,
    pub on_switch_toggle: Callback<u8>,
    pub on_uart_send: Callback<u8>,
    #[prop_or_default]
    pub on_uart_clear: Callback<()>,
    #[prop_or_default]
    pub run_speed_ms: Option<Rc<Cell<u32>>>,
    #[prop_or_default]
    pub on_tutorial_open: Callback<()>,
    #[prop_or_default]
    pub on_examples_open: Callback<()>,
    #[prop_or_default]
    pub on_isa_ref_open: Callback<()>,
    #[prop_or_default]
    pub on_help_open: Callback<()>,
}

#[function_component(CPipeline)]
pub fn c_pipeline(props: &CPipelineProps) -> Html {
    let current_step = use_state(|| CWizardStep::Source);

    // Auto-load first example on first render (skip if already loaded, e.g. tab switch)
    {
        let on_load = props.on_load.clone();
        let examples = props.examples.clone();
        let already_loaded = props.loaded_example.is_some();
        use_effect_with((), move |_| {
            if !already_loaded
                && let Some(first) = examples.first() {
                    on_load.emit(first.clone());
                }
            || ()
        });
    }

    // Reset wizard to Source step when an example is loaded
    {
        let current_step = current_step.clone();
        let load_gen = props.load_generation;
        use_effect_with(load_gen, move |_| {
            current_step.set(CWizardStep::Source);
            // Scroll notebook to top so source cell is visible
            if let Some(window) = web_sys::window()
                && let Some(document) = window.document()
                && let Some(container) = document.get_element_by_id("c-notebook-scroll")
            {
                container.set_scroll_top(0);
            }
            || ()
        });
    }

    // Advance to next wizard step
    let advance_step = {
        let current_step = current_step.clone();
        Callback::from(move |_| {
            let current = *current_step;
            if let Some(next) = current.next() {
                current_step.set(next);
                let scroll_to_cell = next.cell_id().to_string();
                gloo::timers::callback::Timeout::new(100, move || {
                    if let Some(window) = web_sys::window()
                        && let Some(document) = window.document()
                        && let Some(element) = document.get_element_by_id(&scroll_to_cell)
                    {
                        element.scroll_into_view();
                    }
                }).forget();
            }
        })
    };

    // Click on a wizard step
    let on_step_click = {
        let current_step = current_step.clone();
        move |step: CWizardStep| {
            let current_step = current_step.clone();
            Callback::from(move |_| {
                if step <= *current_step {
                    let cell_id = step.cell_id().to_string();
                    if let Some(window) = web_sys::window()
                        && let Some(document) = window.document()
                        && let Some(element) = document.get_element_by_id(&cell_id)
                    {
                        element.scroll_into_view();
                    }
                }
            })
        }
    };

    let all_steps = [CWizardStep::Source, CWizardStep::Compile, CWizardStep::Assemble];

    html! {
        <div class="rust-wizard-layout">
            // Column 1: Sidebar
            <div class="wizard-sidebar">
                <div class="wizard-buttons">
                    <button data-tooltip="Step-by-step guide to the C pipeline" onclick={
                        let cb = props.on_tutorial_open.clone();
                        Callback::from(move |_| cb.emit(()))
                    }>{"Tutorial"}</button>
                    <button data-tooltip="COR24 instruction set reference" onclick={
                        let cb = props.on_isa_ref_open.clone();
                        Callback::from(move |_| cb.emit(()))
                    }>{"ISA Ref"}</button>
                    <button data-tooltip="Help and keyboard shortcuts" onclick={
                        let cb = props.on_help_open.clone();
                        Callback::from(move |_| cb.emit(()))
                    }>{"Help"}</button>
                    <a href="https://software-wrighter-lab.github.io/" target="_blank" rel="noopener"
                       class="sidebar-link" data-tooltip="SW-Lab Blog">{"Blog"}<span class="ext-icon">{" \u{2197}"}</span></a>
                    <a href="https://discord.com/invite/Ctzk5uHggZ" target="_blank" rel="noopener"
                       class="sidebar-link" data-tooltip="SW-Lab Discord">{"Discord"}<span class="ext-icon">{" \u{2197}"}</span></a>
                    <a href="https://www.makerlisp.com/" target="_blank" rel="noopener"
                       class="sidebar-link" data-tooltip="MakerLisp">{"MakerLisp"}<span class="ext-icon">{" \u{2197}"}</span></a>
                </div>
            </div>

            // Column 2: Wizard steps
            <div class="wizard-steps">
                <div class="wizard-header-btn">
                    <button class="toolbar-btn"
                        data-tooltip="Load a pre-built C example"
                        onclick={
                            let cb = props.on_examples_open.clone();
                            Callback::from(move |_: yew::MouseEvent| cb.emit(()))
                        }>{"Examples"}</button>
                </div>

                {for all_steps.iter().map(|&step| {
                    let is_completed = props.is_loaded && step <= *current_step;
                    let is_active = step == *current_step;
                    let is_disabled = step > *current_step;

                    let class = classes!(
                        "wizard-step",
                        is_active.then_some("active"),
                        is_completed.then_some("completed"),
                        is_disabled.then_some("disabled"),
                    );

                    let indicator = if is_completed { "✓" } else { "○" };

                    html! {
                        <div
                            class={class}
                            onclick={if !is_disabled { Some(on_step_click(step)) } else { None }}
                            data-tooltip={step.step_tooltip()}
                        >
                            <span class="step-indicator">{indicator}</span>
                            <span class="step-label">{step.label()}</span>
                        </div>
                    }
                })}

                if props.is_loaded && *current_step != CWizardStep::Assemble {
                    <button class="wizard-action-btn" onclick={advance_step.clone()}
                        data-tooltip={(*current_step).action_tooltip()}>
                        {(*current_step).action_label()}
                    </button>
                }

                <div class="wizard-spacer"></div>
            </div>

            // Column 3: Notebook cells
            <div class="notebook-cells" id="c-notebook-scroll">
                if let Some(example) = &props.loaded_example {
                    // Cell 1: C Source (always visible)
                    <div class="notebook-cell" id="c-cell-source">
                        <div class="cell-header">
                            <span>{"C Source"}</span>
                            <span class="cell-header-example">{&example.name}</span>
                        </div>
                        <div class="cell-content">
                            <pre class="code-block rust-code">{&example.c_source}</pre>
                        </div>
                    </div>

                    // Cell 2: COR24 Assembly (visible after Compile step)
                    if *current_step >= CWizardStep::Compile {
                        <div class="notebook-cell" id="c-cell-asm">
                            <div class="cell-header">
                                <span>{"COR24 Assembly"}</span>
                                <span class="cell-header-example">{&example.name}</span>
                            </div>
                            <div class="cell-content">
                                <pre class="code-block asm-code">{&example.cor24_assembly}</pre>
                            </div>
                        </div>
                    }

                    // Cell 3: Debug Panel (visible after Assemble step)
                    if *current_step >= CWizardStep::Assemble {
                        <div class="notebook-cell notebook-cell-debug" id="c-cell-debug">
                            <div class="cell-header">
                                <span>{"Execution"}</span>
                                <span class="cell-header-example">{&example.name}</span>
                            </div>
                            <DebugPanel
                                cpu_state={props.cpu_state.clone()}
                                is_loaded={props.is_loaded}
                                is_running={props.is_running}
                                on_step={props.on_step.clone()}
                                on_run={props.on_run.clone()}
                                on_stop={props.on_stop.clone()}
                                on_reset={props.on_reset.clone()}
                                switch_value={props.switch_value}
                                on_switch_toggle={props.on_switch_toggle.clone()}
                                on_uart_send={props.on_uart_send.clone()}
                                on_uart_clear={props.on_uart_clear.clone()}
                                listing_scroll_id={"c-asm-listing-scroll".to_string()}
                                run_speed_ms={props.run_speed_ms.clone()}
                            />
                        </div>
                    }
                } else {
                    <div class="notebook-placeholder">
                        <p>{"Click "}<strong>{"Examples"}</strong>{" to load a C program."}</p>
                    </div>
                }

                <div class="pipeline-note">
                    <em>{"Pipeline: C \u{2192} cc24 (MakerLisp) \u{2192} COR24 ASM \u{2192} Machine Code. Examples are pre-built with injected runtime stubs."}</em>
                </div>
            </div>
        </div>
    }
}
