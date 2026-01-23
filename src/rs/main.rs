use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tray_icon::menu::MenuEvent;

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoopBuilder::new().build();
    let mut app = app::App::new();
    app.run()?;

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Ok(event) = MenuEvent::receiver().try_recv() {
            println!("Menu event: {:?}", event);
        }
    });
}
