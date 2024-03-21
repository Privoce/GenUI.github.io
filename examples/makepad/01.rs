use makepad_widgets::*; // Ensure makepad_widgets are imported

live_design!{
    import makepad_widgets::base::*;                  // Import base widgets and components
    import makepad_widgets::theme_desktop_dark::*;    // Use the dark theme for desktop applications
    App = {{App}} {                                   // Define the main App struct
        ui: <Window>{}                                // Use a Window widget as the root UI element
    }
}

#[derive(Live, LiveHook)] // Derive macros to enable live reloading and hooks
pub struct App {
    #[live] ui: WidgetRef, // The root UI element reference
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx); // Register the live design for makepad_widgets
    }
}

app_main!(App); // Macro to designate App as the main application entry