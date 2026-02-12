use dioxus::prelude::*;
use crate::app_state::{AppState, ResourceTarget};

#[component]
pub fn ProjectExplorer() -> Element {
    let mut state = use_context::<AppState>();
    let project = state.project.read();
    let mut current_form = state.current_form;
    // Context menu state: (x, y, item_name)
    let mut context_menu = use_signal(|| None::<(f64, f64, String)>);
    // Confirm removal dialog
    let mut confirm_remove = use_signal(|| None::<String>);
    
    rsx! {
        div {
            class: "project-explorer",
            style: "width: 200px; background: #fafafa; border-right: 1px solid #ccc; padding: 8px;",
            
            h3 { style: "margin: 0 0 8px 0; font-size: 14px;", "Project Explorer" }
            
            div {
                style: "border-top: 1px solid #ccc; padding-top: 8px;",
                
                {
                    if let Some(proj) = project.as_ref() {
                        let current_target = state.current_resource_target.read().clone();
                        rsx! {
                            div {
                                style: "font-weight: bold; margin-bottom: 8px;",
                                "📁 {proj.name}"
                            }
                            
                            div {
                                style: "margin-left: 16px;",
                                
                                // Forms section
                                div {
                                    style: "font-weight: bold; margin-bottom: 4px;",
                                    "📋 Forms"
                                }
                                
                                for form_module in &proj.forms {
                                    {
                                        let form_name = form_module.form.name.clone();
                                        let form_name_ctx = form_module.form.name.clone();
                                        let form_name_res = form_module.form.name.clone();
                                        let is_showing_res = *state.show_resources.read();
                                        let is_form_selected = *current_form.read() == Some(form_name.clone()) && !is_showing_res;
                                        let bg_color = if is_form_selected { "#e3f2fd" } else { "transparent" };
                                        let has_form_res = !form_module.resources.resources.is_empty();
                                        let form_res_count = form_module.resources.resources.len();
                                        let is_form_res_selected = is_showing_res && current_target == Some(ResourceTarget::Form(form_name_res.clone()));
                                        let form_res_bg = if is_form_res_selected { "#e3f2fd" } else { "transparent" };
                                        
                                        rsx! {
                                            div {
                                                key: "{form_name}",
                                                style: "padding: 4px 8px; cursor: pointer; background: {bg_color}; border-radius: 3px; margin-bottom: 2px;",
                                                onclick: move |_| {
                                                    current_form.set(Some(form_name.clone()));
                                                    state.show_code_editor.set(false);
                                                    state.show_resources.set(false);
                                                },
                                                oncontextmenu: move |e: Event<MouseData>| {
                                                    e.prevent_default();
                                                    let coords = e.page_coordinates();
                                                    context_menu.set(Some((coords.x, coords.y, form_name_ctx.clone())));
                                                },
                                                "  {form_name}"
                                            }
                                            // Form-level resources (shown indented under form if they exist)
                                            if has_form_res {
                                                div {
                                                    style: "padding: 2px 8px 2px 24px; cursor: pointer; background: {form_res_bg}; border-radius: 3px; margin-bottom: 2px; font-size: 12px; color: #666;",
                                                    onclick: move |_| {
                                                        state.show_resources.set(true);
                                                        state.show_code_editor.set(false);
                                                        state.current_resource_target.set(Some(ResourceTarget::Form(form_name_res.clone())));
                                                    },
                                                    "📄 {form_name_res}.resx ({form_res_count})"
                                                }
                                            }
                                        }
                                    }
                                }
                                
                                // Code files section
                                if !proj.code_files.is_empty() {
                                    div {
                                        style: "font-weight: bold; margin-top: 12px; margin-bottom: 4px;",
                                        "\u{1F4C4} Code"
                                    }
                                    for code_file in &proj.code_files {
                                        {
                                            let cf_name = code_file.name.clone();
                                            let cf_name_ctx = code_file.name.clone();
                                            let is_selected = *current_form.read() == Some(cf_name.clone()) && !*state.show_resources.read();
                                            let bg_color = if is_selected { "#e3f2fd" } else { "transparent" };

                                            rsx! {
                                                div {
                                                    key: "{cf_name}",
                                                    style: "padding: 4px 8px; cursor: pointer; background: {bg_color}; border-radius: 3px; margin-bottom: 2px;",
                                                    onclick: move |_| {
                                                        current_form.set(Some(cf_name.clone()));
                                                        state.show_code_editor.set(true);
                                                        state.show_resources.set(false);
                                                    },
                                                    oncontextmenu: move |e: Event<MouseData>| {
                                                        e.prevent_default();
                                                        let coords = e.page_coordinates();
                                                        context_menu.set(Some((coords.x, coords.y, cf_name_ctx.clone())));
                                                    },
                                                    "  {cf_name}"
                                                }
                                            }
                                        }
                                    }
                                }

                                // Project Resources section
                                div {
                                    style: "font-weight: bold; margin-top: 12px; margin-bottom: 4px;",
                                    "⚙️ Resources"
                                }
                                
                                // Show each project-level resource file
                                if proj.resource_files.is_empty() {
                                    // No resource files yet — show default "Resources" entry
                                    {
                                        let is_res_selected = *state.show_resources.read() && current_target == Some(ResourceTarget::Project(0));
                                        let bg_color = if is_res_selected { "#e3f2fd" } else { "transparent" };
                                        rsx! {
                                            div {
                                                style: "padding: 4px 8px; cursor: pointer; background: {bg_color}; border-radius: 3px; margin-bottom: 2px;",
                                                onclick: move |_| {
                                                    // Create the default Resources.resx on first click
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
                                                "  Resources.resx"
                                            }
                                        }
                                    }
                                } else {
                                    for (idx, res_file) in proj.resource_files.iter().enumerate() {
                                        {
                                            let res_name = res_file.name.clone();
                                            let res_count = res_file.resources.len();
                                            let is_res_selected = *state.show_resources.read() && current_target == Some(ResourceTarget::Project(idx));
                                            let bg_color = if is_res_selected { "#e3f2fd" } else { "transparent" };
                                            let label = if res_count > 0 {
                                                format!("  {res_name}.resx ({res_count})")
                                            } else {
                                                format!("  {res_name}.resx")
                                            };
                                            rsx! {
                                                div {
                                                    key: "res_{idx}",
                                                    style: "padding: 4px 8px; cursor: pointer; background: {bg_color}; border-radius: 3px; margin-bottom: 2px;",
                                                    onclick: move |_| {
                                                        state.show_resources.set(true);
                                                        state.show_code_editor.set(false);
                                                        state.current_resource_target.set(Some(ResourceTarget::Project(idx)));
                                                    },
                                                    "{label}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        rsx! {
                            div {
                                style: "color: #999; font-style: italic;",
                                "No project loaded"
                            }
                        }
                    }
                }
            }

            // Right-click context menu
            if let Some((x, y, item_name)) = context_menu.read().clone() {
                // Overlay to close
                div {
                    style: "position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; z-index: 2000;",
                    onclick: move |_| context_menu.set(None),
                    oncontextmenu: move |e: Event<MouseData>| {
                        e.prevent_default();
                        context_menu.set(None);
                    },
                }
                // Menu
                div {
                    style: "position: fixed; left: {x}px; top: {y}px; background: white; border: 1px solid #ccc; box-shadow: 2px 2px 6px rgba(0,0,0,0.2); min-width: 140px; z-index: 2001; border-radius: 3px;",
                    div {
                        style: "padding: 6px 14px; cursor: pointer; font-size: 13px;",
                        onmouseenter: move |_| {},
                        onclick: move |_| {
                            confirm_remove.set(Some(item_name.clone()));
                            context_menu.set(None);
                        },
                        "🗑 Remove \"{item_name}\""
                    }
                }
            }

            // Confirmation dialog
            if let Some(item_name) = confirm_remove.read().clone() {
                div {
                    style: "position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; background: rgba(0,0,0,0.3); z-index: 3000; display: flex; align-items: center; justify-content: center;",
                    div {
                        style: "background: white; border: 1px solid #ccc; border-radius: 6px; padding: 20px 28px; box-shadow: 4px 4px 12px rgba(0,0,0,0.3); min-width: 300px;",
                        div {
                            style: "font-size: 14px; font-weight: bold; margin-bottom: 12px;",
                            "Remove Item"
                        }
                        div {
                            style: "margin-bottom: 16px; font-size: 13px;",
                            "Are you sure you want to remove \"{item_name}\" from the project?"
                        }
                        div {
                            style: "display: flex; justify-content: flex-end; gap: 8px;",
                            button {
                                style: "padding: 4px 16px; border: 1px solid #ccc; background: #f0f0f0; border-radius: 3px; cursor: pointer;",
                                onclick: move |_| confirm_remove.set(None),
                                "Cancel"
                            }
                            button {
                                style: "padding: 4px 16px; border: 1px solid #c00; background: #e74c3c; color: white; border-radius: 3px; cursor: pointer;",
                                onclick: move |_| {
                                    state.remove_project_item(&item_name);
                                    confirm_remove.set(None);
                                },
                                "Remove"
                            }
                        }
                    }
                }
            }
        }
    }
}
