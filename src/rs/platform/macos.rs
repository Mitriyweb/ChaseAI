use crate::App;
use cocoa::appkit::{NSApplication, NSApplicationActivationPolicy};
use tao::event_loop::{ControlFlow, EventLoop};
use tray_icon::menu::MenuEvent;

pub fn run() -> anyhow::Result<()> {
    println!("Starting ChaseAI on macOS...");

    // Enforce activation policy BEFORE EventLoop
    // SAFETY: Accessing global NSApp and setting activation policy is safe during initial startup
    unsafe {
        let app = cocoa::appkit::NSApplication::sharedApplication(cocoa::base::nil);
        app.setActivationPolicy_(
            NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory,
        );
    }

    // Create tao event loop first (this initializes NSApp)
    let event_loop = EventLoop::new();

    // Enforce activation policy AFTER EventLoop and force update
    // SAFETY: NSApp is guaranteed to be initialized by EventLoop::new()
    unsafe {
        let app = cocoa::appkit::NSApplication::sharedApplication(cocoa::base::nil);
        app.setActivationPolicy_(
            NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory,
        );
        app.activateIgnoringOtherApps_(cocoa::base::YES);
    }

    // Create and initialize app
    let mut app_instance = App::new();

    // Run app initialization (this sets up tray)
    app_instance.run()?;

    static mut APP_INSTANCE: Option<App> = None;
    // SAFETY: Single-threaded initialization before event loop starts handling events
    unsafe {
        APP_INSTANCE = Some(app_instance);
    }

    println!("Setting up event loop...");
    println!("Entering main loop...");
    println!("Application is running. Tray icon should be visible and clickable in menu bar.");

    let mut initial_setup_done = false;

    // Run the event loop
    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if !initial_setup_done {
            // SAFETY: System is fully initialized, safe to apply final activation policy
            unsafe {
                let app = cocoa::appkit::NSApplication::sharedApplication(cocoa::base::nil);
                app.setActivationPolicy_(
                    NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory,
                );
                app.activateIgnoringOtherApps_(cocoa::base::YES);
            }
            initial_setup_done = true;
            println!("Event loop: Activation policy (Accessory) applied.");
        }

        // Check for menu events
        while let Ok(menu_event) = MenuEvent::receiver().try_recv() {
            println!("Menu event received: {:?}", menu_event.id);
            // SAFETY: APP_INSTANCE is initialized before the event loop starts
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
