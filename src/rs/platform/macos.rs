use crate::App;
use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicy};
use tao::event_loop::{ControlFlow, EventLoop};
use tray_icon::menu::MenuEvent;

pub fn run() -> anyhow::Result<()> {
    println!("Starting ChaseAI on macOS...");

    // Initialize NSApplication
    // SAFETY: Initializing NSApplication and setting activation policy is required for proper macOS UI integration
    // and is safe to call at application startup on the main thread.
    unsafe {
        let app = NSApp();
        app.setActivationPolicy_(
            NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory,
        );
        println!("macOS activation policy set to menu bar only (Accessory)");
    }

    // Create and initialize app
    let mut app_instance = App::new()?;

    // Run app initialization (this sets up tray)
    app_instance.run()?;

    // Store app instance as static (not ideal but works for menu bar app)
    static mut APP_INSTANCE: Option<App> = None;
    // SAFETY: We are initializing a static mut exactly once during the application startup
    // before the event loop starts. Access is subsequently controlled within the main event loop.
    unsafe {
        APP_INSTANCE = Some(app_instance);
    }

    println!("Setting up event loop...");

    // Create tao event loop for proper macOS integration
    let event_loop = EventLoop::new();

    println!("Entering main loop...");
    println!("Application is running. Tray icon should be visible and clickable in menu bar.");

    // Run the event loop
    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Check for menu events
        while let Ok(menu_event) = MenuEvent::receiver().try_recv() {
            println!("Menu event received: {:?}", menu_event.id);
            // SAFETY: Accessing APP_INSTANCE within the single-threaded event loop on macOS
            // is safe as no other thread will be mutating this static after initialization.
            unsafe {
                if let Some(ref mut app) = APP_INSTANCE {
                    app.handle_menu_event(menu_event);
                }
            }
        }
    });
}
