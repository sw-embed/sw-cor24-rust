//! Rust Pipeline view component - Wizard-driven 3-column layout
//! Shows the compilation pipeline: Rust -> WASM -> COR24 Assembly -> Machine Code -> Execution

use wasm_bindgen::JsCast;
use yew::prelude::*;

/// Pre-built example for the Rust pipeline demo
#[derive(Clone, PartialEq)]
pub struct RustExample {
    pub name: String,
    pub description: String,
    pub rust_source: String,
    pub wasm_hex: String,
    pub wasm_size: usize,
    pub wasm_disassembly: String,
    pub cor24_assembly: String,
    pub machine_code_hex: String,
    pub machine_code_size: usize,
    pub listing: String,
}

/// CPU state for display in the Rust pipeline execution panel
#[derive(Clone, PartialEq, Default)]
pub struct RustCpuState {
    pub registers: [u32; 8],
    pub prev_registers: [u32; 8],      // Changed last step (hot)
    pub prev_prev_registers: [u32; 8], // Changed 2 steps ago (warm)
    pub pc: u32,
    pub condition_flag: bool,
    pub is_halted: bool,
    pub led_value: u8,
    pub cycle_count: u32,
    pub memory_low: Vec<u8>,
    pub memory_high: Vec<u8>,
    pub memory_stack: Vec<u8>,          // Stack region around SP
    pub stack_base_addr: u32,           // Base address of stack region
    pub program_end: u32,               // End of code+data region
    pub prev_memory_low: Vec<u8>,      // Changed last step (hot)
    pub prev_memory_high: Vec<u8>,
    pub prev_memory_stack: Vec<u8>,
    pub prev_prev_memory_low: Vec<u8>, // Changed 2 steps ago (warm)
    pub prev_prev_memory_high: Vec<u8>,
    pub prev_prev_memory_stack: Vec<u8>,
    pub current_instruction: String,
    pub assembled_lines: Vec<String>,
}

/// Wizard steps for progressive disclosure
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WizardStep {
    Source,    // Initial state - only Source visible
    Compile,   // WASM binary revealed
    Disasm,    // WASM disassembly revealed
    Translate, // COR24 assembly revealed
    Assemble,  // Debugger revealed
}

impl WizardStep {
    fn label(&self) -> &'static str {
        match self {
            WizardStep::Source => "Source",
            WizardStep::Compile => "Compile",
            WizardStep::Disasm => "Disasm",
            WizardStep::Translate => "Translate",
            WizardStep::Assemble => "Assemble",
        }
    }

    fn next(&self) -> Option<WizardStep> {
        match self {
            WizardStep::Source => Some(WizardStep::Compile),
            WizardStep::Compile => Some(WizardStep::Disasm),
            WizardStep::Disasm => Some(WizardStep::Translate),
            WizardStep::Translate => Some(WizardStep::Assemble),
            WizardStep::Assemble => None,
        }
    }

    fn action_label(&self) -> &'static str {
        match self {
            WizardStep::Source => "Compile",
            WizardStep::Compile => "Disassemble",
            WizardStep::Disasm => "Translate",
            WizardStep::Translate => "Assemble",
            WizardStep::Assemble => "",
        }
    }

    fn cell_id(&self) -> &'static str {
        match self {
            WizardStep::Source => "cell-source",
            WizardStep::Compile => "cell-wasm",
            WizardStep::Disasm => "cell-disasm",
            WizardStep::Translate => "cell-cor24",
            WizardStep::Assemble => "cell-debug",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct RustPipelineProps {
    pub examples: Vec<RustExample>,
    #[prop_or_default]
    pub loaded_example: Option<RustExample>,
    pub on_load: Callback<RustExample>,
    pub on_step: Callback<()>,
    pub on_run: Callback<()>,
    pub on_stop: Callback<()>,
    pub on_reset: Callback<()>,
    #[prop_or_default]
    pub on_unload: Callback<()>,
    pub cpu_state: RustCpuState,
    pub is_loaded: bool,
    pub is_running: bool,
    pub switch_value: u8,
    pub on_switch_toggle: Callback<u8>,
    // Modal callbacks - passed from app.rs
    #[prop_or_default]
    pub on_tutorial_open: Callback<()>,
    #[prop_or_default]
    pub on_examples_open: Callback<()>,
    #[prop_or_default]
    pub on_isa_ref_open: Callback<()>,
    #[prop_or_default]
    pub on_help_open: Callback<()>,
}

#[function_component(RustPipeline)]
pub fn rust_pipeline(props: &RustPipelineProps) -> Html {
    // Wizard step state
    let current_step = use_state(|| WizardStep::Source);

    // Load dialog state
    let load_dialog_open = use_state(|| false);

    // Selected example state - None means "Choose an example..." is shown
    let selected_example = use_state(|| None::<RustExample>);

    // Track PC for auto-scroll
    let last_pc = use_state(|| 0u32);

    // Auto-scroll current assembly line into view when PC changes
    // Only scrolls within the listing container, not the whole page
    {
        let pc = props.cpu_state.pc;
        let last_pc = last_pc.clone();
        use_effect_with(pc, move |&current_pc| {
            if current_pc != *last_pc {
                last_pc.set(current_pc);
                // Scroll within the listing container only
                if let Some(window) = web_sys::window()
                    && let Some(document) = window.document()
                    && let Some(container) = document.get_element_by_id("asm-listing-scroll")
                    && let Some(element) = document.query_selector(".asm-line.current-line").ok().flatten()
                {
                    let element_html: &web_sys::HtmlElement = element.unchecked_ref();
                    let container_html: &web_sys::HtmlElement = container.unchecked_ref();
                    let element_top = element_html.offset_top();
                    let container_height = container_html.client_height();
                    let container_scroll = container.scroll_top();

                    // Only scroll if element is outside visible area
                    let element_bottom = element_top + 20; // approx line height
                    let visible_top = container_scroll;
                    let visible_bottom = container_scroll + container_height;

                    if element_top < visible_top || element_bottom > visible_bottom {
                        // Center the element in the container
                        let target_scroll = element_top - (container_height / 2);
                        container.set_scroll_top(target_scroll.max(0));
                    }
                }
            }
        });
    }

    // Example selector callback
    let on_example_select = {
        let selected_example = selected_example.clone();
        let examples = props.examples.clone();
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<web_sys::HtmlSelectElement>();
            if let Some(select) = target {
                let idx = select.selected_index();
                // Index 0 is "Choose an example...", so subtract 1 for actual examples
                if idx > 0
                    && let Some(example) = examples.get((idx - 1) as usize)
                {
                    selected_example.set(Some(example.clone()));
                }
            }
        })
    };

    // Open load dialog
    let on_load_dialog_open = {
        let load_dialog_open = load_dialog_open.clone();
        Callback::from(move |_| {
            load_dialog_open.set(true);
        })
    };

    // Close load dialog
    let on_load_dialog_close = {
        let load_dialog_open = load_dialog_open.clone();
        Callback::from(move |_| {
            load_dialog_open.set(false);
        })
    };

    // Load example callback - resets to Source step and closes dialog
    let on_load_click = {
        let on_load = props.on_load.clone();
        let selected = selected_example.clone();
        let current_step = current_step.clone();
        let load_dialog_open = load_dialog_open.clone();
        Callback::from(move |_| {
            if let Some(example) = &*selected {
                on_load.emit(example.clone());
                current_step.set(WizardStep::Source);
                load_dialog_open.set(false);
                // Scroll notebook to top to show source cell
                gloo::timers::callback::Timeout::new(50, || {
                    if let Some(window) = web_sys::window()
                        && let Some(document) = window.document()
                        && let Some(container) = document.get_element_by_id("notebook-scroll")
                    {
                        container.set_scroll_top(0);
                    }
                }).forget();
            }
        })
    };

    // Advance to next wizard step with scroll
    let advance_step = {
        let current_step = current_step.clone();
        Callback::from(move |_| {
            let current = *current_step;
            if let Some(next) = current.next() {
                current_step.set(next);
                // Determine which cell to scroll to:
                // - For Assemble step: scroll to Execution cell (the new cell)
                // - For other steps: scroll to keep previous cell (N-1) at top
                let scroll_to_cell = if next == WizardStep::Assemble {
                    next.cell_id().to_string() // Scroll to Execution
                } else {
                    current.cell_id().to_string() // Scroll to previous cell (N-1)
                };

                gloo::timers::callback::Timeout::new(100, move || {
                    if let Some(window) = web_sys::window()
                        && let Some(document) = window.document()
                        && let Some(container) = document.get_element_by_id("notebook-scroll")
                        && let Some(element) = document.get_element_by_id(&scroll_to_cell)
                    {
                        let element_html: &web_sys::HtmlElement = element.unchecked_ref();
                        container.set_scroll_top(element_html.offset_top());
                    }
                }).forget();
            }
        })
    };

    // Click on a wizard step to jump to it
    let on_step_click = {
        let current_step = current_step.clone();
        move |step: WizardStep| {
            let current_step = current_step.clone();
            Callback::from(move |_| {
                if step <= *current_step {
                    // Scroll to the cell - align at top
                    let cell_id = step.cell_id().to_string();
                    if let Some(window) = web_sys::window()
                        && let Some(document) = window.document()
                        && let Some(container) = document.get_element_by_id("notebook-scroll")
                        && let Some(element) = document.get_element_by_id(&cell_id)
                    {
                        let element_html: &web_sys::HtmlElement = element.unchecked_ref();
                        container.set_scroll_top(element_html.offset_top());
                    }
                }
            })
        }
    };

    // Reset wizard to Source step (always enabled) - also unloads the example
    let on_wizard_reset = {
        let current_step = current_step.clone();
        let on_unload = props.on_unload.clone();
        let selected_example = selected_example.clone();
        Callback::from(move |_| {
            current_step.set(WizardStep::Source);
            selected_example.set(None);
            on_unload.emit(());
        })
    };

    // Execution callbacks
    let on_step_click_exec = {
        let on_step = props.on_step.clone();
        Callback::from(move |_| on_step.emit(()))
    };

    let on_run_click = {
        let on_run = props.on_run.clone();
        Callback::from(move |_| on_run.emit(()))
    };

    let on_stop_click = {
        let on_stop = props.on_stop.clone();
        Callback::from(move |_| on_stop.emit(()))
    };

    let on_reset_click = {
        let on_reset = props.on_reset.clone();
        Callback::from(move |_| on_reset.emit(()))
    };

    let state = &props.cpu_state;

    // Compute LED and switch values outside html! macro for cleaner parsing
    let led_on = (state.led_value & 1) == 1;
    let led_class = if led_on { "led led-on led-large" } else { "led led-off led-large" };
    let led_status = if led_on { "ON" } else { "OFF" };
    let switch_on = (props.switch_value & 1) == 1;
    let switch_class = if switch_on { "switch switch-on switch-large" } else { "switch switch-off switch-large" };
    let switch_status = if switch_on { "PRESSED" } else { "RELEASED" };

    // Switch toggle callback
    let on_switch_click = {
        let on_switch_toggle = props.on_switch_toggle.clone();
        let current_value = props.switch_value;
        Callback::from(move |_: MouseEvent| {
            on_switch_toggle.emit(current_value ^ 1);
        })
    };

    // All wizard steps for rendering
    let all_steps = [
        WizardStep::Source,
        WizardStep::Compile,
        WizardStep::Disasm,
        WizardStep::Translate,
        WizardStep::Assemble,
    ];

    html! {
        <div class="rust-wizard-layout">
            // Column 1: Sidebar with buttons and peripherals
            <div class="wizard-sidebar">
                <div class="wizard-buttons">
                    <button onclick={
                        let cb = props.on_tutorial_open.clone();
                        Callback::from(move |_| cb.emit(()))
                    }>{"Tutorial"}</button>
                    <button onclick={
                        let cb = props.on_examples_open.clone();
                        Callback::from(move |_| cb.emit(()))
                    }>{"Examples"}</button>
                    <button onclick={
                        let cb = props.on_isa_ref_open.clone();
                        Callback::from(move |_| cb.emit(()))
                    }>{"ISA Ref"}</button>
                    <button onclick={
                        let cb = props.on_help_open.clone();
                        Callback::from(move |_| cb.emit(()))
                    }>{"Help"}</button>
                </div>

                // LED D2 and Button S2 (matches COR24-TB hardware)
                <div class="wizard-peripherals">
                    <div class="peripheral-section">
                        <div class="peripheral-label">{"LED D2"}</div>
                        <div class="led-row-vertical">
                            <div class={led_class}>{"D2"}</div>
                        </div>
                        <div class="led-hex">{led_status}</div>
                    </div>
                    <div class="peripheral-section">
                        <div class="peripheral-label">{"Button S2"}</div>
                        <div class="switch-row-vertical">
                            <div class={switch_class} onclick={on_switch_click} title="Click to toggle button S2">
                                {"S2"}
                            </div>
                        </div>
                        <div class="switch-hex">{switch_status}</div>
                    </div>
                </div>
            </div>

            // Column 2: Wizard steps
            <div class="wizard-steps">
                <div class="wizard-header-space"></div>

                {for all_steps.iter().map(|&step| {
                    // Step is completed if we're at or past it (and an example is loaded)
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
                        >
                            <span class="step-indicator">{indicator}</span>
                            <span class="step-label">{step.label()}</span>
                        </div>
                    }
                })}

                // Action button - Load when not loaded, then Compile/Disassemble/etc.
                if !props.is_loaded {
                    <button class="wizard-action-btn" onclick={on_load_dialog_open.clone()} disabled={props.is_running}>
                        {"Load"}
                    </button>
                } else if *current_step != WizardStep::Assemble {
                    <button class="wizard-action-btn" onclick={advance_step.clone()}>
                        {(*current_step).action_label()}
                    </button>
                }

                // Reset button - always enabled, red color
                <button class="wizard-reset-btn" onclick={on_wizard_reset}>
                    {"Reset"}
                </button>
            </div>

            // Column 3: Notebook cells
            <div class="notebook-cells" id="notebook-scroll">
                if let Some(example) = &props.loaded_example {
                    // Cell 1: Rust Source (always visible)
                    <div class="notebook-cell" id="cell-source">
                        <div class="cell-header">{"Rust Source"}</div>
                        <div class="cell-content">
                            <pre class="code-block rust-code">{&example.rust_source}</pre>
                        </div>
                    </div>

                    // Cell 2: WASM Binary (visible after Compile step)
                    if *current_step >= WizardStep::Compile {
                        <div class="notebook-cell" id="cell-wasm">
                            <div class="cell-header">
                                <span>{"WASM Binary"}</span>
                                <span class="cell-badge">{format!("{} bytes", example.wasm_size)}</span>
                            </div>
                            <div class="cell-content">
                                <pre class="code-block hex-dump">{&example.wasm_hex}</pre>
                            </div>
                        </div>
                    }

                    // Cell 3: WASM Disassembly (visible after Disasm step)
                    if *current_step >= WizardStep::Disasm {
                        <div class="notebook-cell" id="cell-disasm">
                            <div class="cell-header">{"WASM Disassembly"}</div>
                            <div class="cell-content">
                                <pre class="code-block wasm-disasm">{&example.wasm_disassembly}</pre>
                            </div>
                        </div>
                    }

                    // Cell 4: COR24 Assembly (visible after Translate step)
                    if *current_step >= WizardStep::Translate {
                        <div class="notebook-cell" id="cell-cor24">
                            <div class="cell-header">{"COR24 Assembly"}</div>
                            <div class="cell-content">
                                <pre class="code-block asm-code">{&example.cor24_assembly}</pre>
                            </div>
                        </div>
                    }

                    // Cell 5: Debug Panel (visible after Assemble step)
                    if *current_step >= WizardStep::Assemble {
                        <div class="notebook-cell notebook-cell-debug" id="cell-debug">
                            <div class="cell-header">{"Execution"}</div>
                            <div class="debug-panel">
                                // Debug controls
                                <div class="debug-controls">
                                    <button class="step-btn" onclick={on_step_click_exec.clone()}
                                        disabled={!props.is_loaded || state.is_halted || props.is_running}>
                                        {"Step"}
                                    </button>
                                    if props.is_running {
                                        <button class="stop-btn" onclick={on_stop_click.clone()}>
                                            {"Stop"}
                                        </button>
                                    } else {
                                        <button class="run-btn" onclick={on_run_click.clone()}
                                            disabled={!props.is_loaded || state.is_halted}>
                                            {"Run"}
                                        </button>
                                    }
                                    <button class="reset-btn" onclick={on_reset_click.clone()}
                                        disabled={!props.is_loaded || props.is_running}>
                                        {"Reset"}
                                    </button>
                                    if state.is_halted {
                                        <span class="status-halted">{"HALTED"}</span>
                                    }
                                </div>

                                // Two-column debug content
                                <div class="debug-content">
                                    // Left: Assembly listing
                                    <div class="debug-left">
                                        <h4>{"Assembly"}</h4>
                                        if props.is_loaded && !state.assembled_lines.is_empty() {
                                            <div class="listing-scroll" id="asm-listing-scroll">
                                                {for state.assembled_lines.iter().map(|line| {
                                                    let is_current = if line.len() > 4 && line.chars().nth(4) == Some(':') {
                                                        if let Ok(addr) = u32::from_str_radix(&line[0..4], 16) {
                                                            addr == state.pc
                                                        } else {
                                                            false
                                                        }
                                                    } else {
                                                        false
                                                    };
                                                    let class = if is_current { "asm-line current-line" } else { "asm-line" };
                                                    html! {
                                                        <div class={class}>{line}</div>
                                                    }
                                                })}
                                            </div>
                                        }
                                    </div>

                                    // Right: Registers and state
                                    <div class="debug-right">
                                        <h4>{"Registers"}</h4>
                                        <div class="register-grid-compact">
                                            {for (0..8).map(|i| {
                                                let name = match i {
                                                    0 => "r0",
                                                    1 => "r1",
                                                    2 => "r2",
                                                    3 => "fp",
                                                    4 => "sp",
                                                    5 => "z",
                                                    6 => "iv",
                                                    7 => "ir",
                                                    _ => "??",
                                                };
                                                let val = state.registers[i];
                                                // Heat map: hot (just changed), warm (changed last step), cold (normal)
                                                let is_hot = val != state.prev_registers[i];
                                                let is_warm = !is_hot && state.prev_registers[i] != state.prev_prev_registers[i];
                                                let row_class = if is_hot {
                                                    "register-row-compact hot"
                                                } else if is_warm {
                                                    "register-row-compact warm"
                                                } else {
                                                    "register-row-compact"
                                                };
                                                html! {
                                                    <div class={row_class}>
                                                        <span class="reg-name">{name}</span>
                                                        <span class="reg-value">{format!("0x{:06X}", val)}</span>
                                                    </div>
                                                }
                                            })}
                                            <div class="register-row-compact">
                                                <span class="reg-name">{"PC"}</span>
                                                <span class="reg-value">{format!("0x{:06X}", state.pc)}</span>
                                            </div>
                                            <div class="register-row-compact">
                                                <span class="reg-name">{"C"}</span>
                                                <span class="reg-value">{if state.condition_flag { "1" } else { "0" }}</span>
                                            </div>
                                        </div>

                                        <div class="cycle-info">
                                            <span>{"Cycles: "}{state.cycle_count}</span>
                                        </div>

                                        // Memory viewer - three regions
                                        if props.is_loaded {
                                            <div class="memory-section">
                                                <h4>{format!("Program (0x000000 \u{2192} 0x{:06X})", state.program_end)}</h4>
                                                <div class="memory-dump-compact">{
                                                    format_memory_dump_heatmap(&state.memory_low, &state.prev_memory_low, &state.prev_prev_memory_low, 0x000000)
                                                }</div>
                                            </div>
                                            <div class="memory-section">
                                                <h4>{format!("Stack (SP = 0x{:06X})", state.stack_base_addr)}</h4>
                                                <div class="memory-dump-compact">{
                                                    format_memory_dump_heatmap(&state.memory_stack, &state.prev_memory_stack, &state.prev_prev_memory_stack, state.stack_base_addr)
                                                }</div>
                                            </div>
                                            <div class="memory-section">
                                                <h4>{"I/O (0xFFFF80 \u{2192})"}</h4>
                                                <div class="memory-dump-compact">{
                                                    format_memory_dump_reversed_heatmap(&state.memory_high, &state.prev_memory_high, &state.prev_prev_memory_high, 0xFFFF80)
                                                }</div>
                                            </div>
                                        }
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    // No example loaded - show placeholder
                    <div class="notebook-placeholder">
                        <p>{"Click "}<strong>{"Load Example"}</strong>{" to get started."}</p>
                    </div>
                }

                // Future: Server-side compilation notice
                <div class="pipeline-note">
                    <em>{"Note: Examples are pre-built. Server-side compilation coming soon."}</em>
                </div>
            </div>

            // Load Example Dialog
            if *load_dialog_open {
                <div class="wizard-dialog-overlay" onclick={on_load_dialog_close.clone()}>
                    <div class="wizard-dialog" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                        <div class="wizard-dialog-header">
                            <h3>{"Load Example"}</h3>
                            <button class="wizard-dialog-close" onclick={on_load_dialog_close.clone()}>{"×"}</button>
                        </div>
                        <div class="wizard-dialog-body">
                            <label>{"Select an example:"}</label>
                            <select class="wizard-dialog-select" onchange={on_example_select} disabled={props.is_running}>
                                <option value="" selected={selected_example.is_none()} disabled={true}>
                                    {"Choose an example..."}
                                </option>
                                {for props.examples.iter().map(|ex| {
                                    let is_selected = match &*selected_example {
                                        Some(sel) => sel.name == ex.name,
                                        None => false,
                                    };
                                    html! {
                                        <option value={ex.name.clone()} selected={is_selected}>
                                            {&ex.name}{" - "}{&ex.description}
                                        </option>
                                    }
                                })}
                            </select>
                        </div>
                        <div class="wizard-dialog-footer">
                            <button class="wizard-dialog-cancel" onclick={on_load_dialog_close}>{"Cancel"}</button>
                            <button class="wizard-dialog-load" onclick={on_load_click} disabled={props.is_running || selected_example.is_none()}>{"Load"}</button>
                        </div>
                    </div>
                </div>
            }
        </div>
    }
}

/// Format memory as hex dump with heat map highlighting (returns Html)
fn format_memory_dump_heatmap(data: &[u8], prev: &[u8], prev_prev: &[u8], base_addr: u32) -> Html {
    html! {
        <>
            {for data.chunks(16).enumerate().map(|(i, chunk)| {
                let addr = base_addr + (i * 16) as u32;
                html! {
                    <div class="memory-row">
                        <span class="memory-addr">{format!("{:06X}: ", addr)}</span>
                        {for chunk.iter().enumerate().map(|(j, byte)| {
                            let idx = i * 16 + j;
                            let prev_byte = prev.get(idx).copied().unwrap_or(0);
                            let prev_prev_byte = prev_prev.get(idx).copied().unwrap_or(0);
                            let is_hot = *byte != prev_byte;
                            let is_warm = !is_hot && prev_byte != prev_prev_byte;
                            let class = if is_hot {
                                "mem-byte hot"
                            } else if is_warm {
                                "mem-byte warm"
                            } else {
                                "mem-byte"
                            };
                            html! {
                                <span class={class}>{format!("{:02X}", byte)}</span>
                            }
                        })}
                    </div>
                }
            })}
        </>
    }
}

/// Format memory as hex dump in reverse order with heat map highlighting (returns Html)
fn format_memory_dump_reversed_heatmap(data: &[u8], prev: &[u8], prev_prev: &[u8], base_addr: u32) -> Html {
    let chunks: Vec<_> = data.chunks(16).collect();
    html! {
        <>
            {for chunks.iter().enumerate().rev().map(|(i, chunk)| {
                let addr = base_addr + (i * 16) as u32;
                html! {
                    <div class="memory-row">
                        <span class="memory-addr">{format!("{:06X}: ", addr)}</span>
                        {for chunk.iter().enumerate().map(|(j, byte)| {
                            let idx = i * 16 + j;
                            let prev_byte = prev.get(idx).copied().unwrap_or(0);
                            let prev_prev_byte = prev_prev.get(idx).copied().unwrap_or(0);
                            let is_hot = *byte != prev_byte;
                            let is_warm = !is_hot && prev_byte != prev_prev_byte;
                            let class = if is_hot {
                                "mem-byte hot"
                            } else if is_warm {
                                "mem-byte warm"
                            } else {
                                "mem-byte"
                            };
                            html! {
                                <span class={class}>{format!("{:02X}", byte)}</span>
                            }
                        })}
                    </div>
                }
            })}
        </>
    }
}
