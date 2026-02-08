use crate::App;
use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicy};
use tao::event_loop::{ControlFlow, EventLoop};
use tray_icon::menu::MenuEvent;

pub fn run() -> anyhow::Result<()> {
    println!("Starting ChaseAI on macOS...");

    // Enforce activation policy BEFORE EventLoop (in case it respects current state)
    unsafe {
        let app = NSApp();
        app.setActivationPolicy_(
            NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory,
        );
    }

    // Create tao event loop first (this initializes NSApp)
    let event_loop = EventLoop::new();

    // Enforce activation policy AFTER EventLoop (override if it reset it)
    unsafe {
        let app = NSApp();
        app.setActivationPolicy_(
            NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory,
        );
    }

    // Create and initialize app
    let mut app_instance = App::new()?;

    // Run app initialization (this sets up tray)
    app_instance.run()?;

    // Store app instance as static (not ideal but works for menu bar app)
    static mut APP_INSTANCE: Option<App> = None;
    unsafe {
        APP_INSTANCE = Some(app_instance);
    }

    println!("Setting up event loop...");
    println!("Entering main loop...");
    println!("Application is running. Tray icon should be visible and clickable in menu bar.");

    // Run the event loop
    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Check for menu events
        while let Ok(menu_event) = MenuEvent::receiver().try_recv() {
            println!("Menu event received: {:?}", menu_event.id);
            unsafe {
                if let Some(ref mut app) = APP_INSTANCE {
                    app.handle_menu_event(menu_event);
                }
            }
        }
    });

    // Note: run() theoretically diverts control, but for the Result signature:
    #[allow(unreachable_code)]
    Ok(())
}
