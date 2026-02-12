use dioxus::prelude::*;
use crate::app_state::{AppState, ResourceTarget};

#[component]
pub fn Toolbar() -> Element {
    let mut state = use_context::<AppState>();
    let run_mode = *state.run_mode.read();
    let mut show_add_dropdown = use_signal(|| false);

    rsx! {
        div {
            class: "toolbar",
            style: "
                height: 36px;
                background: #f0f0f0;
                border-bottom: 1px solid #ccc;
                display: flex;
                align-items: center;
                padding: 0 8px;
                gap: 8px;
            ",
            
            // Run Button
            div {
                style: {
                    let bg = if run_mode { "#e0e0e0" } else { "transparent" };
                    let op = if run_mode { "0.5" } else { "1.0" };
                    let pe = if run_mode { "none" } else { "auto" };
                    format!("padding: 4px; border: 1px solid transparent; border-radius: 3px; cursor: pointer; background: {}; opacity: {}; pointer-events: {};", bg, op, pe)
                },
                title: "Start",
                onclick: move |_| {
                    if !run_mode {
                        state.run_mode.set(true);
                    }
                },
                // Play Icon (Triangle)
                svg {
                    width: "16",
                    height: "16",
                    view_box: "0 0 16 16",
                    path {
                        d: "M4 2 L14 8 L4 14 Z",
                        fill: "#0078d4",
                        stroke: "#005a9e",
                        stroke_width: "1"
                    }
                }
            }
            
            // Stop Button
            div {
                style: {
                    let opacity = if !run_mode { "0.5" } else { "1.0" };
                    let pe = if !run_mode { "none" } else { "auto" };
                    format!("padding: 4px; border: 1px solid transparent; border-radius: 3px; cursor: pointer; opacity: {}; pointer-events: {};", opacity, pe)
                },
                title: "End",
                onclick: move |_| {
                    if run_mode {
                        state.run_mode.set(false);
                    }
                },
                // Stop Icon (Square)
                svg {
                    width: "16",
                    height: "16",
                    view_box: "0 0 16 16",
                    rect {
                        x: "3",
                        y: "3",
                        width: "10",
                        height: "10",
                        fill: "#d13438",
                        stroke: "#a4262c",
                        stroke_width: "1"
                    }
                }
            }
            
            div { style: "width: 1px; height: 20px; background: #ccc; margin: 0 4px;" }
            
            // Toggle Object View Button
            {
                let show_res = *state.show_resources.read();
                let is_designer = !*state.show_code_editor.read() && !show_res;
                let bg = if is_designer { "#d0e8ff" } else { "transparent" };
                let border = if is_designer { "#0078d4" } else { "transparent" };
                rsx! {
                    div {
                        style: format!("padding: 4px; border: 1px solid {border}; background: {bg}; border-radius: 3px; cursor: pointer;"),
                        title: "View Object (Designer)",
                        onclick: move |_| {
                            state.show_code_editor.set(false);
                            state.show_resources.set(false);
                        },
                        svg {
                            width: "16",
                            height: "16",
                            view_box: "0 0 16 16",
                            rect {
                                x: "2",
                                y: "2",
                                width: "12",
                                height: "11",
                                stroke: "black",
                                fill: "none",
                                stroke_width: "1.5"
                            }
                            rect { x: "2", y: "2", width: "12", height: "3", fill: "black" }
                        }
                    }
                }
            }

            // Toggle Code View Button
            {
                let show_res = *state.show_resources.read();
                let is_code = *state.show_code_editor.read() && !show_res;
                let bg = if is_code { "#d0e8ff" } else { "transparent" };
                let border = if is_code { "#0078d4" } else { "transparent" };
                rsx! {
                    div {
                        style: format!("padding: 4px; border: 1px solid {border}; background: {bg}; border-radius: 3px; cursor: pointer;"),
                        title: "View Code",
                        onclick: move |_| {
                            state.show_code_editor.set(true);
                            state.show_resources.set(false);
                        },
                        svg {
                            width: "16",
                            height: "16",
                            view_box: "0 0 16 16",
                            path {
                                d: "M10 2 L8 2 L3 14 L5 14 Z M13.5 6 L16 8 L13.5 10 M2.5 6 L0 8 L2.5 10",
                                stroke: "black",
                                fill: "none",
                                stroke_width: "1.5"
                            }
                        }
                    }
                }
            }
            
            // Toggle Resources Button
             div {
                style: {
                    let is_active = *state.show_resources.read();
                    let bg = if is_active { "#e0e0e0" } else { "transparent" };
                    let border = if is_active { "#bbb" } else { "transparent" };
                    format!("
                        padding: 4px;
                        border: 1px solid {border};
                        background: {bg};
                        border-radius: 3px;
                        cursor: pointer;
                    ")
                },
                title: "View Resources",
                onclick: move |_| {
                    // Ensure default resource file exists
                    {
                        let mut proj_w = state.project.write();
                        if let Some(p) = proj_w.as_mut() {
                            if p.resource_files.is_empty() {
                                p.resource_files.push(irys_project::ResourceManager::new());
                            }
                        }
                    }
                    state.show_resources.set(true);
                    state.show_code_editor.set(false);
                    state.current_resource_target.set(Some(ResourceTarget::Project(0)));
                },
                // Resource Icon (Table-like)
                svg {
                    width: "16",
                    height: "16",
                    view_box: "0 0 16 16",
                    path {
                        d: "M2 3 L14 3 L14 13 L2 13 Z M2 6 L14 6 M6 3 L6 13",
                        stroke: "black",
                        fill: "none",
                        stroke_width: "1.5"
                    }
                }
            }
            
            div { style: "width: 1px; height: 20px; background: #ccc; margin: 0 4px;" }
            
            // Add New dropdown button
            div {
                style: "position: relative;",
                button {
                    style: "padding: 4px 8px; font-size: 12px; cursor: pointer; border: 1px solid #999; background: white; border-radius: 3px; display: flex; align-items: center; gap: 4px;",
                    title: "Add New Item",
                    onclick: move |_| {
                        let current = *show_add_dropdown.read();
                        show_add_dropdown.set(!current);
                    },
                    "➕ Add"
                    span { style: "font-size: 8px;", "▼" }
                }
                if *show_add_dropdown.read() {
                    // Dropdown menu
                    div {
                        style: "position: absolute; top: 100%; left: 0; background: white; border: 1px solid #ccc; box-shadow: 2px 2px 5px rgba(0,0,0,0.2); min-width: 140px; z-index: 1001; margin-top: 2px;",
                        div {
                            style: "padding: 6px 12px; cursor: pointer;",
                            onmouseenter: move |_| {},
                            onclick: move |_| {
                                state.add_new_form();
                                show_add_dropdown.set(false);
                            },
                            "📋 Form"
                        }
                        div {
                            style: "padding: 6px 12px; cursor: pointer;",
                            onclick: move |_| {
                                state.add_code_file();
                                show_add_dropdown.set(false);
                            },
                            "\u{1F4C4} Code"
                        }
                        div {
                            style: "padding: 6px 12px; cursor: pointer;",
                            onclick: move |_| {
                                // Add a new resource file to the project
                                let new_idx;
                                {
                                    let mut proj_w = state.project.write();
                                    if let Some(p) = proj_w.as_mut() {
                                        if p.resource_files.is_empty() {
                                            p.resource_files.push(irys_project::ResourceManager::new());
                                        }
                                        new_idx = 0;
                                    } else {
                                        new_idx = 0;
                                    }
                                }
                                state.show_resources.set(true);
                                state.show_code_editor.set(false);
                                state.current_resource_target.set(Some(ResourceTarget::Project(new_idx)));
                                show_add_dropdown.set(false);
                            },
                            "⚙️ Resource"
                        }
                    }
                    // Overlay to close dropdown
                    div {
                        style: "position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; z-index: 1000;",
                        onclick: move |_| show_add_dropdown.set(false)
                    }
                }
            }
        }
    }
}
