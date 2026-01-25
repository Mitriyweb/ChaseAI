#[cfg(target_os = "macos")]
#[allow(unexpected_cfgs)]
#[allow(clippy::all)]
fn main() -> anyhow::Result<()> {
    use tray_icon::menu::MenuEvent;
    use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicy};
    use cocoa::base::{id, nil};
    use cocoa::foundation::NSString;
    use objc::declare::ClassDecl;
    use objc::runtime::{Object, Sel};
    use objc::{class, msg_send, sel, sel_impl};
    use std::sync::Once;

    println!("Starting ChaseAI...");

    // Initialize NSApplication
    unsafe {
        let app = NSApp();
        app.setActivationPolicy_(
            NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory,
        );
        println!("macOS activation policy set to menu bar only (Accessory)");
    }

    // Create and initialize app
    let mut app_instance = app::App::new();

    // Run app initialization (this sets up tray)
    app_instance.run()?;

    // Store app instance as static (not ideal but works for menu bar app)
    static mut APP_INSTANCE: Option<app::App> = None;
    unsafe {
        APP_INSTANCE = Some(app_instance);
    }

    println!("Setting up menu event handler...");

    // Create a timer to poll for menu events
    unsafe {
        static REGISTER_CLASS: Once = Once::new();
        REGISTER_CLASS.call_once(|| {
            let superclass = class!(NSObject);
            let mut decl = ClassDecl::new("MenuEventHandler", superclass).unwrap();

            extern "C" fn handle_timer(_: &Object, _: Sel, _: id) {
                while let Ok(menu_event) = MenuEvent::receiver().try_recv() {
                    println!("Menu event received: {:?}", menu_event.id);
                    unsafe {
                        if let Some(ref mut app) = APP_INSTANCE {
                            app.handle_menu_event(menu_event);
                        }
                    }
                }
            }

            decl.add_method(
                sel!(handleTimer:),
                handle_timer as extern "C" fn(&Object, Sel, id),
            );

            decl.register();
        });

        let handler_class = class!(MenuEventHandler);
        let handler: id = msg_send![handler_class, new];

        // Create a timer that fires every 50ms to check for menu events
        let timer: id = msg_send![class!(NSTimer),
            scheduledTimerWithTimeInterval: 0.05
            target: handler
            selector: sel!(handleTimer:)
            userInfo: nil
            repeats: true
        ];

        let run_loop: id = msg_send![class!(NSRunLoop), currentRunLoop];
        let mode = NSString::alloc(nil).init_str("kCFRunLoopDefaultMode");
        let _: () = msg_send![run_loop, addTimer:timer forMode:mode];
    }

    println!("Entering main loop...");
    println!("Application is running. Tray icon should be visible and clickable in menu bar.");

    // Run the NSApplication event loop
    unsafe {
        let app: id = NSApp();
        let _: () = msg_send![app, run];
    }

    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn main() -> anyhow::Result<()> {
    eprintln!("This application is designed for macOS only");
    std::process::exit(1);
}
