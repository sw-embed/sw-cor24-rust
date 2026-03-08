use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MemoryViewerProps {
    pub memory: Vec<u8>,
    pub pc: u32,
    /// Base address of the memory region (for display)
    #[prop_or(0)]
    pub base_address: u32,
    #[prop_or(128)]
    pub bytes_to_show: usize,
    #[prop_or(16)]
    pub bytes_per_row: usize,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub changed_addresses: Vec<usize>,
    /// If true, display as 16-bit words instead of bytes (for word-addressable machines)
    #[prop_or(false)]
    pub word_mode: bool,
}

#[derive(Properties, PartialEq)]
pub struct WordMemoryViewerProps {
    pub memory: Vec<u16>,
    pub pc: u16,
    #[prop_or(64)]
    pub words_to_show: usize,
    #[prop_or(8)]
    pub words_per_row: usize,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub changed_addresses: Vec<usize>,
}

#[function_component(MemoryViewer)]
pub fn memory_viewer(props: &MemoryViewerProps) -> Html {
    let title = props
        .title
        .clone()
        .unwrap_or_else(|| format!("Memory (First {} Bytes)", props.bytes_to_show));

    let base = props.base_address;
    let rows = (0..props.bytes_to_show)
        .step_by(props.bytes_per_row)
        .map(|offset| {
            let display_addr = base + offset as u32;
            html! {
                <div class="memory-row" key={offset}>
                    <span class="memory-address">
                        {format!("{:06X}:", display_addr)}
                    </span>
                    { for (0..props.bytes_per_row).map(|i| {
                        let byte_offset = offset + i;
                        if byte_offset < props.memory.len() {
                            let byte = props.memory[byte_offset];
                            let abs_addr = base + byte_offset as u32;
                            let is_pc = abs_addr == props.pc;
                            let is_changed = props.changed_addresses.contains(&byte_offset);

                            let class = if is_pc {
                                "memory-byte pc-highlight"
                            } else if is_changed {
                                "memory-byte changed"
                            } else {
                                "memory-byte"
                            };

                            html! {
                                <span {class} key={byte_offset}>
                                    {format!("{:02X}", byte)}
                                </span>
                            }
                        } else {
                            html! {
                                <span class="memory-byte" key={byte_offset}>
                                    {"  "}
                                </span>
                            }
                        }
                    })}
                </div>
            }
        });

    html! {
        <div class="memory-panel">
            <div class="panel-title">{title}</div>
            <div class="memory-viewer">
                { for rows }
            </div>
        </div>
    }
}

#[function_component(WordMemoryViewer)]
pub fn word_memory_viewer(props: &WordMemoryViewerProps) -> Html {
    let title = props
        .title
        .clone()
        .unwrap_or_else(|| format!("Memory (First {} Words)", props.words_to_show));

    let rows = (0..props.words_to_show)
        .step_by(props.words_per_row)
        .map(|word_addr| {
            html! {
                <div class="memory-row" key={word_addr}>
                    <span class="memory-address">
                        {format!("{:04X}:", word_addr)}
                    </span>
                    { for (0..props.words_per_row).map(|i| {
                        let addr = word_addr + i;
                        if addr < props.memory.len() {
                            let word = props.memory[addr];
                            let is_pc = addr == props.pc as usize;
                            let is_changed = props.changed_addresses.contains(&addr);

                            // Check for IBM 1130 special memory locations
                            let is_trap = addr == 0;  // Safety trap location
                            let is_index_reg = (1..=3).contains(&addr);  // XR1, XR2, XR3
                            let is_interrupt = (8..=13).contains(&addr);  // Interrupt vectors

                            let class = if is_pc {
                                "memory-word pc-highlight"
                            } else if is_changed {
                                "memory-word changed"
                            } else if is_trap {
                                "memory-word trap"
                            } else if is_index_reg {
                                "memory-word index-reg"
                            } else if is_interrupt {
                                "memory-word interrupt"
                            } else {
                                "memory-word"
                            };

                            // Create tooltip for special locations
                            let tooltip = if is_trap {
                                "Location 0: Safety trap (infinite loop)"
                            } else if addr == 1 {
                                "Location 1: XR1 (Index Register 1)"
                            } else if addr == 2 {
                                "Location 2: XR2 (Index Register 2)"
                            } else if addr == 3 {
                                "Location 3: XR3 (Index Register 3)"
                            } else if is_interrupt {
                                "Interrupt vector"
                            } else {
                                ""
                            };

                            html! {
                                <span {class} key={addr} title={tooltip}>
                                    {format!("{:04X}", word)}
                                </span>
                            }
                        } else {
                            html! {
                                <span class="memory-word" key={addr}>
                                    {"    "}
                                </span>
                            }
                        }
                    })}
                </div>
            }
        });

    html! {
        <div class="memory-panel">
            <div class="panel-title">{title}</div>
            <div class="memory-viewer">
                { for rows }
            </div>
        </div>
    }
}
