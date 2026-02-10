use dioxus::prelude::*;
use crate::app_state::AppState;
use irys_ui::runtime_panel::RuntimeProject;

/// Editor wrapper – provides the project from AppState as a RuntimeProject
/// context, then delegates entirely to the shared FormRunner in irys_ui.
#[component]
pub fn RuntimePanel() -> Element {
    let state = use_context::<AppState>();

    // Bridge: expose the editor's project signal via the shared RuntimeProject
    // context so FormRunner can read it.
    use_context_provider(|| RuntimeProject {
        project: state.project,
    });

    rsx! {
        irys_ui::FormRunner {}
    }
}
